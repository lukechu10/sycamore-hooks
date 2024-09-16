use sycamore::prelude::*;

/// Create a simple toggle signal.
pub fn create_toggle_bool( initial: bool) -> (ReadSignal<bool>, impl Fn() + Copy + 'static) {
    let state = create_signal(initial);

    (*state, move || state.set(!state.get()))
}

/// Create a state that is toggled between two possible values.
/// TODO: Remove `Clone` requirement on `T`.
pub fn create_toggle<T: Clone>(
    initial: T,
    other: T,
) -> (ReadSignal<T>, impl Fn() + Copy + 'static) {
    let (toggle, update) = create_toggle_bool(true);
    let state = create_memo(move || {
        if toggle.get() {
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
        let _ = create_root(|| {
            let (state, update) = create_toggle_bool(true);
            assert!(state.get());

            update();
            assert!(!state.get());
        });
    }

    #[test]
    fn test_toggle() {
        let _ = create_root(|| {
            let (state, update) = create_toggle("hello", "bonjour");
            assert_eq!(state.get(), "hello");

            update();
            assert_eq!(state.get(), "bonjour");
        });
    }
}
