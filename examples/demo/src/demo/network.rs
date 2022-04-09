use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore_hooks::network::{use_web_socket, Message};

#[component]
pub fn UseWebSocket<G: Html>(cx: Scope) -> View<G> {
    let ws = use_web_socket(cx, "wss://echo.websocket.events/")
        .expect("could not connect to web socket");
    let message = ws.message();

    let input = create_signal(cx, String::new());

    view! { cx,
        h2 { "use_web_socket" }

        input(bind:value=input) { "Input" }
        button(on:click=move |_| {
            spawn_local_scoped(cx, async move {
                ws.send(Message::Text(input.get().as_ref().clone())).await.expect("could not send message");
            });
        }) { "Send!" }
        p { "Message: " (message.get()) }
    }
}
