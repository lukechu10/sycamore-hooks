use futures::channel::oneshot;
use sycamore::prelude::*;

mod toggle;
pub use toggle::*;

/// Returns a future that resolves when the given function returns `true`. The condition is checked every time
/// a tracked state is updated.
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

/// Create a simple counter signal. Returns the reactive signal, an increment callback, and a decrement callback.
pub fn create_counter(
    cx: Scope,
    initial: i32,
) -> (
    &ReadSignal<i32>,
    impl Fn() + Copy + '_,
    impl Fn() + Copy + '_,
) {
    let counter = create_signal(cx, initial);

    (
        counter,
        || counter.set(*counter.get() + 1),
        || counter.set(*counter.get() - 1),
    )
}
