use std::collections::HashMap;

use leptos::*;
use super::view_helpers::{get_base_context, get_modal_context};
use crate::{
    char_data::{
        character::Character, 
        conditions::{
            ConditionData, 
            FullConditionView
        }
    }, 
    views::view_helpers::get_all_conditions_vector_memo_from_context
};

#[component]
pub fn ConditionSection() -> impl IntoView {
    let condition_memo = get_all_conditions_vector_memo_from_context("ConditionSection");
    let add_cond_visible_signal = create_rw_signal(false);
    view!{
        <section id="condition_section">
            <div class="condition-div">
                <For
                    each=move|| condition_memo.get().clone()
                    key=move |val| val.name.clone() 
                    children = move |condition| {
                        let cond_clone = condition.clone();
                        view! {
                            <ConditionView condition=cond_clone/>
                        }
                    }
                />
                <img alt="image-not-working" src="icons/add.svg" class="icon"
                        on:click={move|_| add_cond_visible_signal.set(true)}
                    />
            </div>
            <Show
                when=move||add_cond_visible_signal.get()
            >
                <ConditionSelectView add_cond_visible_signal=add_cond_visible_signal/>
            </Show>
        </section>
    }
}

#[component]
pub fn ConditionView(condition: FullConditionView) -> impl IntoView {
    let (get_char, set_char) = get_base_context("ConditionSection");
    let modal_data = get_modal_context("ConditionsSection");
    let conditions_map: HashMap<String, ConditionData> = use_context().expect("ConditionsSection expected conditiondata to be ready");
    let name = condition.name.clone();

    let current_state_memo: Memo<FullConditionView> = create_memo({
        let cond_map_clone = conditions_map.clone();
        move |_| {
            let cond_name_clone = name.clone();
            match get_char.with(|c| c.get_all_conditions(&cond_map_clone)){
                Ok(conditions_vec) => {
                    conditions_vec.iter().find(|cond| (**cond).name == cond_name_clone).cloned().expect("condition view expects condition to exist")
                },
                Err(e) => {
                    let message = format!("Error with getting condition data: {}", e);
                    panic!("{}", message);
                },
            }
        }
    });

    let get_current_cond_view_state = move || current_state_memo.get();

    let handle_sub_conditions_on_remove = move |full_view: &FullConditionView, conditions_map_clone: &HashMap<String, ConditionData>, character: &mut Character| {
        for condition_to_add_name in  full_view.condition_data.added_on_remove.clone() {
            character.add_condition(&conditions_map_clone, &condition_to_add_name, true);
        }
    };

    let change_level_and_delete_if_zero = {
        let conditions_map_clone = conditions_map.clone();
        move |modifier: i32| {
            let cond_view = get_current_cond_view_state();
            let cond_name = cond_view.name.clone();
            if cond_view.forced {
                return;
            }
            let mut delete = false;
            set_char.update(|c|{
                c.conditions.iter_mut().for_each(|f_cond| {
                    if f_cond.name != cond_view.name {
                        return;
                    }
                    match f_cond.level {
                        Some(level) => {
                            let new_val = level + modifier;
                            if new_val == 0 {
                                delete = true;
                            }
                            f_cond.level = Some(new_val);
                        },
                        None => {delete = true; },//this is the way to remove conditions that dont have a level
                    }
                });
                if delete {
                    c.remove_condition(&cond_name);
                    if cond_view.active { //add on-remove conditions like dying
                        handle_sub_conditions_on_remove(&cond_view, &conditions_map_clone, c);
                    }
                }
            });
        }
    };

    //we have to clone this, because they are passed into different show contexts, which means we require this data twice
    let change_level_and_delete_if_zero_clone = change_level_and_delete_if_zero.clone();


    let open_condition_data_modal = move || {
        let cond_data_clone = condition.condition_data.clone();
        modal_data.update(|data| {
            data.reset();
            data.title = cond_data_clone.name;
            data.description = cond_data_clone.description;
            data.show();
        });
    };

    let change_activate = move || {
        let cond = get_current_cond_view_state();
        let conditions_map_clone = conditions_map.clone();
        if cond.forced {
            return;
        }
        set_char.update(|c| {
            let current_condition = c.conditions.iter_mut().find(|f_cond| f_cond.name == cond.name);
            if current_condition.is_none() {
                return;
            }

            let char_condition = current_condition.unwrap();
            if !char_condition.active {
                //we add the condition again so that all normal effects on gain are triggered 
                //and conditions like dying always start at the right level
                c.add_condition(&conditions_map_clone, &cond.name, false);
                return;
            }

            char_condition.active = !char_condition.active;
            if !char_condition.active {
                handle_sub_conditions_on_remove(&cond, &conditions_map_clone, c); //trigger remove logic when deactivating condition
            }
        }); 
    };

    view!{
        <div class="condition" 
                title=move || get_current_cond_view_state().condition_data.description.clone() 
                class:selected-condition=move ||get_current_cond_view_state().active
                on:click=move|_|open_condition_data_modal()
                on:contextmenu=move|_|change_activate()>
            <label class="no-grow no-margins">
                {move || get_current_cond_view_state().name.clone()} 
            </label>
            <Show when=move||{let state = get_current_cond_view_state(); state.level.is_some() && state.active}>
                <label class="no-grow no-margins">
                    {move || get_current_cond_view_state().level.unwrap()}
                </label>
            </Show> 
            
            <Show when=move||get_current_cond_view_state().forced>
                <img alt="disabled" src="icons/lock.svg" class="icon"/>
            </Show>
            <Show when=move||{let val: FullConditionView = get_current_cond_view_state(); !val.forced}>
                <img alt="decr" src="icons/remove.svg" class="icon"
                    on:click={
                        let change_lvl = change_level_and_delete_if_zero.clone();
                        move|ev| {ev.stop_propagation(); change_lvl(-1)}
                    }
                /> 
            </Show>
            <Show when=move||{let val: FullConditionView = get_current_cond_view_state(); !val.forced && val.active && val.level.is_some()}>
                <img alt="incr" src="icons/add.svg" class="icon"
                    on:click={
                        let change_lvl = change_level_and_delete_if_zero_clone.clone();
                        move|ev| {ev.stop_propagation(); change_lvl(1)}
                    }
                />
            </Show> 
        </div>
    }.into_view()
}


