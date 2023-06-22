use js_sys::{Object, Reflect, WebAssembly};
use tracing_subscriber::{fmt::time::UtcTime, prelude::*};
use tracing_web::MakeConsoleWriter;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub async fn start(client_wasm_url: &str) {
    console_error_panic_hook::set_once();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(MakeConsoleWriter);

    tracing_subscriber::registry().with(fmt_layer).init();

    tracing::info!("Fetching: {client_wasm_url}");

    let window = web_sys::window().unwrap();
    let resp = window.fetch_with_str(client_wasm_url);

    // let resp = JsFuture::from(window.fetch_with_str(client_wasm_url))
    //     .await
    //     .unwrap()
    //     .dyn_into::<Response>()
    //     .unwrap();

    let imports = Object::new();

    // Returns a `{ module, instance }` object.
    let res = JsFuture::from(WebAssembly::instantiate_streaming(&resp, &imports))
        .await
        .unwrap();

    let instance = Reflect::get(&res, &"instance".into())
        .unwrap()
        .dyn_into::<WebAssembly::Instance>()
        .unwrap();

    tracing::info!("Instantiated WebAssembly instance");

    // let module = Reflect::get(res, "instance"): tracing::info!("Instantiated WebAssembly instance");

    let exports = instance.exports();

    tracing::info!("Module exports: {exports:?}");

    // tracing::info!(?resp, status=?resp.status(), "Got response");

    println!("Hello, world!");
}
