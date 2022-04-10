use sycamore::prelude::*;

#[component]
pub fn Index<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        h1 { "Sycamore Hooks Demo" }
        h2 { "Network" }
        ul {
            li {
                a(href="demo/net/use_web_socket") { "use_web_socket" }
            }
        }
        h2 { "Reactive" }
        ul {
            li {
                "until"
            }
            li {
                "create_toggle"
            }
            li {
                "create_bool_toggle"
            }
            li {
                "create_counter"
            }
        }
        h2 { "Window" }
        ul {
            li {
                a(href="demo/window/use_title") { "use_title" }
            }
            li {
                a(href="demo/window/use_full_screen") { "use_full_screen" }
            }
        }
    }
}
