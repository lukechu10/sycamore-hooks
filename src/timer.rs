//! Hooks for working with `setTimeout` and `setInterval`.

use futures::StreamExt;
use gloo::timers::future::{IntervalStream, TimeoutFuture};
use std::time::Duration;
use sycamore::{futures::spawn_local_scoped, prelude::*};

/// Creates a new timeout. If the scope is destroyed before the timeout is executed, the timeout is canceled automatically.
pub fn create_timeout<'a>(cx: Scope<'a>, f: impl FnOnce() + 'a, delay: Duration) {
    spawn_local_scoped(cx, async move {
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
pub fn create_interval<'a>(cx: Scope<'a>, mut f: impl FnMut() + 'a, delay: Duration) {
    spawn_local_scoped(cx, async move {
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
pub fn create_polled<'a, T>(
    cx: Scope<'a>,
    mut f: impl FnMut() -> T + 'a,
    delay: Duration,
) -> &'a ReadSignal<T> {
    let state = create_signal(cx, f());
    create_interval(cx, move || state.set(f()), delay);
    state
}
