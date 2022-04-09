use sycamore::prelude::*;

/// Create a simple toggle signal.
pub fn use_toggle_bool(cx: Scope, initial: bool) -> (&ReadSignal<bool>, impl Fn() + '_) {
    let state = create_signal(cx, initial);

    (state, move || state.set(!*state.get()))
}

/// Create a state that is toggled between two possible values.
/// TODO: Remove `Clone` requirement on `T`.
pub fn use_toggle<T: Clone>(cx: Scope, initial: T, other: T) -> (&ReadSignal<T>, impl Fn() + '_) {
    let (toggle, update) = use_toggle_bool(cx, true);
    let state = create_memo(cx, move || {
        if *toggle.get() {
            initial.clone()
        } else {
            other.clone()
        }
    });

    (state, update)
}
