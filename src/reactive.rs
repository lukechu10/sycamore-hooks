use async_trait::async_trait;
use futures::channel::oneshot;
use sycamore::prelude::*;

/// Extension trait for [`Scope`] providing reactivity utilites.
#[async_trait(?Send)]
pub trait ScopeReactiveExt<'a> {
    async fn until(&'a self, f: impl FnMut() -> bool + 'a);
}

#[async_trait(?Send)]
impl<'a> ScopeReactiveExt<'a> for Scope<'a> {
    async fn until(&'a self, mut f: impl FnMut() -> bool + 'a) {
        let (rx, tx) = oneshot::channel();
        let mut rx = Some(rx);

        self.create_effect(move || {
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
}
