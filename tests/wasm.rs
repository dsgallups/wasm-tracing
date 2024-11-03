use wasm_bindgen_test::*;
use wasm_tracing::prelude::*;
#[wasm_bindgen_test]
pub fn test() {
    wasm_tracing::set_as_global_default();
}
