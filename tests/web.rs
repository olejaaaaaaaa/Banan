//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

mod tests {
    use wasm_bindgen_futures::spawn_local;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    async fn fetch(url: &str) -> String {
        let client = reqwest::Client::new();
        let resp = client.get(url).send().await.unwrap();
        resp.text().await.unwrap()
    }
    
    #[wasm_bindgen_test]
    fn fetch_test() {
        spawn_local(async move {
            let result = fetch("https://ifconfig.me/ip").await;
            wasm_bindgen_test::console_log!("Your IP: {}", result);
        });
    }
}