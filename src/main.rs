use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use winregfont::font_resource::{add_from_cstring as add_font_resource, broadcast_fontchange};

const FONT_FILENAME_EXTENSIONS: [&'static str; 6] = ["fon", "fnt", "ttf", "ttc", "fot", "otf"];

fn try_parse_font_filename(filename: &OsStr) -> Result<PathBuf, String> {
    let path = Path::new(filename);

    match path.extension() {
        None => return Err("Missing extension".to_string()),
        Some(ext) => {
            if !FONT_FILENAME_EXTENSIONS
                .iter()
                .any(|valid_ext| ext == *valid_ext)
            {
                return Err("Invalid extension".to_string());
            }
        }
    }

    if !path.is_file() {
        return Err("Not a file".to_string());
    }

    Ok(path.to_path_buf())
}

#[derive(clap::Clap)]
#[clap(author, about, version, setting = clap::AppSettings::WaitOnError, setting = clap::AppSettings::ColoredHelp)]
struct AddFontResourceOpts {
    /// Don't broadcast font change
    ///
    /// Do not send a WM_FONTCHANGE message as a broadcast to all top-level windows
    #[clap(long = "no-fontchange-broadcast", parse(from_flag = std::ops::Not::not))]
    broadcast_fontchange: bool,

    /// Font filename
    ///
    /// Valid filenames have an extension of .fon, .fnt, .ttf, .ttc, .fot, or .otf
    #[clap(name = "FILE", required = true, min_values = 1, parse(try_from_os_str = try_parse_font_filename))]
    filenames: Vec<PathBuf>,
}

fn main() {
    use clap::Clap;

    let opts = AddFontResourceOpts::parse();

    for filename in opts.filenames {
        let filename_cstr =
            widestring::U16CString::from_os_str(filename.clone()).expect("Contains no NUL value");

        match add_font_resource(filename_cstr) {
            Ok(num_fonts) => println!(
                "Successfully added {} font for file {}",
                num_fonts,
                filename.display()
            ),
            Err(_) => eprintln!("Could not add font for file {}", filename.display()),
        }
    }

    if opts.broadcast_fontchange {
        match broadcast_fontchange() {
            true => println!("Successfully broadcasted font change message"),
            false => eprintln!("Could not broadcast font change message"),
        }
    }
}
