//! Hooks for working with `setTimeout` and `setInterval`.

use futures::StreamExt;
use gloo_timers::future::{IntervalStream, TimeoutFuture};
use std::time::Duration;
use sycamore::{futures::spawn_local_scoped, prelude::*};

/// Creates a new timeout. If the scope is destroyed before the timeout is executed, the timeout is canceled automatically.
pub fn create_timeout(f: impl FnOnce() + 'static, delay: Duration) {
    spawn_local_scoped(async move {
        TimeoutFuture::new(
            delay
                .as_millis()
                .try_into()
                .expect("could not convert delay into an u32"),
        )
        .await;
        f();
    });
}

/// Creates a new interval. The interval is cancelled automatically when the scope is destroyed.
pub fn create_interval(mut f: impl FnMut() + 'static, delay: Duration) {
    spawn_local_scoped(async move {
        while IntervalStream::new(
            delay
                .as_millis()
                .try_into()
                .expect("could not convert delay into an u32"),
        )
        .next()
        .await
        .is_some()
        {
            f();
        }
    });
}

/// Periodically polls a function. The function is called at least once when the poll is created.
pub fn create_polled<T>(
    mut f: impl FnMut() -> T + 'static,
    delay: Duration,
) -> ReadSignal<T> {
    let state = create_signal(f());
    create_interval(move || state.set(f()), delay);
    *state
}
