#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod comms;
pub mod desktop;
pub mod encryption;
pub mod pages;
pub mod types;

fn main() {
    println!("Hello, world!");
    dioxus_desktop::launch(App)
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        "Hello, World!"
    })
}
