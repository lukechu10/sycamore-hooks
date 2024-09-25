//! Useful math utilities for Sycamore.

use std::cmp::Ordering;

use sycamore::prelude::*;

macro_rules! use_math {
    ($(#[$attr:meta])* $name:ident, $function:path) => {
        $(#[$attr])*
        pub fn $name(value: impl Accessor<f64> + 'static) -> ReadSignal<f64> {
            create_memo(move || $function(value.value()))
        }
    };
}

use_math! {
    /// Reactive version of [`f64::abs`].
    use_abs, f64::abs
}

use_math! {
    /// Reactive version of [`f64::floor`].
    use_floor, f64::floor
}

use_math! {
    /// Reactive version of [`f64::ceil`].
    use_ceil, f64::ceil
}

use_math! {
    /// Reactive version of [`f64::round`].
    use_round, f64::round
}

use_math! {
    /// Reactive version of [`f64::round_ties_even`].
    use_round_ties_even, f64::round_ties_even
}

use_math! {
    /// Reactive version of [`f64::trunc`].
    use_trunc, f64::trunc
}

use_math! {
    /// Reactive version of [`f64::fract`].
    use_fract, f64::fract
}

use_math! {
    /// Reactive version of [`f64::signum`].
    use_signum, f64::signum
}

/// Reactively get the max value in a list of values.
///
/// # Example
///
/// ```
/// use sycamore::prelude::*;
/// use sycamore_hooks::math::use_max;
///
/// # let _ = create_root(|| {
/// let list = create_signal(vec![4, 5, 3, 1, 2]);
/// let max = use_max::<i32, Vec<i32>>(list);
/// assert_eq!(max.get(), Some(5));
///
/// list.set(vec![7, 8, 6, 10, 9]);
/// assert_eq!(max.get(), Some(10));
/// # });
/// ```
pub fn use_max<T: PartialOrd, U: IntoIterator<Item = T> + 'static>(
    values: impl Accessor<U> + 'static,
) -> ReadSignal<Option<T>> {
    create_memo(move || {
        let mut values = values.value().into_iter();
        let first = values.next()?;
        values.try_fold(first, |acc, x| match acc.partial_cmp(&x)? {
            Ordering::Less => Some(x),
            _ => Some(acc),
        })
    })
}

/// Reactively get the min value in a list of values.
///
/// # Example
///
/// ```
/// use sycamore::prelude::*;
/// use sycamore_hooks::math::use_min;
///
/// # let _ = create_root(|| {
/// let list = create_signal(vec![4, 5, 3, 1, 2]);
/// let min = use_min::<i32, Vec<i32>>(list);
/// assert_eq!(min.get(), Some(1));
///
/// list.set(vec![7, 8, 6, 10, 9]);
/// assert_eq!(min.get(), Some(6));
/// # });
/// ```
pub fn use_min<T: PartialOrd, U: IntoIterator<Item = T> + 'static>(
    values: impl Accessor<U> + 'static,
) -> ReadSignal<Option<T>> {
    create_memo(move || {
        let mut values = values.value().into_iter();
        let first = values.next()?;
        values.try_fold(first, |acc, x| match acc.partial_cmp(&x)? {
            Ordering::Less => Some(acc),
            _ => Some(x),
        })
    })
}

/// Reactive logical AND.
pub fn use_and(
    x: impl Accessor<bool> + 'static,
    y: impl Accessor<bool> + 'static,
) -> ReadSignal<bool> {
    create_memo(move || x.value() && y.value())
}

/// Reactive logical OR.
pub fn use_or(
    x: impl Accessor<bool> + 'static,
    y: impl Accessor<bool> + 'static,
) -> ReadSignal<bool> {
    create_memo(move || x.value() || y.value())
}

/// Reactive logical NOT.
pub fn use_not(x: impl Accessor<bool> + 'static) -> ReadSignal<bool> {
    create_memo(move || !x.value())
}

/// Reactive logical XOR.
pub fn use_xor(
    x: impl Accessor<bool> + 'static,
    y: impl Accessor<bool> + 'static,
) -> ReadSignal<bool> {
    create_memo(move || x.value() ^ y.value())
}
