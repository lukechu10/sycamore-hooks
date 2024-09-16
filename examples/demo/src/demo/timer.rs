use std::time::Duration;

use js_sys::Date;
use sycamore::prelude::*;
use sycamore_hooks::timer::create_polled;

#[component]
pub fn CreatePolled() -> View {
    let time = create_polled(Date::new_0, Duration::from_millis(1000));

    view! {
        h2 { "create_polled" }

        p {
            "The current time is: " (time.get_clone().to_string().as_string().unwrap())
        }
    }
}
