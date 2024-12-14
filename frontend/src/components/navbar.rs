#![allow(non_snake_case)]
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx!(
        header { class: "bg-white",
            nav { class: "flex items-center justify-between w-[92%]  mx-auto",
                div { class: "w-16",
                    p { class: "uppercase font-semibold", "acstor" }
                }
                div {
                    ul { class: "flex items-center gap-[4vw]",
                        li {
                            Link {
                                class: "hover:text-gray-500",
                                to: Route::Home {},
                                "Home"
                            }
                        }
                        li {
                            Link {
                                class: "hover:text-gray-500",
                                to: Route::DemoMenuDefault {},
                                "Demos"
                            }
                        }
                        li {
                            Link {
                                class: "hover:text-gray-500",
                                to: Route::PostList {},
                                "Blog List"
                            }
                        }
                        li {
                            a { class: "hover:text-gray-500", "More" }
                        }
                    }
                }
                div {
                    button { class: "hover:text-gray-500", "Sign In " }
                }
            }
        }

        // The Outlet component will render child routes (In this case just the Home component) inside the Outlet component
        Outlet::<Route> {}
    )
}
