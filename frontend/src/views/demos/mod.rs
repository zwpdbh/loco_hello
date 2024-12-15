#![allow(non_snake_case)]
use crate::routes::Route;
use dioxus::prelude::*;

mod async_with_coroutines;
mod async_with_resource;
mod async_with_spawn;
mod context;
mod dynamic_rendering;
mod event_handler;
mod hooks;
mod llm;
mod prop;
mod rsx_basic;
mod user_input;

pub use async_with_coroutines::DemoCoroutines;
pub use async_with_resource::DemoResource;
pub use async_with_spawn::DemoSpawn;
pub use context::DemoContext;
pub use dynamic_rendering::DemoDynamicRendering;
pub use event_handler::DemoEventHandler;
pub use hooks::DemoHooks;
pub use llm::DemoLLM;
pub use prop::DemoProp;
pub use rsx_basic::RsxBasic;
pub use user_input::UserInput;

const SIDEBAR_CSS: Asset = asset!("/assets/styling/sidebar.css");

/// Place holder for Demo section
#[component]
pub fn Demo() -> Element {
    rsx!(
        div { class: "columns",
            div { class: "column is-one-fifth", DemoMenu {} }
            div { class: "column", Outlet::<Route> {} }
        }
    )
}

#[component]
pub fn DemoMenuDefault() -> Element {
    rsx!()
}

/// This is the sidebar menu to show different demos for demo section
#[component]
fn DemoMenu() -> Element {
    rsx!(
        document::Link { rel: "stylesheet", href: SIDEBAR_CSS }
        aside { class: "w-64 h-screen bg-gray-100 p-4 overflow-y-auto",
            p { class: "text-gray-500 text-xs uppercase font-semibold mb-2", "General" }
            ul { class: "space-y-1 mb-4",
                li { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                    Link { to: Route::RsxBasic {}, "RsxBasic" }
                }
                li { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                    Link { to: Route::DemoProp {}, "Prop" }
                }
                li { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                    Link { to: Route::DemoEventHandler {}, "Event Handler" }
                }
                li { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                    Link { to: Route::DemoHooks {}, "Hooks" }
                }
                li { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                    Link { to: Route::UserInput {}, "User Input" }
                }
                li { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                    Link { to: Route::DemoContext {}, "Context" }
                }
                li { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                    Link { to: Route::DemoDynamicRendering {}, "Dynamic Rendering" }
                }
                li { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                    Link { to: Route::DemoResource {}, "Async with Resource" }
                }
                li { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                    Link { to: Route::DemoCoroutines {}, "Async with Coroutines" }
                }
                li { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                    Link { to: Route::DemoSpawn {}, "Async with Spawn" }
                }
            }
            p { class: "text-gray-500 text-xs uppercase font-semibold mb-2", "LLM service" }
            ul {
                li { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                    Link { to: Route::DemoLLM {}, "LLM service" }
                }
            }
            p { class: "text-gray-500 text-xs uppercase font-semibold mb-2", "ACStor CRUD" }
            ul {
                li {
                    a { class: "block hover:bg-gray-200 px-2 py-1 rounded", "Team Settings" }
                }
                li {
                    a { class: "block hover:bg-gray-200 px-2 py-1 rounded", "Manage Your Team" }
                    ul {
                        li {
                            a { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                                "Members"
                            }
                        }
                        li {
                            a { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                                "Plugins"
                            }
                        }
                        li {
                            a { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                                "Add a member"
                            }
                        }
                    }
                }
                li {
                    a { class: "block hover:bg-gray-200 px-2 py-1 rounded", "Invitations" }
                }
                li {
                    a { class: "block hover:bg-gray-200 px-2 py-1 rounded",
                        "Cloud Storage Environment Settings"
                    }
                }
                li {
                    a { class: "block hover:bg-gray-200 px-2 py-1 rounded", "Authentication" }
                }
            }
        }
    )
}

#[component]
fn MyCard(children: Element) -> Element {
    rsx!(
        div {
            div {
                // Notice the children is placed inside "{}"
                div { {children} }
            }
        }
    )
}
