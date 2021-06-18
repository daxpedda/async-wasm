use js_sys::Promise;
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::console;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    JsFuture::from(waiting()).await?;
    console::info_1(&JsValue::from("this never happens"));

    Ok(())
}

fn waiting() -> Promise {
    Promise::new(&mut |_resolve, _fail| {
        let window = web_sys::window().expect("no global `window` exists");
        let fun = Closure::wrap(
            Box::new(|| console::info_1(&JsValue::from("this happens"))) as Box<dyn Fn()>
        );
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                fun.as_ref().unchecked_ref(),
                2000,
            )
            .expect("timeout failed");
        fun.forget()
    })
}
