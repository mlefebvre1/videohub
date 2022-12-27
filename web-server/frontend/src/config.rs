use lazy_static::lazy_static;
use wasm_bindgen::UnwrapThrowExt;

lazy_static! {
    pub static ref HOST_ADDRESS: String = {
        let location = web_sys::window().unwrap_throw().location();
        location
            .host()
            .unwrap_or_else(|_| "127.0.0.1:8000".to_string())
    };
}
