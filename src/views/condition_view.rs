
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
    view!{
        <For
            each=move || get_active_conditions()
            key=move |val| val.name.clone() 
            children = move |condition| {
                let cond_clone = condition.clone();
                let ret = view!{
                    <h3 style="no-grow">
                    {move || cond_clone.name.clone()} {
                        move || match cond_clone.level.clone() {
                            Some(value) => format!("{value}"),
                            None => format!(""),
                        }
                    }
                    </h3>
                    <label>forced: {move || cond_clone.forced} active: {move || cond_clone.active}</label>
                };
                ret
            }
        />
    }
}