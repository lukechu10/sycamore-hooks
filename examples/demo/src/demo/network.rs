use sycamore::futures::ScopeSpawnLocal;
use sycamore::prelude::*;
use sycamore_hooks::{Message, use_web_socket};

#[component]
pub fn UseWebSocket<G: Html>(ctx: Scope) -> View<G> {
    let ws = use_web_socket(ctx, "wss://echo.websocket.events/")
        .expect("could not connect to web socket");
    let message = ws.message();

    let input = ctx.create_signal(String::new());

    view! { ctx,
        h2 { "use_web_socket" }

        input(bind:value=input) { "Input" }
        button(on:click=move |_| {
            ctx.spawn_local(async move {
                ws.send(Message::Text(input.get().as_ref().clone())).await.expect("could not send message");
            });
        }) { "Send!" }
        p { "Message: " (message.get()) }
    }
}
