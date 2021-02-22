#[allow(dead_code)]
mod both;
#[allow(dead_code)]
mod pack;

#[cfg(not(feature = "server"))]
use {
    crate::both::*,
    crate::pack::Pack,
    serde_json::{json, Value},
    wasm_bindgen::prelude::*,
    wasm_bindgen::JsCast,
    web_sys::{MessageEvent, WebSocket},
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(not(feature = "server"))]
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Location of the PAC websocket server
#[cfg(not(feature = "server"))]
static PAC_SERVER: &'static str = env!("PAC_SERVER");

#[cfg(not(feature = "server"))]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// PAC entrypoint
///
/// Called when the WASM module is loaded via a web worker. Threading and gpu acceleration are currently limited in WASM
/// therefore the current setup is strictly a synchronous example. Development of these modules can be followed on the
/// [threads repository](https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md) and [gpuweb repository](https://github.com/gpuweb/gpuweb/wiki/Implementation-Status).
/// Todo: Feature gate node connection API to enable native compilation
#[cfg(not(feature = "server"))]
#[wasm_bindgen]
pub fn initialize() {
    // Initialize connection to PAC server
    let ws = WebSocket::new(PAC_SERVER).expect("Error initializing websocket!");
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    // Initialize local PAC state
    let pack = Pack::new();

    // Bind websocket onmessage callback to PAC events
    let cloned_ws = ws.clone();
    let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        // Try to deserialize message into a PacEvent
        let deserialize: Result<PacEvent, _> =
            serde_json::from_str(&event.data().as_string().unwrap());

        if let Ok(msg) = deserialize {
            // For now executing log running tasks in a proper threaded way is not possible so we will run tasks in sync
            match msg.event {
                EventType::Start(hash) => {
                    if let Some(result) = pack.start(hash) {
                        cloned_ws.send_with_str(
                            &serde_json::to_string::<PacEvent>(&PacEvent::resolved(result))
                                .unwrap(),
                        );
                        cloned_ws.send_with_str(
                            &serde_json::to_string::<PacEvent>(&PacEvent::request()).unwrap(),
                        );
                    } else {
                        cloned_ws.send_with_str(
                            &serde_json::to_string::<PacEvent>(&PacEvent::request()).unwrap(),
                        );
                    }
                }
                EventType::Stop => {
                    // Todo: Stop websocket
                }
                _ => {}
            }
        }
    }) as Box<dyn FnMut(MessageEvent)>);

    // Set callback and forget
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    // Request work from the PAC server when we connect
    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        log("PAC connected!");
        cloned_ws.send_with_str(&serde_json::to_string::<PacEvent>(&PacEvent::request()).unwrap());
    }) as Box<dyn FnMut(JsValue)>);

    // Set callback and forget
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
}
