use js_sys::{Function, Object, Reflect, WebAssembly};
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

    let wbg = Object::new();

    js_sys::Reflect::set(
        &wbg,
        &"__wbindgen_throw".into(),
        &Function::new_no_args("").into(),
    )
    .unwrap();

    let imports = Object::new();
    js_sys::Reflect::set(&imports, &"wbg".into(), &wbg).unwrap();

    tracing::info!("Imports: {imports:?}");

    // Returns a `{ module, instance }` object.
    let res = JsFuture::from(WebAssembly::instantiate_streaming(&resp, &imports))
        .await
        .unwrap();

    let instance = Reflect::get(&res, &"instance".into())
        .unwrap()
        .dyn_into::<WebAssembly::Instance>()
        .unwrap();

    // let module = Reflect::get(res, "instance"): tracing::info!("Instantiated WebAssembly instance");

    let exports = instance.exports();

    tracing::info!("Module exports: {exports:?}");
    //
    // let add = Reflect::get(exports.as_ref(), &"call_int2".into())
    //     .expect("add export wasn't found")
    //     .dyn_into::<Function>()
    //     .expect("add export wasn't a function");
    //
    // let result = add.call2(&JsValue::null(), &1.into(), &2.into()).unwrap();
    //
    // tracing::info!("Add function: {add:?} = {result:?}");

    let call_timestamp = Reflect::get(exports.as_ref(), &"call_timestamp".into())
        .expect("timestamp export wasn't found")
        .dyn_into::<Function>()
        .expect("add export wasn't a function");

    let result = call_timestamp.call0(&JsValue::null()).unwrap();

    tracing::info!("Date time stamp: {result:?}");

    println!("Hello, world!");
}
