#![feature(proc_macro_hygiene)]

use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::MouseEvent;

use css_rs_macro::css;
use virtual_dom_rs::prelude::*;
use rust_psd::psd::Psd;
use std::include_bytes;

#[wasm_bindgen]
struct App {
  dom_updater: DomUpdater
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new () -> App {
        let start_view = html! { <div> Hello </div> };

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let mut dom_updater = DomUpdater::new_append_to_mount(start_view, &body);

        let greetings = "Hello, World!";

        let end_view = html! {
           // Use regular Rust comments within your html
           <div class="big blue">
              /* Interpolate values using braces */
              <strong>{ greetings }</strong>

              <button
                class=MY_COMPONENT_CSS
                onclick=|_event: MouseEvent| {
                   web_sys::console::log_1(&"Button Clicked!".into());
                   let context = include_bytes!("../../../example.psd");
                    // println!("context is : {:#?}", context);
                    
                    let psd = Psd::from_bytes(context).unwrap();
                    // web_sys::console::log_1(&psd);
                    console_log!("psd file is : {:#?}", psd);
                    // println!("fileHeader is : {:#?}", psd);

                }
              >
                // No need to wrap text in quotation marks (:
                Click me and check your console
              </button>
           </div>
        };

        dom_updater.update(end_view);

        App { dom_updater }
    }
}

static MY_COMPONENT_CSS: &'static str = css!{r#"
:host {
    font-size: 24px;
    font-weight: bold;
}
"#};

static _MORE_CSS: &'static str = css!{r#"
.big {
  font-size: 30px;
}

.blue {
  color: blue;
}
"#};

#[macro_export]
macro_rules! console_log {
  // Note that this is using the `log` function imported above during
  // `bare_bones`
  ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}
