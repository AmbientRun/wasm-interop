use wasm_bindgen::prelude::wasm_bindgen;

#[no_mangle]
pub extern "C" fn call_int2(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn call_str(left: String, right: &str) -> String {
    format!("{}{}", left, right)
}

#[repr(C)]
pub struct Composite {
    a: i32,
    f: f32,
}

pub extern "C" fn call_struct(s: Composite) -> i32 {
    s.a
}
