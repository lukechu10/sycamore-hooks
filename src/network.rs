//! Web socket hooks.

use std::cell::RefCell;

use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo::utils::errors::JsError;
use reqwasm::websocket;
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::WebSocketError;
use sycamore::futures::ScopeSpawnLocal;
use sycamore::prelude::*;

pub use reqwasm::websocket::Message;

#[derive(Clone, Copy)]
pub struct WebSocketHandle<'a> {
    ws_write: &'a RefCell<SplitSink<WebSocket, Message>>,
    state: &'a Signal<websocket::State>,
    message: &'a Signal<String>,
    message_bytes: &'a Signal<Vec<u8>>,
}

/// Opens a web socket connection at the specified `url`. The connection is closed when the enclosing scope is destroyed
/// or when [`WebSocketHandle::close`] is called.
pub fn use_web_socket<'a>(ctx: Scope<'a>, url: &str) -> Result<WebSocketHandle<'a>, JsError> {
    let state = ctx.create_signal(websocket::State::Closed);
    let message = ctx.create_signal(String::new());
    let message_bytes = ctx.create_signal(Vec::new());

    let ws = WebSocket::open(url)?;
    let (write, mut read) = ws.split();

    ctx.spawn_local(async move {
        while let Some(next) = read.next().await {
            if let Ok(m) = next {
                match m {
                    Message::Text(t) => message.set(t),
                    Message::Bytes(b) => message_bytes.set(b),
                }
            }
        }
    });

    Ok(WebSocketHandle {
        ws_write: ctx.create_ref(RefCell::new(write)),
        state,
        message,
        message_bytes,
    })
}

impl<'a> WebSocketHandle<'a> {
    pub async fn send(self, message: Message) -> Result<(), WebSocketError> {
        self.ws_write.borrow_mut().send(message).await
    }

    pub fn state(self) -> &'a ReadSignal<websocket::State> {
        self.state
    }

    pub fn message(self) -> &'a ReadSignal<String> {
        self.message
    }

    pub fn message_bytes(self) -> &'a ReadSignal<Vec<u8>> {
        self.message_bytes
    }

    /// NOTE: Not yet implemented due to technical reasons.
    pub fn close(self) {
        unimplemented!();
    }
}
