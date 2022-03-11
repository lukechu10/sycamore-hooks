use sycamore::prelude::*;

#[component]
fn App<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        "Hello World!"
    }
}

fn main() {
    sycamore::render(|ctx| view! { ctx,
        App {}
    });
}
