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
        <div class="flex-col">
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
    let (_, set_char) = get_base_context("ConditionSection");
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

    view!{
        <div class="flex-row">
            <input type="checkbox" prop:checked=move|| cond_clone.active prop:disabled=move||cond_clone.forced on:change=move|_| change_activate(&cond_clone.name)/>
            <h3 style="no-grow" title=condition.condition_data.description.clone()>
            {move || name.clone()} {
                move || match cond_clone.level {
                    Some(value) => format!(" {value}"),
                    None => format!(""),
                }
            }
            </h3>
            <Show when=move||cond_clone.forced>
                <img alt="disabled" src="icons/lock.svg" style="width: 20px; height:20px;"/>
            </Show> 
            <Show when=move||!cond_clone.forced>
                <img alt="test" src="icons/add.svg" style="width: 20px; height:20px;"/>
                <img alt="test" src="icons/remove.svg" style="width: 20px; height:20px;"/>
            </Show> 
        </div>
    }.into_view()
}