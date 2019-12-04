mod native;

use std::ffi::CStr;
use std::ffi::CString;

pub fn version() -> String {
    unsafe {
        CStr::from_ptr(native::solidity_version())
            .to_string_lossy()
            .into_owned()
    }
}

pub fn license() -> String {
    unsafe {
        CStr::from_ptr(native::solidity_license())
            .to_string_lossy()
            .into_owned()
    }
}

// FIXME support read callback
pub fn compile(input: &str) -> String {
    let input_cstr = CString::new(input).expect("CString failed (input contains a 0 byte?)");
    unsafe {
        CStr::from_ptr(native::solidity_compile(
            input_cstr.as_ptr() as *const i8,
            None,
        ))
        .to_string_lossy()
        .into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_ne!(version().len(), 0);
    }

    #[test]
    fn test_license() {
        assert_ne!(license().len(), 0);
    }

    #[test]
    fn test_compile() {
        assert_ne!(compile("").len(), 0);
    }
}
