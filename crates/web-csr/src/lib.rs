mod counter;

use wasm_bindgen::prelude::*;
use web_sys::console;

// Called by our JS entry point to run the example.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {

    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    let ready_state = document.ready_state();
    
    if ready_state == "loading" {

        console::log_1(&"Dom is ready".into());
        // Document not yet fully parsed, listen for DOMContentLoaded
        let closure = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
            counter::load().unwrap();
        }) as Box<dyn FnMut(_)>);
        
        document.add_event_listener_with_callback("DOMContentLoaded", closure.as_ref().unchecked_ref())?;
        closure.forget();
    } else {
        console::log_1(&"Dom is already ready".into());
        // DOM already ready, just run load directly
        counter::load()?;
    }

    Ok(())
}