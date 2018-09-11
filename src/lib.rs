mod native;

use std::ffi::CString;
use std::ffi::CStr;

pub fn version() -> String {
    unsafe {
        CStr::from_ptr(native::version()).to_string_lossy().into_owned()
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
}
