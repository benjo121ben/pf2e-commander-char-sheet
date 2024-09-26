use crate::char_data::character::*;
use crate::char_data::proficiency::ProficiencyLevel;
use crate::char_data::tactics::Tactic;
use super::stats_views::TraitView;
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
                        <div class="flex-col smaller-gap align-flex-start bright-bg" on:click=move |_| collapse.update(|c| *c = !*c)>
                            <div class="flex-row">
                                <h4>{
                                    let actions = tactic.actions; 
                                    move || String::from(format!("{tac_name}   {actions}"))
                                }</h4>
                                <input type="checkbox" style="width:fit-content" 
                                    prop:checked= move || tactic.selected
                                    on:change={
                                        let tac_name2 = tactic.name.clone();
                                        move |_| character_write.update(|c|{
                                            let mut_ref: &mut Tactic = c.tactics.iter_mut().find(|val| val.name == tac_name2).unwrap(); 
                                            mut_ref.selected = !mut_ref.selected
                                        })
                                    }
                                />
                            </div>
                            <Show when=move || collapse.get()>
                                <TraitView trait_names=tactic.traits.clone()/>
                                <div class="tiny-text" inner_html={let desc = tactic.description.clone(); move || desc.clone()}></div>
                            </Show>
                        </div>
                    }.into_view()
                }
            />
        </div>
    }
}