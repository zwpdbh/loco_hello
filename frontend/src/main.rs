use sycamore::prelude::*;

fn main() {
    sycamore::render(|| {
        view! {
            h1 { "Hello, world!" }
            p { "This is my first Sycamore app !" }
        }
    });
}
