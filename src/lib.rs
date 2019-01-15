mod native;

use std::ffi::CString;
use std::ffi::CStr;

pub fn version() -> String {
    unsafe {
        CStr::from_ptr(native::version()).to_string_lossy().into_owned()
    }
}

// FIXME support read callback
pub fn compile(input: String) -> String {
    let input_cstr = CString::new(input).expect("input expected");
    unsafe {
        CStr::from_ptr(native::compileStandard(input_cstr.as_ptr() as *const i8, None)).to_string_lossy().into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_version() {
        assert_ne!(version().len(), 0);
    }

    #[test]
    fn test_compile() {
       assert_ne!(compile("".to_string()).len(), 0);
    }
}
