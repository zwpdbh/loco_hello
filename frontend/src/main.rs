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
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/tailwind.css"),
        }
        Router::<Route> {}
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}
