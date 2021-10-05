#[cfg(test)]
mod tests {
    use std::{ffi::CStr, os::raw::c_char};
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        unsafe { librclone_sys::RcloneInitialize() };

        let string: &str = "rc/noop";
        let bytes: Vec<u8> = String::from(string).into_bytes();
        let mut c_chars: Vec<i8> = bytes.iter().map(|c| *c as i8).collect::<Vec<i8>>();
        c_chars.push(0); // null terminator
        let ptr1: *mut c_char = c_chars.as_mut_ptr();

        let string: &str = "{}";
        let bytes: Vec<u8> = String::from(string).into_bytes();
        let mut c_chars: Vec<i8> = bytes.iter().map(|c| *c as i8).collect::<Vec<i8>>();
        c_chars.push(0); // null terminator
        let ptr2: *mut c_char = c_chars.as_mut_ptr();

        let result = unsafe { librclone_sys::RcloneRPC(ptr1, ptr2) };
        let c_str: &CStr = unsafe { CStr::from_ptr(result.Output) };
        let str_slice: &str = c_str.to_str().unwrap();
        let str_buf: String = str_slice.to_owned(); // if necessary
        dbg!(result, str_buf);
    }
}
