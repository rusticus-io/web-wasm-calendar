use wasm_bindgen::prelude::*;

use js_sys::JsString;

#[wasm_bindgen(method)]
pub fn parse(out: JsString) -> JsString {
    let mut out: String = out.into();
    out.push_str(" bla");
    out.into()
}