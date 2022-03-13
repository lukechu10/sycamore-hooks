mod demo;
mod index;

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

#[derive(Debug, Clone, Copy, Route)]
enum Routes {
    #[to("/")]
    Index,
    // region: Demo pages
    #[to("/demo/network/use_web_socket")]
    WebSocketDemo,
    // endregion
    #[not_found]
    NotFound,
}

#[component]
fn App<G: Html>(ctx: Scope) -> View<G> {
    view! { ctx,
        Router {
            integration: HistoryIntegration::new(),
            view: |ctx, route: &ReadSignal<Routes>| {
                view! { ctx,
                    div {
                        (match *route.get() {
                            Routes::Index => view! { ctx, index::Index {} },
                            Routes::WebSocketDemo => view! { ctx, demo::network::UseWebSocket {} },
                            Routes::NotFound => view! { ctx, "404 Not Found" },
                        })
                    }
                }
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|ctx| {
        view! { ctx,
            App {}
        }
    });
}
