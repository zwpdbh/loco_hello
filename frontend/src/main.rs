#![allow(non_snake_case)]

use dioxus::prelude::*;
use routes::Route;
use tracing::Level;

mod components;
mod error;
mod pages;
mod routes;

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}
