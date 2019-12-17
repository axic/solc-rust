//! Rust bindings for the Solidity compiler.
//!
//! # Example
//! ```
//! pub fn main() {
//!     // This is an optional callback.
//!     let callback = |data: &str| -> Result<String, String> {
//!         if data == "file_i_have.sol" {
//!             Ok("contract C { function f() {} }".to_string())
//!         } else {
//!             Err("I don't have that file.".to_string())
//!         }
//!     };
//!     // Let input be a valid "Standard Solidity Input JSON"
//!     let input = "{}";
//!     let output = solc::compile(&input, Some(callback));
//!     assert_ne!(output.len(), 0);
//! }

mod native;

//#[macro_use]
//extern crate lazy_static;

use std::ffi::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Mutex;

/// Returns the compiler version string.
pub fn version() -> String {
//    unsafe {
//        CStr::from_ptr(native::solidity_version())
//            .to_string_lossy()
//            .into_owned()
//    }
    "A".to_string()
}

/// Returns the complete license text.
pub fn license() -> String {
//    unsafe {
//        CStr::from_ptr(native::solidity_license())
//            .to_string_lossy()
//            .into_owned()
//    }
    "A".to_string()
}
/*
trait CopyToSolidity {
    unsafe fn to_solidity(&self) -> *mut c_char;
}

impl CopyToSolidity for String {
    unsafe fn to_solidity(&self) -> *mut c_char {
        let len = self.len();
        // FIXME: use libsolc's exported malloc
//        let ptr = libc::malloc(len);
	let ptr = std::ptr::null_mut();
        // Don't assert on memory allocation failure
        if ptr != std::ptr::null_mut() {
            std::ptr::copy(self.as_ptr(), ptr as *mut u8, len);
        }
        ptr as *mut c_char
    }
}
*/

//unsafe fn from_solidity(

/// Read callback for the compiler asking for more input. The input argument is the filename
/// and the callback returns either the result or an error string.
pub type ReadCallback = fn(&str) -> Result<String, String>;

/*
lazy_static! {
    static ref CALLBACK: Mutex<Option<ReadCallback>> = Mutex::new(None);
}

fn callback_set(callback: Option<ReadCallback>) {
    *CALLBACK.lock().expect("Expected to acquire callback mutex") = callback;
}

fn callback_get() -> Option<ReadCallback> {
    *CALLBACK.lock().expect("Expected to acquire callback mutex")
}
*/
/*
unsafe extern "C" fn callback_wrapper(
    data: *const c_char,
    contents: *mut *mut c_char,
    error: *mut *mut c_char,
) {
let cb: Option<ReadCallback> = None;
//    let cb = callback_get();

    let ret = if cb.is_some() {
        assert!(data != std::ptr::null());

        let data = CStr::from_ptr(data)
            .to_str()
            .expect("non-UTF8 data received");

        Some(cb.unwrap()(&data))
    } else {
        None
    };

    if let Some(ret) = ret {
        println!("Callback was found");
        // Callback was found.
        if ret.is_err() {
            *error = ret.err().expect("error").to_solidity();
        } else {
            *contents = ret.ok().expect("result").to_solidity();
        }
        return;
    }

    // This means the callback was not supported (or provided)
    *contents = std::ptr::null_mut::<c_char>();
    *error = std::ptr::null_mut::<c_char>();
}
*/

/// Compile using a valid JSON input and return a JSON output.
pub fn compile(input: &str, callback: Option<ReadCallback>) -> String {
//    callback_set(callback);

    let input_cstr = CString::new(input).expect("CString failed (input contains a 0 byte?)");
//    let input_cstr = input_cstr.into_raw();
    let ret_raw_ptr = unsafe {
//        native::solidity_compile(input_cstr.as_ptr() as *const i8, Some(callback_wrapper))
        native::solidity_compile(input_cstr.as_ptr() as *const i8, None)
//        native::solidity_compile(input_cstr as *const i8, None)
    };
//    unsafe { CString::from_raw(input_cstr); }
//    let ret_raw_ptr = unsafe {
//        native::solidity_compile(input.as_ptr() as *const i8, None) //Some(callback_wrapper))
//    };

//    unsafe {
//        let ret_cstr = CStr::from_ptr(ret_raw_ptr);
//        let ret = ret_cstr.to_string_lossy().into_owned();
//        ret
//    }
    "Hello".to_string()
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
    fn test_compile_smoke() {
        assert_ne!(compile("", None).len(), 0);
    }

    #[test]
    fn test_compile_single() {
        let input = r#"
        {
          "language": "Solidity",
          "settings": {
            "outputSelection": {
              "*": {
                "*": [ "evm.bytecode", "evm.gasEstimates" ]
              }
            }
          },
          "sources": {
            "c.sol": {
              "content": "contract C { function g() public { } function h() internal {} }"
            }
          }
        }
        "#;
        let output = compile(&input, None);
        // TODO: parse JSON and do a better job here
        assert_eq!(output.find("\"severity\":\"error\"").is_none(), true);
        assert_eq!(output.find("\"object\":\"").is_some(), true);
        assert_eq!(output.find(" CODECOPY ").is_some(), true);
    }

    #[test]
    fn test_compile_multi_missing() {
        let input = r#"
        {
          "language": "Solidity",
          "settings": {
            "outputSelection": {
              "*": {
                "*": [ "evm.bytecode", "evm.gasEstimates" ]
              }
            }
          },
          "sources": {
            "c.sol": {
              "content": "import \"d.sol\"; contract C { function g() public { } function h() internal {} }"
            }
          }
        }
        "#;
        let output = compile(&input, None);
        // TODO: parse JSON and do a better job here
        assert_eq!(output.find("\"severity\":\"error\"").is_none(), false);
        assert_eq!(output.find(" not found: ").is_some(), true);
    }

    #[test]
    fn test_compile_callback() {
        let callback = |data: &str| -> Result<String, String> {
            if data == "d.sol" {
                Ok("contract D { function f() {} }".to_string())
            } else {
                Err("File not found.".to_string())
            }
        };
        let input = r#"
        {
          "language": "Solidity",
          "settings": {
            "outputSelection": {
              "*": {
                "*": [ "evm.bytecode", "evm.gasEstimates" ]
              }
            }
          },
          "sources": {
            "c.sol": {
              "content": "import \"d.sol\"; contract C is D { function g() public { } function h() internal {} }"
            }
          }
        }
        "#;
        let output = compile(&input, Some(callback));
        println!("output: {}", output);
        assert_eq!(output.find("\"severity\":\"error\"").is_none(), true);
        assert_eq!(output.find("\"object\":\"").is_some(), true);
        assert_eq!(output.find(" CODECOPY ").is_some(), true);
    }
}
