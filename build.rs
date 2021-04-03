use std::io::Write;

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").expect("env var 'OUT_DIR' is set");
    let dest_path = std::path::Path::new(&out_dir).join("crate-pkg.h");
    let mut header_file = std::fs::File::create(dest_path).expect("can create file 'crate-pkg.h'");

    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_PKG_") {
            if value.is_empty() {
                writeln!(header_file, "// {} is undefined", key)
                    .expect("can write comment to header file");
            } else {
                writeln!(header_file, "#define {} {}", key, value)
                    .expect("can write #define directive to header file");
            }
        }
    }

    embed_resource::compile("winregfont.rc");
}
