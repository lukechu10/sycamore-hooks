use sycamore::prelude::*;

#[component]
pub fn Index() -> View {
    view! {
        h1 { "Sycamore Hooks Demo" }
        h2 { "Network" }
        ul {
            li {
                a(href="demo/websocket/use_web_socket") { "use_web_socket" }
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
        h2 { "Timer" }
        ul {
            li {
                "create_timeout"
            }
            li {
                "create_interval"
            }
            li {
                a(href="demo/timer/create_polled") { "create_polled" }
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
