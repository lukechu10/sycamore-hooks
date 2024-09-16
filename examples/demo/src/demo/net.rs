use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore_hooks::net::{use_web_socket, Message};

#[component]
pub fn UseWebSocket() -> View {
    let ws = use_web_socket("wss://echo.websocket.events/")
        .expect("could not connect to web socket");
    let message = ws.message();

    let input = create_signal(String::new());

    view! {
        h2 { "use_web_socket" }

        input(bind:value=input) { "Input" }
        button(r#type="button", on:click=move |_| {
            spawn_local_scoped(async move {
                ws.send(Message::Text(input.get_clone())).await.expect("could not send message");
            });
        }) { "Send!" }
        p { "Message: " (message.get_clone()) }
    }
}
