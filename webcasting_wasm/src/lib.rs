use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub unsafe fn test(s: &str) {
    alert("test");
    alert(s);
}
