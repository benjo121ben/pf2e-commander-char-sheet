use crate::char_data::character::*;
use crate::char_data::proficiency::ProficiencyLevel;
use crate::char_data::tactics::Tactic;
use crate::views::action_view::ActionView;
use super::stats_views::TraitView;
use ev::Event;
use leptos::*;
use leptos::logging::log;

#[component]
pub fn EquipView() -> impl IntoView {
}

#[component]
pub fn WeaponView() -> impl IntoView {
    let character_data = use_context::<ReadSignal<Character>>().expect("WeaponView: Character should be set at this point");
    view! {
        TEST
    }
}

#[component]
pub fn TacticsView() -> impl IntoView {
    let character_write = use_context::<WriteSignal<Character>>().expect("TacticsView: Character should be set at this point");
    let character_data = use_context::<ReadSignal<Character>>().expect("TacticsView: Character should be set at this point");
    let max_tactics = 2;
    let count_tactics = {
        move || character_data.with(|c| {
            c.tactics.iter().filter(|tactic| tactic.selected).count()
        })
    };
    let tactics_header = move || {
        let count = count_tactics();
        String::from(format!("Tactics [{count} / {max_tactics}]"))
    };
    view! {
        <div>
            <h4>{tactics_header}</h4>
            <div class="flex-col no-grow" style="gap:10px">
                <For
                    each=move ||character_data.with(|k| k.tactics.clone())
                    key=|tactic| tactic.name.clone()
                    children=move |tactic| {
                        let tac_name = tactic.name.clone();
                        let collapse = create_rw_signal(false);
                        let get_selected_on_tactic = {
                            let tac_name2 = tactic.name.clone();
                            move || character_data.with(|c|{
                                c.tactics.iter().find(|val| val.name == tac_name2).unwrap().selected
                            })
                        };
                        let update_selected_on_tactic = {
                            let tac_name2 = tactic.name.clone();
                            let get_selected_on_tactic_clone = get_selected_on_tactic.clone();
                            move |_| {
                                if count_tactics() >= max_tactics && !get_selected_on_tactic_clone() {
                                    return;
                                }
                                character_write.update(|c|{
                                    let mut_ref: &mut Tactic = c.tactics.iter_mut().find(|val| val.name == tac_name2).unwrap(); 
                                    mut_ref.selected = !mut_ref.selected
                                });
                            }
                        };
                        view! {
                            <div class="flex-col align-flex-start bright-bg" 
                                on:click=move |_| collapse.update(|c| *c = !*c)
                                on:contextmenu=update_selected_on_tactic
                                class:selected-tactic=get_selected_on_tactic
                            >
                                <div class="flex-row feat-title-row">
                                    <h4>{
                                        move || String::from(format!("{tac_name}"))
                                    }</h4>
                                    <Show when=move || tactic.actions != 0>
                                        <ActionView number=tactic.actions/>
                                    </Show>
                                </div>
                                <Show when=move || collapse.get()>
                                    <TraitView trait_names=tactic.traits.clone()/>
                                    <hr/>
                                    <div class="tiny-text" inner_html={let desc = tactic.description.clone(); move || desc.clone()}></div>
                                </Show>
                            </div>
                        }.into_view()
                    }
                />
            </div>
        </div>
    }
}