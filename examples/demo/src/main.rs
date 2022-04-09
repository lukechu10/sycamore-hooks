mod demo;
mod index;

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

#[derive(Debug, Clone, Copy, Route)]
enum Routes {
    #[to("/")]
    Index,
    // region: Demo pages
    #[to("/demo/net/use_web_socket")]
    WebSocketDemo,
    // endregion
    #[not_found]
    NotFound,
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Router {
            integration: HistoryIntegration::new(),
            view: |cx, route: &ReadSignal<Routes>| {
                view! { cx,
                    div {
                        (match *route.get() {
                            Routes::Index => view! { cx, index::Index {} },
                            Routes::WebSocketDemo => view! { cx, demo::net::UseWebSocket {} },
                            Routes::NotFound => view! { cx, "404 Not Found" },
                        })
                    }
                }
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|cx| {
        view! { cx,
            App {}
        }
    });
}
