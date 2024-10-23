

use log::warn;
use pollster::FutureExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response, Window};

// #[wasm_bindgen]
// pub async fn fetch_data(url: &str) {
    
//     let window = web_sys::window().ok_or("no global window available").unwrap();
    
//     let mut opts = RequestInit::new();
//     opts.method("GET");

//     let request = Request::new_with_str_and_init(url, &opts).unwrap();

//     let response_value = JsFuture::from(window.fetch_with_request(&request));
//     //let response: Response = response_value.into();

//     // if response.ok() {
//     //     //let text_value = JsFuture::from(response.text().unwrap()).await.unwrap();
//     //     warn!("{:?}", "OK");
//     // } else {
        
//     // }

// }
