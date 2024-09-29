use leptos::*;

#[component]
pub fn ActionView(
    number: i32
) -> impl IntoView {
    match number {
        -1 => view! {<div class="action-icon">[reaction]</div>},
        1 => view! {<div class="action-icon">[one-action]</div>},
        2 => view! {<div class="action-icon">[two-actions]</div>},
        3 => view! {<div class="action-icon">[three-actions]</div>},
        _ => view! {<div>{format!("Cannot Load action view with number {number}")}</div>}
    }
}