#![feature(result_flattening)]
#[rustfmt::skip::macros(view)]
use cfg_if::cfg_if;

mod about;
pub mod app;
mod projects;
mod art;
mod home;
mod dark_mode;

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        
        #[wasm_bindgen]
        pub fn hydrate() {
            use crate::app::App;
            use leptos::*;
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();
            mount_to_body(move || {
                view! { <App/> }
            });
        }
    }
}
