use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use winregfont::font_resource::{
    add_from_cstring as add_font_resource, broadcast_fontchange,
    remove_from_cstring as remove_font_resource,
};

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
enum ResourceAction {
    /// Add font
    #[clap(alias = "a")]
    Add,
    /// Remove font
    #[clap(alias = "r")]
    Remove,
}

/// CLI to add or remove files as temporary system fonts
///
/// Installing font files as permanent system fonts usually requires administrative permissions.
/// Using this CLI you can install temporary system fonts as a regular user for the current session.
#[derive(clap::Clap)]
#[clap(author, version,
    setting = clap::AppSettings::WaitOnError, setting = clap::AppSettings::ColoredHelp,
    before_help = "App icon \"Increase Font Size\" by \"Hello Many\" from thenounproject.com",
    before_long_help = "Application .exe icon is \"Increase Font Size\" by \"Hello Many\" from thenounproject.com",
    after_long_help = r#"Note that due to CLI syntax you can also just drag & drop font files on the .exe

The WM_FONTCHANGE broadcast allows other applications to reload the font list. If applictions do not support this message, they have to be restarted to detect those font changes.

Note that the underlying Windows API supports Type1 fonts which require a special "abcxxxxx.pfm | abcxxxxx.pfb" filename syntax.
This syntax and .mmm files are not supported by this CLI.
"#,
)]
struct FontResourceOpts {
    /// Action
    ///
    /// Action to apply to the font resources
    #[clap(arg_enum, short, long, default_value = "add")]
    action: ResourceAction,

    /// Don't broadcast font change
    ///
    /// Do not send a WM_FONTCHANGE message as a broadcast to all top-level windows
    #[clap(long = "no-fontchange-broadcast", parse(from_flag = std::ops::Not::not))]
    broadcast_fontchange: bool,

    /// Font filename
    ///
    /// Valid filenames have an extension of .fon, .fnt, .ttf, .ttc, .fot, or .otf
    #[clap(name = "FILE", parse(try_from_os_str = try_parse_font_filename))]
    filenames: Vec<PathBuf>,
}

fn main() {
    use clap::Clap;

    let opts = FontResourceOpts::parse();

    for filename in opts.filenames {
        let filename_cstr =
            widestring::U16CString::from_os_str(filename.clone()).expect("Contains no NUL value");

        match opts.action {
            ResourceAction::Add => match add_font_resource(filename_cstr) {
                Ok(num_fonts) => println!(
                    "Successfully added {} font for file {}",
                    num_fonts,
                    filename.display()
                ),
                Err(_) => eprintln!("Could not add font for file {}", filename.display()),
            },
            ResourceAction::Remove => match remove_font_resource(filename_cstr) {
                true => println!("Successfully removed font for file {}", filename.display()),
                false => eprintln!("Could not remove font for file {}", filename.display()),
            },
        }
    }

    if opts.broadcast_fontchange {
        match broadcast_fontchange() {
            true => println!("Successfully broadcasted font change message"),
            false => eprintln!("Could not broadcast font change message"),
        }
    }
}
