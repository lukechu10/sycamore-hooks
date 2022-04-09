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
    }
}
