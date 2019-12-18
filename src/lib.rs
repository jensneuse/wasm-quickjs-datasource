use std::ffi::{CStr};
use std::mem;
use std::os::raw::{c_char, c_void};
use quick_js::{Context};
use quick_js;

#[no_mangle]
pub extern fn allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub extern fn deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern fn invoke(input: *mut c_char) -> *const u8 {
    let code = unsafe { CStr::from_ptr(input).to_str().unwrap() };

    let context = Context::new().unwrap();
    let value = context.eval(code).unwrap();

    value.as_str().unwrap().as_ptr()
}

#[cfg(test)]
mod tests {
    use quick_js::{Context,JsValue};
    #[test]
    fn quick_js() {
        let context = Context::new().unwrap();
        let value = context.eval("1 + 2").unwrap();
        assert_eq!(value, JsValue::Int(3));
    }
}