use sycamore::prelude::*;
use sycamore_hooks::reactive::create_toggle_bool;
use sycamore_hooks::window::{use_title, use_full_screen};

#[component]
pub fn UseTitle() -> View {
    #[component]
    fn Comp() -> View {
        use_title("Hello Title!");

        view! {
            p { "My Component" }
        }
    }

    let (mounted, toggle) = create_toggle_bool(false);

    view! {
        h2 { "use_title" }

        button(r#type="button", on:click=move |_| toggle()) { "Toggle" }
        p { "Mounted: " (mounted.get().to_string()) }

        (if mounted.get() {
            view! { Comp {} }
        } else {
            view! { }
        })
    }
}

#[component]
pub fn UseFullScreen() -> View {
    let (fs, toggle) = create_toggle_bool(false);
    use_full_screen(fs);

    view! {
        h2 { "use_full_screen" }

        button(r#type="button", on:click=move |_| toggle()) { "Toggle Fullscreen" }
        p { "Fullscreen: " (fs.get().to_string()) }
    }
}
