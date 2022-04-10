use sycamore::prelude::*;

/// A hook that sets the title of the page to the given string and restores the previous title
/// when the scope is destroyed.
pub fn use_title(cx: Scope, title: &str) {
    let prev = gloo::utils::document().title();

    gloo::utils::document().set_title(title);

    on_cleanup(cx, move || {
        gloo::utils::document().set_title(&prev);
    });
}
