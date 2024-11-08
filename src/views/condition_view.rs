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
    let check_icon = move |condition: FullConditionView| {
        if condition.forced {
            "lock.svg"
        }
        else if condition.active {
            "./icons/add.svg"
        }
        else {
            "./icons/remove.svg"
        }
    };
    view!{
        <For
            each=move || get_active_conditions()
            key=move |val| val.name.clone() 
            children = move |condition| {
                let cond_clone = condition.clone();
                let name = cond_clone.name.clone();
                view!{
                    <h3 style="no-grow">
                    {move || name.clone()} {
                        move || match cond_clone.level {
                            Some(value) => format!("{value}"),
                            None => format!(""),
                        }
                    }
                    </h3>
                    <i alt="test" src="icons/lock.svg"/>
                }
            }
        />
    }
}