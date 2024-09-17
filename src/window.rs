use sycamore::prelude::*;

/// A hook that sets the title of the page to the given string and restores the previous title
/// when the scope is destroyed.
pub fn use_title(title: &str) {
    let prev = document().title();

    document().set_title(title);

    on_cleanup(move || {
        document().set_title(&prev);
    });
}

/// A side-effect hook that sets the document in full-screen mode when `active` is `true`.
pub fn use_full_screen(active: ReadSignal<bool>) {
    create_effect(move || {
        let fs_enabled = document().fullscreen_element().is_some();
        if active.get() && !fs_enabled {
            document().document_element().unwrap().request_fullscreen().unwrap();
        } else if fs_enabled {
            document().exit_fullscreen();
        }
    });
}
