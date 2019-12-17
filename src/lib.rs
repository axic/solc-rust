//! Rust bindings for the Solidity compiler.
//!
//! # Example
//! ```
//! pub fn main() {
//!     // Let input be a valid "Standard Solidity Input JSON"
//!     let input = "{}";
//!     let output = solc::compile(&input);
//!     assert_ne!(output.len(), 0);
//! }

mod native;

use std::ffi::CStr;
use std::ffi::CString;

/// Returns the compiler version string.
pub fn version() -> String {
    unsafe {
        CStr::from_ptr(native::solidity_version())
            .to_string_lossy()
            .into_owned()
    }
}

/// Returns the complete license text.
pub fn license() -> String {
    unsafe {
        CStr::from_ptr(native::solidity_license())
            .to_string_lossy()
            .into_owned()
    }
}

/// Compile using a valid JSON input and return a JSON output.
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
    fn test_compile_smoke() {
        assert_ne!(compile("").len(), 0);
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
        let output = compile(&input);
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
        let output = compile(&input);
        // TODO: parse JSON and do a better job here
        assert_eq!(output.find("\"severity\":\"error\"").is_none(), false);
        assert_eq!(output.find(" not found: ").is_some(), true);
    }

    #[test]
    fn test_compile_callback() {
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
        let output = compile(&input);
        println!("output: {}", output);
        assert_eq!(output.find("\"severity\":\"error\"").is_some(), true);
        assert_eq!(output.find("\"object\":\"").is_none(), true);
        assert_eq!(output.find(" CODECOPY ").is_none(), true);
    }
}
