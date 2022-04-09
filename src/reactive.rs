use futures::channel::oneshot;
use sycamore::prelude::*;

mod toggle;
pub use toggle::*;

pub async fn until<'a>(cx: Scope<'a>, mut f: impl FnMut() -> bool + 'a) {
    let (rx, tx) = oneshot::channel();
    let mut rx = Some(rx);

    create_effect(cx, move || {
        if let Some(rx) = rx.take() {
            if f() {
                // Rationale: `tx` is not dropped until `rx` is sent for the first time,
                // after which it can no longer be used.
                rx.send(()).unwrap();
            }
        }
    });

    // Rationale: `rx` is owned by the create_effect
    tx.await.unwrap();
}
