use crate::char_data::character::*;
use crate::char_data::proficiency::ProficiencyLevel;
use crate::char_data::tactics::Tactic;
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
    let character_write = use_context::<WriteSignal<Character>>().expect("TacticsView: Character should be set at this point");
    let character_data = use_context::<ReadSignal<Character>>().expect("TacticsView: Character should be set at this point");
    view! {
        <div class="flex-col no-grow" style="gap:10px">
            <For
                each=move ||character_data.with(|k| k.tactics.clone())
                key=|tactic| tactic.name.clone()
                children=move |tactic| {
                    let tac_name = tactic.name.clone();
                    let collapse = create_rw_signal(false);
                    view! {
                        <div class="flex-col"  on:click=move |_| collapse.update(|c| *c = !*c)>
                            <h4>{
                                let actions = tactic.actions; 
                                move || String::from(format!("{tac_name}   {actions}"))
                            }</h4>
                                <input type="checkbox" style="width:fit-content" 
                                    prop:checked= move || tactic.selected
                                    on:change={
                                        let tac_name2 = tactic.name.clone();
                                        move |ev| character_write.update(|c|{
                                            let mut_ref: &mut Tactic = c.tactics.iter_mut().find(|val| val.name == tac_name2).unwrap(); 
                                            mut_ref.selected = !mut_ref.selected
                                        })
                                    }
                                />
                            <Show when=move || collapse.get()>
                                <div class="flex-row">{tactic.traits.clone().into_iter().map(|t| view!{<div class="trait">{t}</div>}).collect::<Vec<_>>()}</div>
                                <div inner_html={let desc = tactic.description.clone(); move || desc.clone()}></div>
                            </Show>
                        </div>
                    }.into_view()
                }
            />
        </div>
    }
}