use std::collections::HashMap;

use leptos::*;
use leptos::logging::*;
use server_fn::codec::IntoRes;
use super::view_helpers::{get_base_context, get_modal_context};
use crate::char_data::{character::Character, conditions::{ConditionData, FullConditionView}};

#[component]
pub fn ConditionSection() -> impl IntoView {
    let (read_char, _) = get_base_context("ConditionSection");
    let conditions_map: HashMap<String, ConditionData> = use_context().expect("ConditionsSection expected conditiondata to be ready");
    let get_char_conditions = move || {
        match read_char.with(|c| c.get_all_conditions(&conditions_map)) {
            Ok(condition_list) => condition_list.clone(),
            Err(error) => {log!("ConditionSection: error getting character conditions: {error}"); vec![]}
        }
    };
    let condition_memo = create_memo(move |_| get_char_conditions());
    let add_cond_visible_signal = create_rw_signal(false);
    create_effect(move |_| {let _ = condition_memo.get(); log!("cond_view_updated")});
    view!{
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
            <img alt="test" src="icons/add.svg" style="width: 20px; height:20px;"
                    on:click={move|_| add_cond_visible_signal.set(true)}
                />
        </div>
        <Show
            when=move||add_cond_visible_signal.get()
        >
            <ConditionSelectView add_cond_visible_signal=add_cond_visible_signal/>
        </Show>
    }
}

#[component]
pub fn ConditionView(condition: FullConditionView) -> impl IntoView {
    let (get_char, set_char) = get_base_context("ConditionSection");
    let modal_data = get_modal_context("ConditionsSection");
    let conditions_map: HashMap<String, ConditionData> = use_context().expect("ConditionsSection expected conditiondata to be ready");
    let cond_data = conditions_map.get(&condition.name).expect("ConditionsSection, expecting condition to exist in map").clone();
    let name = condition.name.clone();

    let current_state_memo: Memo<FullConditionView> = create_memo(move |_| {
        let cond_name_clone = name.clone();
        log!("update called to {}", cond_name_clone); 
        match get_char.with(|c| c.get_all_conditions(&conditions_map)){
            Ok(conditions_vec) => {
                conditions_vec.iter().find(|cond| (**cond).name == cond_name_clone).cloned().expect("condition view expects condition to exist")
            },
            Err(e) => {
                let message = format!("Error with getting condition data: {}", e);
                log!("{}", message);
                panic!("{}", message);
            },
        }
    });
    let get_current_cond_view_state = move || current_state_memo.get();

    let change_level_and_delete_if_zero = move |modifier: i32| {
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
            }
        });
    };


    let open_condition_data_modal = move || {
        let cond_data_clone = cond_data.clone();
        modal_data.update(|data| {
            data.visible = true;
            data.title = cond_data_clone.name;
            data.description = cond_data_clone.description;
        });
    };

    let change_activate = move || {
        let cond = get_current_cond_view_state();
        if cond.forced {
            return;
        }
        set_char.update(|c|{
            c.conditions.iter_mut().for_each(|f_cond| {
                if f_cond.name == cond.name {
                    f_cond.active = !f_cond.active;
                }
            });
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
            <Show when=move||get_current_cond_view_state().level.is_some()>
                <label class="no-grow no-margins">
                    {move || get_current_cond_view_state().level.unwrap()}
                </label>
            </Show> 
            
            <Show when=move||get_current_cond_view_state().forced>
                <img alt="disabled" src="icons/lock.svg" style="width: 20px; height:20px;"/>
            </Show>
            <Show when=move||{let val: FullConditionView = get_current_cond_view_state(); !val.forced}>
                <img alt="decr" src="icons/remove.svg" style="width: 20px; height:20px;"
                    on:click=move|ev| {ev.stop_propagation(); change_level_and_delete_if_zero(-1)}
                /> 
            </Show>
            <Show when=move||{let val: FullConditionView = get_current_cond_view_state(); !val.forced && val.level.is_some()}>
                <img alt="incr" src="icons/add.svg" style="width: 20px; height:20px;"
                    on:click=move|ev| {ev.stop_propagation(); change_level_and_delete_if_zero(1)}
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
        <div class="modal" style="justify-content: flex-end;" on:click=move |_| add_cond_visible_signal.set(false)>
            <div class="condition-modal">
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