#[component]
pub fn ConditionSelectView(add_cond_visible_signal: RwSignal<bool>) -> impl IntoView {
    let (_, set_char) = get_base_context("ConditionSelectView");
    let conditions_map: HashMap<String, ConditionData> = use_context().expect("ConditionSelectView expected conditiondata to be ready");
    let cond_map_clone = conditions_map.clone();

    let get_condition_list = move |conditions_map: &HashMap<String, ConditionData>| {
        let mut keys: Vec<String> = conditions_map.keys().cloned().collect();
        keys.sort();
        return keys;
    };
    
    let add_condition = move|name: &str, conditions_map: &HashMap<String, ConditionData>| {
        set_char.update(|c: &mut Character| {
            c.add_condition(conditions_map, name, false);
        })
    };
    
    view! {
        <div class="condition-modal" style="justify-content: flex-end;" on:click=move |_| add_cond_visible_signal.set(false)>
            <div class="condition-modal-content">
                <For each=move|| get_condition_list(&conditions_map)
                    key=move|val|val.clone()
                    children=move |name| {
                        let nameclone = name.clone();
                        let conditions_map_clone = cond_map_clone.clone();
                        view!{
                            <label class="bright-bg"
                                on:click=move |_| add_condition(&name, &conditions_map_clone)
                            >
                                {move || nameclone.clone()}
                            </label>
                        }
                    }
                />
            </div>
        </div>
    }
}