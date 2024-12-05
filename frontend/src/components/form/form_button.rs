use dioxus::{events::MouseEvent, prelude::*};

// #[derive(Props)]
// pub struct FormButtonProps<'a> {
//     onclick: EventHandler<'a, MouseEvent>,
//     label: String,
// }

// pub fn FormButton_Lg<'a>(cx: Scope<'a, FormButtonProps<'a>>) -> Element {
//     cx.render(rsx! {
//         button {
//             class: "btn btn-lg btn-primary pull-xs-right",
//             r#type: "button",
//             onclick: move |evt| cx.props.onclick.call(evt),
//             "{cx.props.label}"
//         }
//     })
// }

#[component]
pub fn FormButton(label: String, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "btn btn-lg btn-primary pull-xs-right",
            r#type: "button",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}
