use miniserde::{json, Serialize, Deserialize};
use std::alloc::{alloc, dealloc, Layout};

#[no_mangle]
pub unsafe fn wasm_alloc(size: usize) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    alloc(layout)
}

#[no_mangle]
pub unsafe fn wasm_dealloc(ptr: *mut u8, size: usize) {
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    dealloc(ptr, layout);
}

#[derive(Serialize, Deserialize)]
struct JsonResult<T: Serialize> {
    err: Option<String>,
    value: Option<T>
}

#[derive(Serialize, Deserialize)]
struct HelloWow {
    hey: String,
    wow: String,
}

#[derive(Serialize, Deserialize)]
struct HelloWowResult {
    here: String,
}

#[no_mangle]
pub unsafe fn json_call(ptr: *mut u8, len: usize, ret_size: *mut u32) -> *mut u8 {
    let data = Vec::from_raw_parts(ptr, len, len);
    let json_str: String = String::from_utf8_lossy(&data).to_string();

    let my_struct: HelloWow = match json::from_str(&json_str) {
        Ok(res) => res,
        Err(_) => {
            return serialize_result::<HelloWowResult>(None, Some("json deserialization failed".to_string()), ret_size);
        }
    };

    let ret_value = HelloWowResult{
        here: format!("Omg! {}", my_struct.wow),
    };
    serialize_result(Some(ret_value), None, ret_size)
}

unsafe fn serialize_result<T>(result: Option<T>, err: Option<String>, ret_size: *mut u32) -> *mut u8
    where T: Serialize {

    let ret = JsonResult{
        err,
        value: result,
    };
    let mut bytes = json::to_string(&ret).into_bytes().to_owned();
    let ret_ptr = bytes.as_mut_ptr();
    *ret_size = bytes.len() as u32;
    // Tell rust not to drop bytes
    std::mem::forget(bytes);
    ret_ptr
}

