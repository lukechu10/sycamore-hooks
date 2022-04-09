//! Web socket hooks.

use std::cell::RefCell;

use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo::net::websocket;
use gloo::net::websocket::futures::WebSocket;
use gloo::net::websocket::WebSocketError;
use gloo::utils::errors::JsError;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;

pub use gloo::net::websocket::Message;

#[derive(Clone, Copy)]
pub struct WebSocketHandle<'a> {
    ws_write: &'a RefCell<SplitSink<WebSocket, Message>>,
    state: &'a Signal<websocket::State>,
    message: &'a Signal<String>,
    message_bytes: &'a Signal<Vec<u8>>,
}

/// Opens a web socket connection at the specified `url`. The connection is closed when the enclosing scope is destroyed
/// or when [`WebSocketHandle::close`] is called.
pub fn use_web_socket<'a>(cx: Scope<'a>, url: &str) -> Result<WebSocketHandle<'a>, JsError> {
    let state = create_signal(cx, websocket::State::Closed);
    let message = create_signal(cx, String::new());
    let message_bytes = create_signal(cx, Vec::new());

    let ws = WebSocket::open(url)?;
    let (write, mut read) = ws.split();

    spawn_local_scoped(cx, async move {
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
        ws_write: create_ref(cx, RefCell::new(write)),
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
