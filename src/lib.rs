#![doc = include_str!("../README.md")]

use std::{ffi::CStr, os::raw::c_char};

/// Initializes rclone as a library.
pub fn initialize() {
    unsafe { librclone_sys::RcloneInitialize() };
}

/// Finalizes rclone as a library. Currently just calls the Go GC; don't stress if you never call it. :-)
pub fn finalize() {
    unsafe { librclone_sys::RcloneFinalize() };
}

/// Does a single librclone RPC call.
/// - `method`: e.g. `operations/list`, from <https://rclone.org/rc/#supported-commands>
/// - `input`: a serialized JSON object.
/// - Return value (`Ok` or `Err`) is a serialized JSON String.
pub fn rpc<S1: Into<String>, S2: Into<String>>(method: S1, input: S2) -> Result<String, String> {
    let method_bytes: Vec<u8> = method.into().into_bytes();
    let mut method_c_chars: Vec<i8> = method_bytes.iter().map(|c| *c as i8).collect::<Vec<i8>>();
    method_c_chars.push(0); // null terminator
    let method_mut_ptr: *mut c_char = method_c_chars.as_mut_ptr();

    let input_bytes: Vec<u8> = input.into().into_bytes();
    let mut input_c_chars: Vec<i8> = input_bytes.iter().map(|c| *c as i8).collect::<Vec<i8>>();
    input_c_chars.push(0); // null terminator
    let input_mut_ptr: *mut c_char = input_c_chars.as_mut_ptr();

    let result = unsafe { librclone_sys::RcloneRPC(method_mut_ptr, input_mut_ptr) };
    let output_c_str: &CStr = unsafe { CStr::from_ptr(result.Output) };
    let output_slice: &str = output_c_str.to_str().unwrap();
    let output: String = output_slice.to_owned();
    unsafe { librclone_sys::RcloneFreeString(result.Output) };

    match result.Status {
        200 => Ok(output),
        _ => Err(output),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        initialize();
        assert_eq!(rpc("rc/noop", "{}"), Ok("{}\n".to_string()));
        assert_eq!(rpc("rc/error", "{}"), Err("{\n\t\"error\": \"arbitrary error on input map[]\",\n\t\"input\": {},\n\t\"path\": \"rc/error\",\n\t\"status\": 500\n}\n".to_string()));
        finalize();
    }
}
