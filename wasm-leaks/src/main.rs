use miniserde::{json, Serialize, Deserialize};
use rust_wasm;


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


fn main() {
    //let mut input_str = json::to_string(&HelloWow{hey: "heyyy".to_string(), wow: "wowwww".to_string()});
    let mut input_str = "{{{{garbage".to_string();
    let mut output_len: u32 = 0;
    let output_ptr = &mut output_len as *mut u32;

    unsafe {
        let res = rust_wasm::json_call(input_str.as_mut_ptr(), input_str.len(), output_ptr);

        println!("{output_len}");
        let bytes = std::slice::from_raw_parts(res, output_len as usize);
        let output = String::from_utf8_lossy(&bytes);
        println!("{output}");

        rust_wasm::wasm_dealloc(res, output_len as usize);
    }

    std::mem::forget(input_str);
}
