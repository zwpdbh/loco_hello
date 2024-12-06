#![allow(non_snake_case)]
use crate::components::{FormButton, FormInput};
use crate::error::Result;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PostCreated {
    id: i32,
    title: String,
    content: String,
}

pub async fn create_post(title: &str, content: &str) -> Result<PostCreated> {
    let client = Client::new();
    let post_created = client
        .post("http://localhost:5150/api/posts")
        .json(&json!({
            "title": title,
            "content": content
        }))
        .send()
        .await?
        .json::<PostCreated>()
        .await?;

    Ok(post_created)
}

#[derive(Debug, PartialEq, Clone)]
pub enum CreatedPost {
    NotStarted,
    InProgress,
    Finished(PostCreated),
    Error(String),
}

#[component]
pub fn PostCreate() -> Element {
    let mut title = use_signal(|| "".to_string());
    let mut content = use_signal(|| "".to_string());

    let mut created_post = use_signal(|| CreatedPost::NotStarted);
    // let mut create_post_future: Resource<Result<PostCreated>> =
    //     use_resource(move || async move { create_post(&title(), &content()).await });

    let post_created = move |_event: MouseEvent| {
        info!("Submitting post - Title: {}, Content: {}", title, content);
        let _ = spawn(async move {
            created_post.set(CreatedPost::InProgress);

            let post = create_post(&title(), &content()).await;
            match post {
                Ok(post) => created_post.set(CreatedPost::Finished(post)),
                Err(e) => created_post.set(CreatedPost::Error(e.to_string())),
            }
        });
    };

    rsx!(
        h1 { "page for create post" }
        p { "How to build form using component?" }

        form {
            div { class: "field",
                label { class: "label", "Post title" }
                div { class: "control" }
                input { value: "{title}", oninput: move |e| title.set(e.value()) }
            }

            div { class: "field",
                label { class: "label", "Post content" }
                div { class: "control" }
                input {
                    value: "{content}",
                    oninput: move |e| content.set(e.value())
                }
            }

            div { class: "field",
                p { class: "control",
                    button {
                        class: "button is-primary",
                        onclick: post_created,
                        prevent_default: "onclick",
                        "Submit"
                    }
                }
            }
        }

        // Render created post based on condition
        CreatedPostStatus { created_post: created_post() }
    )
}

#[component]
fn CreatedPostStatus(created_post: CreatedPost) -> Element {
    rsx!(
        div { class: "mt-4",
            h2 { "Created Post Status" }
            match created_post {
                CreatedPost::NotStarted => rsx!(
                    p { "No post has been created yet." }
                ),
                CreatedPost::InProgress => rsx!(
                    p { "Creating post..." }
                    // You could add a loading spinner here
                ),
                CreatedPost::Finished(post) => rsx!(
                    div {
                        h3 { "Post Created Successfully" }
                        p { "Title: {post.title}" }
                        p { "Content: {post.content}" }
                        p { "ID: {post.id}" }
                    }
                ),
                CreatedPost::Error(e) => rsx!(
                    div { class: "error",
                        h3 { "Error Creating Post" }
                        p { "An error occurred: {e}" }
                    }
                ),
            }
        }
    )
}
