mod demo;
mod index;

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

#[derive(Debug, Clone, Copy, Route)]
enum Routes {
    #[to("/")]
    Index,
    #[to("/demo/websocket/use_web_socket")]
    UseWebSocketDemo,
    #[to("/demo/timer/create_polled")]
    CreatePolledDemo,
    #[to("/demo/window/use_title")]
    UseTitleDemo,
    #[to("/demo/window/use_full_screen")]
    UseFullScreenDemo,
    #[not_found]
    NotFound,
}

#[component]
fn App() -> View {
    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<Routes>| {
                view! {
                    div {
                        (match route.get() {
                            Routes::Index => view! { index::Index {} },
                            Routes::UseWebSocketDemo => view! { demo::websocket::UseWebSocket {} },
                            Routes::CreatePolledDemo => view! { demo::timer::CreatePolled {} },
                            Routes::UseTitleDemo => view! { demo::window::UseTitle {} },
                            Routes::UseFullScreenDemo => view! { demo::window::UseFullScreen {} },
                            Routes::NotFound => view! { "404 Not Found" },
                        })
                    }
                }
            }
        )
    }
}

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
