use sycamore::prelude::*;
use sycamore_hooks::reactive::create_toggle_bool;
use sycamore_hooks::window::{use_title, use_full_screen};

#[component]
pub fn UseTitle<G: Html>(cx: Scope) -> View<G> {
    #[component]
    fn Comp<G: Html>(cx: Scope) -> View<G> {
        use_title(cx, "Hello Title!");

        view! { cx,
            p { "My Component" }
        }
    }

    let (mounted, toggle) = create_toggle_bool(cx, false);

    view! { cx,
        h2 { "use_title" }

        button(type="button", on:click=move |_| toggle()) { "Toggle" }
        p { "Mounted: " (mounted.get()) }

        (if *mounted.get() {
            view! { cx, Comp {} }
        } else {
            view! { cx,}
        })
    }
}

#[component]
pub fn UseFullScreen<G: Html>(cx: Scope) -> View<G> {
    let (fs, toggle) = create_toggle_bool(cx, false);
    use_full_screen(cx, fs);

    view! { cx,
        h2 { "use_full_screen" }

        button(type="button", on:click=move |_| toggle()) { "Toggle Fullscreen" }
        p { "Fullscreen: " (fs.get()) }
    }
}
