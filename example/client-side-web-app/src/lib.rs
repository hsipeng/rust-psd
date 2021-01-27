#![feature(proc_macro_hygiene)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::*;
use web_sys::*;
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
          //  <div class="big blue">
          //     /* Interpolate values using braces */
          //     <strong>{ greetings }</strong>

          //     <button
          //       class=MY_COMPONENT_CSS
          //       onclick=|_event: MouseEvent| {
          //          web_sys::console::log_1(&"Button Clicked!".into());
          //          let context = include_bytes!("../../../example.psd");
          //           // println!("context is : {:#?}", context);
                    
          //           let psd = Psd::from_bytes(context).unwrap();
          //           // web_sys::console::log_1(&psd);
          //           console_log!("psd file is : {:#?}", psd);
          //           // println!("fileHeader is : {:#?}", psd);

          //       }
          //     >
          //       // No need to wrap text in quotation marks (:
          //       Click me and check your console
          //     </button>
          //  </div>
           <div class=APP_CONTAINER>
            <div class="left-column">
              <h2>Psd</h2>
              <div class="psd-drop-area"
              ondragenter=|event: web_sys::DragEvent|{
                event.prevent_default();
                event.stop_propagation();
              }

              ondragover=|event: web_sys::DragEvent|{
                event.prevent_default();
                event.stop_propagation();
              }

              ondrop=|event: web_sys::DragEvent|{
                event.prevent_default();
                event.stop_propagation();
                let dt = event.data_transfer().unwrap();
                let files = dt.files().unwrap();
                let psd = files.item(0).unwrap();
                let file_reader = web_sys::FileReader::new().unwrap();
                file_reader.read_as_array_buffer(&psd).unwrap();
                // console_log!("psd file is : {:#?}", psd);
                let mut onload = Closure::wrap(Box::new(move | event: web_sys::Event | {
                  let file_reader: FileReader = event.target().unwrap().dyn_into().unwrap();
                  let psd = file_reader.result().unwrap();
                  let psd = js_sys::Uint8Array::new(&psd);
                  console_log!("psd Uint8Array is : {:#?}", &psd);
                  let mut psd_file = vec![0; psd.length() as usize];
                  psd.copy_to(&mut psd_file);

                  let psd = Psd::from_bytes(&psd_file).unwrap();
                  console_log!("psd file is : {:#?}", psd);
                }) as Box<FnMut(_)>);
                file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();
                // let file_reader = web_sys::FileReader::new().unwrap();
                // file_reader.read_as_array_buffer(&psd).unwrap();
                // let psd = file_reader.result().unwrap();
                // let psd = js_sys::Uint8Array::new(&psd);
                // console_log!("psd Uint8Array is : {:#?}", &psd);
                // let mut psd_file = vec![0; psd.length() as usize];
                // psd.copy_to(&mut psd_file[..]);
                // // psd_file = psd.to_vec();
                // let psd = Psd::from_bytes(&psd_file).unwrap();
                // console_log!("psd file is : {:#?}", psd);
              }
              >
                <strong>{" Drag and Drop here to upload a PSD file"}</strong>
              </div>
            </div>
            <div class="right-column">
              <h2>Layers</h2>
            </div>
          </div>
        };

        dom_updater.update(end_view);

        App { dom_updater }
    }
}

static APP_CONTAINER: &'static str = css!{r#"
:host {
  display: flex;
  width: 100%;
  height: 100%;
}
"#};


static _LAYOUT: &'static str = css! {r#"
.left-column {
}

.psd-drop-area{
  height: 100px;
  display: flex;
  align-items: center;
  justify-content: center;
}
.right-column {
    background-color: #f7f7f7;
    padding-left: 5px;
    padding-right: 5px;
}

.layer-dark-background {
   background-color: #b8b8b8;
}

.layer-light-background {
   background-color: #e0e0e0;
}
"#};

#[macro_export]
macro_rules! console_log {
  // Note that this is using the `log` function imported above during
  // `bare_bones`
  ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}
