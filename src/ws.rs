//! Web socket hooks.

use std::cell::RefCell;

use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo::utils::errors::JsError;
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::{self, WebSocketError};
use sycamore::futures::ScopeSpawnLocal;
use sycamore::prelude::*;

pub use reqwasm::websocket::Message;

pub struct WebSocketHandle<'a> {
    ws_write: RefCell<SplitSink<WebSocket, Message>>,
    state: &'a Signal<websocket::State>,
    message: &'a Signal<String>,
    message_bytes: &'a Signal<Vec<u8>>,
}

/// Extension trait for [`Scope`].
pub trait ScopeUseWebSocket<'a> {
    /// Opens a web socket connection at the specified `url`. The connection is closed when the enclosing scope is destroyed
    /// or when [`WebSocketHandle::close`] is called.
    fn use_web_socket(&'a self, url: &str) -> Result<WebSocketHandle<'a>, JsError>;
}

impl<'a> ScopeUseWebSocket<'a> for Scope<'a> {
    fn use_web_socket(&'a self, url: &str) -> Result<WebSocketHandle<'a>, JsError> {
        let state = self.create_signal(websocket::State::Closed);
        let message = self.create_signal(String::new());
        let message_bytes = self.create_signal(Vec::new());

        let ws = WebSocket::open(url)?;
        let (write, mut read) = ws.split();

        self.spawn_local(async move {
            while let Some(next) = read.next().await {
                match next {
                    Ok(m) => match m {
                        Message::Text(t) => message.set(t),
                        Message::Bytes(b) => message_bytes.set(b),
                    },
                    Err(_) => {}
                }
            }
        });

        Ok(WebSocketHandle {
            ws_write: RefCell::new(write),
            state,
            message,
            message_bytes,
        })
    }
}

impl<'a> WebSocketHandle<'a> {
    pub async fn send(&self, message: Message) -> Result<(), WebSocketError> {
        self.ws_write.borrow_mut().send(message).await
    }

    pub fn state(&'a self) -> &'a ReadSignal<websocket::State> {
        self.state
    }

    pub fn message(&'a self) -> &'a ReadSignal<String> {
        self.message
    }

    pub fn message_bytes(&'a self) -> &'a ReadSignal<Vec<u8>> {
        self.message_bytes
    }

    /// NOTE: Not yet implemented due to technical reasons.
    pub fn close(&self) {
        unimplemented!();
    }
}
