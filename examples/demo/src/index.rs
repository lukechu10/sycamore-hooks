use sycamore::prelude::*;

#[component]
pub fn Index<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        h1 { "Sycamore Hooks Demo" }
        h2 { "Network" }
        ul {
            li {
                a(href="demo/use_web_socket") { "use_web_socket" }
            }
        }
    }
}
