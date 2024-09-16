use futures::channel::oneshot;
use sycamore::prelude::*;

mod toggle;
pub use toggle::*;

/// Returns a future that resolves when the given function returns `true`. The condition is checked every time
/// a tracked state is updated.
pub async fn until(mut f: impl FnMut() -> bool + 'static) {
    let (rx, tx) = oneshot::channel();
    let mut rx = Some(rx);

    create_effect(move || {
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
    initial: i32,
) -> (
    ReadSignal<i32>,
    impl Fn() + Copy + 'static,
    impl Fn() + Copy + 'static,
) {
    let counter = create_signal(initial);

    (
        *counter,
        move || counter.set(counter.get() + 1),
        move || counter.set(counter.get() - 1),
    )
}
