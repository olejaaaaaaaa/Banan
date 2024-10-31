
use wasm_bindgen::prelude::*;
use web_sys::AudioContext;
use web_sys::{HtmlAudioElement};

use wasm_bindgen::prelude::*;
use web_sys::{HtmlButtonElement, window};


struct ComponentAudio {

}

pub fn debug_play() {
    let document = window().unwrap().document().unwrap();

    // Создаем кнопку
    let button: HtmlButtonElement = document.create_element("button").unwrap()
            .dyn_into().unwrap();
    button.set_inner_html("Play Music");
    document.body().unwrap().append_child(&button).unwrap();

    // Создаем элемент audio
    let audio: HtmlAudioElement = document.create_element("audio").unwrap()
            .dyn_into().unwrap();
    audio.set_src("file.mp3"); // замените на путь к вашему файлу

    let audio_clone = audio.clone();
    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        audio_clone.play().unwrap();
    }) as Box<dyn FnMut(_)>);

    button.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget(); // сохраняем замыкание
}