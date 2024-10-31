#![allow(warnings)]

use std::collections::HashMap;
type Id = i64;
use rand::Rng;

use crate::Entity;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

const test: &str = include_str!("./shaders/test.wgsl");

pub fn load_shader(file_name: &str) -> &str {
    match file_name {
        "test.wgsl" => test,
        _ => "Нихуя"
    }
}

pub fn load_image() {

}

pub fn get_unique_id<T: 'static>(entity: &Entity) -> i64 {

    let mut rng = rand::thread_rng();
    let mut is_unique_id = true;

    loop {
        let id = rng.gen_range(std::i64::MIN..std::i64::MAX);

        for i in &entity.components {
            if let Some(x) = i.downcast_ref::<HashMap<Id, T>>() {

                for j in x.keys() {
                    if *j == id {
                        is_unique_id = false;
                    }
                }
            }
        }

        if is_unique_id { return id }
    }
}