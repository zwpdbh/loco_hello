#![allow(non_snake_case)]
use super::post_create::{CreatePostResult, PostCreated};
use crate::error::Result;
use dioxus::prelude::*;
use dioxus_sortable::{use_sorter, NullHandling, PartialOrdBy, SortBy, Sortable, Th, ThStatus};
use reqwest::Client;
use tracing::info;

async fn get_posts() -> Result<Vec<PostCreated>> {
    let client = Client::new();
    let posts = client
        .get("http://localhost:5150/api/posts")
        .send()
        .await?
        .json::<Vec<PostCreated>>()
        .await?;

    Ok(posts)
}

#[derive(Debug, PartialEq, Clone)]
pub enum GetPostsResult {
    NotStarted,
    InProgress,
    Finished(Vec<PostCreated>),
    Error(String),
}

#[component]
pub fn PostList() -> Element {
    let mut get_posts_result = use_signal(|| GetPostsResult::NotStarted);

    let get_posts_action = move |_event: MouseEvent| {
        info!("Getting posts");
        let _ = spawn(async move {
            get_posts_result.set(GetPostsResult::InProgress);

            let posts = get_posts().await;
            match posts {
                Ok(posts) => get_posts_result.set(GetPostsResult::Finished(posts)),
                Err(e) => get_posts_result.set(GetPostsResult::Error(e.to_string())),
            }
        });
    };

    rsx!(
        h1 { "page for list posts" }

        div {
            button { class: "button", onclick: get_posts_action, "Get Posts" }
        }

        RenderGetPostsResult { get_posts_result: get_posts_result() }
    )
}

#[component]
fn RenderGetPostsResult(get_posts_result: GetPostsResult) -> Element {
    rsx!(
        div { class: "mt-4",
            match get_posts_result {
                GetPostsResult::NotStarted => rsx! {
                    "No posts has been fetched"
                },
                GetPostsResult::InProgress => rsx! {
                    p { "Getting posts..." }
                },
                GetPostsResult::Finished(posts) => rsx! {
                    p {"successfully get posts: {posts.len()}"}
                },
                GetPostsResult::Error(e) => rsx! {
                    h3 { "Error Creating Post" }
                    p { "An error occurred: {e}" }
                }
            }
        }
    )
}
