use std::collections::HashMap;

use leptos::*;
use leptos::logging::*;
use super::view_helpers::get_base_context;
use crate::char_data::conditions::{ConditionData, FullConditionView};


#[component]
pub fn ConditionSection() -> impl IntoView {
    let (read_char, _) = get_base_context("ConditionSection");
    let conditions_map: HashMap<String, ConditionData> = use_context().expect("ConditionsSection expected conditiondata to be ready");
    let get_active_conditions = move || {
        match read_char.with(|c| c.get_all_conditions(&conditions_map)) {
            Ok(condition_list) => condition_list.clone(),
            Err(error) => {log!("ConditionSection: error getting character conditions: {error}"); vec![]}
        }
    };
    let condition_memo = create_memo(move |_| get_active_conditions());
    view!{
        <div class="condition-div">
            <For
                each=move|| condition_memo.get().clone()
                key=move |val| val.name.clone() 
                children = move |condition| {
                    log!("{}", condition.name);
                    let cond_clone = condition.clone();
                    view! {
                        <ConditionView condition=cond_clone/>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn ConditionView(condition: FullConditionView) -> impl IntoView {
    let (get_char, set_char) = get_base_context("ConditionSection");
    let conditions_map: HashMap<String, ConditionData> = use_context().expect("ConditionsSection expected conditiondata to be ready");
    let cond_clone = condition.clone();
    let name = cond_clone.name.clone();
    let change_activate = move |cond_name: &str| {
        set_char.update(|c|{
            c.conditions.iter_mut().for_each(|f_cond| {
                if f_cond.name == cond_name {
                    f_cond.active = !f_cond.active;
                }
            });
        });
    };
    let change_level = move |cond_name: &str, modifier: i32| {
        set_char.update(|c|{
            c.conditions.iter_mut().for_each(|f_cond| {
                if f_cond.name != cond_name {
                    return;
                }
                match f_cond.level {
                    Some(level) => {f_cond.level = Some(std::cmp::max(level + modifier, 0));},
                    None => {log!("condition: {cond_name} does not have a level")},
                }
            });
        });
    };
    
    let get_current_cond_state = move|cond_name: &str| {
        match get_char.with(|c| c.get_all_conditions(&conditions_map)){
            Ok(conditions_vec) => conditions_vec.iter().find(|cond| (**cond).name == cond_name).cloned(),
            Err(e) => {log!("Error with getting condition data: {}", e); None},
        }
    };

    let current_state_memo: Memo<FullConditionView> = create_memo(move |_| get_current_cond_state(&condition.name).unwrap());
    let get_memo = move || current_state_memo.get();

    view!{
        <div class="flex-row align-center bright-bg" title=move || get_memo().condition_data.description.clone()>
            <input type="checkbox" prop:checked=move|| get_memo().active prop:disabled=move||get_memo().forced on:change=move|_| change_activate(&get_memo().name)/>
            <label class="no-grow no-margins condition-text">
                {let _name = name.clone(); move || _name.clone()} 
            </label>
            <Show when=move||get_memo().level.is_some()>
                <label class="no-grow no-margins condition-text">
                    {move || get_memo().level.unwrap()}
                </label>
            </Show> 
            
            <Show when=move||get_memo().forced>
                <img alt="disabled" src="icons/lock.svg" style="width: 20px; height:20px;"/>
            </Show> 
            <Show when=move||{let val: FullConditionView = get_memo(); !val.forced && val.level.is_some()}>
                <img alt="test" src="icons/add.svg" style="width: 20px; height:20px;"
                    on:click={let _name =name.clone(); move|_| change_level(&_name, 1)}
                />
                <img alt="test" src="icons/remove.svg" style="width: 20px; height:20px;"
                    on:click={let _name =name.clone(); move|_| change_level(&_name, -1)}
                />
            </Show> 
        </div>
    }.into_view()
}