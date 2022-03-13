use futures::channel::oneshot;
use sycamore::prelude::*;

pub async fn until<'a>(cx: Scope<'a>, mut f: impl FnMut() -> bool + 'a) {
    let (rx, tx) = oneshot::channel();
    let mut rx = Some(rx);

    cx.create_effect(move || {
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
