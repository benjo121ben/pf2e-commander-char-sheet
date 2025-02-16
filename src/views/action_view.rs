use leptos::prelude::*;

#[component]
pub fn ActionView(
    number: i32
) -> impl IntoView {
    match number {
        -1 => view! {<div class="action-icon">[reaction]</div>}.into_any(),
        1 => view! {<div class="action-icon">[one-action]</div>}.into_any(),
        2 => view! {<div class="action-icon">[two-actions]</div>}.into_any(),
        3 => view! {<div class="action-icon">[three-actions]</div>}.into_any(),
        _ => view! {<div>{format!("Cannot Load action view with number {number}")}</div>}.into_any()
    }
}