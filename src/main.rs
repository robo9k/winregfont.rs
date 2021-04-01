use winregfont::font_resource::add_from_cstring as add_font_resource;

fn main() {
    let filename = std::env::args_os()
        .nth(1)
        .expect("Expected one command line argument for font filename");
    let filename =
        widestring::U16CString::from_os_str(filename).expect("Filename arg contains NUL values");

    let num_fonts_added = add_font_resource(filename).expect("Success adding font resource");
    println!("AddFontResourceW: {}", num_fonts_added);
}
