#![allow(non_snake_case)]
use crate::components::{FormButton, FormInput};
use dioxus::prelude::*;

#[component]
pub fn PostCreate() -> Element {
    rsx!(
        h1 { "page for create post" }
        p { "How to build form using component?" }

        form {
            FormInput { name: "post name", placeholder: Some("title".to_string()) }
        }
    )
}
