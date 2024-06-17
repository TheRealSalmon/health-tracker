pub mod delete_data;
pub mod get_data;
pub mod println;
pub mod submit_data;

pub use delete_data::delete_data;
pub use get_data::get_data;
#[allow(unused_imports)]
pub use println::println;
pub use submit_data::submit_data;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
