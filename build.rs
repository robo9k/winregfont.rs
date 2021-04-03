fn write_env_var<W: std::io::Write>(
    write: &mut W,
    key: &str,
    value: &str,
) -> Result<(), std::io::Error> {
    if value.is_empty() {
        writeln!(write, "// {} is empty", key).expect("can write comment");
    } else {
        writeln!(write, "#define {} {}", key, value).expect("can write #define");
    }

    Ok(())
}

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").expect("env var 'OUT_DIR' is set");
    let dest_path = std::path::Path::new(&out_dir).join("crate-pkg.h");
    let mut header_file = std::fs::File::create(dest_path).expect("can create file 'crate-pkg.h'");

    let debug = std::env::var("DEBUG").expect("env var 'DEBUG' is present");
    let debug: bool = debug.parse().expect("env var 'DEBUG' has boolean value");
    let debug = if debug { "1" } else { "" };
    write_env_var(&mut header_file, "DEBUG", debug).expect("can write env var to header file");

    let profile = std::env::var("PROFILE").expect("env var 'PROFILE' is present");
    write_env_var(&mut header_file, "PROFILE", &profile).expect("can write env var to header file");

    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_PKG_") {
            write_env_var(&mut header_file, &key, &value)
                .expect("can write env var to header file");
        }
    }

    embed_resource::compile("winregfont.rc");
}
