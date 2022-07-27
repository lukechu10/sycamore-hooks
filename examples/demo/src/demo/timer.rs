use std::time::Duration;

use js_sys::Date;
use sycamore::prelude::*;
use sycamore_hooks::timer::create_polled;

#[component]
pub fn CreatePolled<G: Html>(cx: Scope) -> View<G> {
    let time = create_polled(cx, Date::new_0, Duration::from_millis(1000));

    view! { cx,
        h2 { "create_polled" }

        p {
            "The current time is: " (time.get().to_string())
        }
    }
}
