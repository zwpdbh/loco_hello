#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn PostList() -> Element {
    rsx!(
        h1 { "page for list posts" }
    )
}
