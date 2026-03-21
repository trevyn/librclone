#![doc = include_str!("../README.md")]

use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

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
    let method_c = CString::new(method.into())
        .map_err(|_| "method contains an interior null byte".to_string())?;
    let input_c = CString::new(input.into())
        .map_err(|_| "input contains an interior null byte".to_string())?;

    let result = unsafe {
        librclone_sys::RcloneRPC(
            method_c.as_ptr() as *mut c_char,
            input_c.as_ptr() as *mut c_char,
        )
    };

    if result.Output.is_null() {
        return Err("RcloneRPC returned null output".to_string());
    }

    let output = unsafe { CStr::from_ptr(result.Output) }
        .to_string_lossy()
        .to_string();
    unsafe { librclone_sys::RcloneFreeString(result.Output) };

    if result.Status == 200 {
        Ok(output)
    } else {
        Err(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        initialize();
        assert_eq!(rpc("rc/noop", "{}"), Ok("{}\n".to_string()));
        assert_eq!(
            rpc("rc/error", "{}"),
            Err("{\n\t\"error\": \"arbitrary error on input map[]\",\n\t\"input\": {},\n\t\"path\": \"rc/error\",\n\t\"status\": 500\n}\n".to_string())
        );
        finalize();
    }
}
