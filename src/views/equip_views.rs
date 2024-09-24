use crate::char_data::character::*;
use crate::char_data::proficiency::ProficiencyLevel;
use ev::Event;
use leptos::*;
use leptos::logging::log;



#[component]
pub fn WeaponView() -> impl IntoView {
    view! {
        TEST
    }
}

#[component]
pub fn TacticsView() -> impl IntoView {
    let character_data = use_context::<ReadSignal<Character>>().expect("TacticsView: Character should be set at this point");
    view! {
        <For
            each=move ||character_data.with(|k| k.tactics.clone())
            key=|tactic| tactic.name.clone()
            children=move |tactic| {
                view! {
                    <div class="flex-col">
                        <h4>{move || tactic.name.clone()}</h4>
                        <div class="flex-row">{tactic.traits.clone().into_iter().map(|t| view!{<div>{t}</div>}).collect::<Vec<_>>()}</div>
                        <div>{tactic.description.clone()}</div>
                    </div>
                }.into_view()
            }
        />
    }
}