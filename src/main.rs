use bindings::Windows::Win32::{Gdi::AddFontResourceW, SystemServices::PWSTR};
use std::iter::once;
use std::os::windows::prelude::*;

fn main() {
    let filename = std::env::args_os()
        .nth(1)
        .expect("Expected one command line argument for font filename");
    let mut filename: Vec<u16> = filename.encode_wide().chain(once(0)).collect();

    let rv = unsafe {
        let filename = filename.as_mut_ptr();
        AddFontResourceW(PWSTR(filename))
    };

    println!("AddFontResourceW: {}", rv);
}
