pub mod font_resource {
    use bindings::Windows::Win32::WindowsAndMessaging::*;
    use bindings::Windows::Win32::{
        Gdi::{AddFontResourceW, RemoveFontResourceW},
        SystemServices::PWSTR,
    };
    use std::convert::TryFrom;

    pub fn add_from_cstring<F: Into<widestring::U16CString>>(filename: F) -> Result<u16, ()> {
        let mut filename = filename.into().into_vec_with_nul();
        let filename = PWSTR(filename.as_mut_ptr());

        match unsafe { AddFontResourceW(filename) } {
            0 => Err(()),
            rv => Ok(u16::try_from(rv).expect("API contract violation")),
        }
    }

    pub fn remove_from_cstring<F: Into<widestring::U16CString>>(filename: F) -> bool {
        let mut filename = filename.into().into_vec_with_nul();
        let filename = PWSTR(filename.as_mut_ptr());

        let rv = unsafe { RemoveFontResourceW(filename) };
        rv.into()
    }

    pub fn broadcast_fontchange() -> bool {
        let rv = unsafe { PostMessageW(HWND(0xffff), WM_FONTCHANGE, WPARAM(0), LPARAM(0)) };
        rv.into()
    }
}
