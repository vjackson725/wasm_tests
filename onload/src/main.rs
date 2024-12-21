
use wasm_bindgen::prelude::*;
use web_sys::{console, window};

fn main() {
  console_error_panic_hook::set_once();

  let callback = {
    Closure::<dyn Fn()>::new(move || {
      console::log_1(&"loaded (from Rust)".into());
    })
  };
  let _ = {
    window()
      .expect("window")
      .add_event_listener_with_callback(
        "load",
        JsCast::unchecked_ref(callback.as_ref())
      )
  };
  callback.forget();


  let document = window()
    .and_then(|win| win.document())
    .expect("Could not access the document");
  let body = document.body().expect("Could not access document.body");
  let text_node = document.create_text_node("Running!");
  body.append_child(text_node.as_ref())
    .expect("Failed to append text");
}
