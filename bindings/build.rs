fn main() {
    windows::build!(
        Windows::Win32::Gdi::AddFontResourceW,
        Windows::Win32::WindowsAndMessaging::{PostMessageW, WM_FONTCHANGE},
    );
}
