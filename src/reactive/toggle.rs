use sycamore::prelude::*;

/// Create a simple toggle signal.
pub fn create_toggle_bool(cx: Scope, initial: bool) -> (&ReadSignal<bool>, impl Fn() + Copy + '_) {
    let state = create_signal(cx, initial);

    (state, || state.set(!*state.get()))
}

/// Create a state that is toggled between two possible values.
/// TODO: Remove `Clone` requirement on `T`.
pub fn create_toggle<T: Clone>(
    cx: Scope,
    initial: T,
    other: T,
) -> (&ReadSignal<T>, impl Fn() + Copy + '_) {
    let (toggle, update) = create_toggle_bool(cx, true);
    let state = create_memo(cx, move || {
        if *toggle.get() {
            initial.clone()
        } else {
            other.clone()
        }
    });

    (state, update)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_toggle() {
        create_scope_immediate(|cx| {
            let (state, update) = create_toggle_bool(cx, true);
            assert!(*state.get());

            update();
            assert!(!*state.get());
        });
    }

    #[test]
    fn test_toggle() {
        create_scope_immediate(|cx| {
            let (state, update) = create_toggle(cx, "hello", "bonjour");
            assert_eq!(*state.get(), "hello");

            update();
            assert_eq!(*state.get(), "bonjour");
        });
    }
}
