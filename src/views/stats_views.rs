use crate::char_data::character::*;
use crate::char_data::proficiency::ProficiencyLevel;
use crate::char_data::stats::DependentStat;
use leptos::*;
use leptos::logging::log;


#[component]
pub fn MainStatsView(
    read_char: ReadSignal<Character>,
    write_char: WriteSignal<Character>
) -> impl IntoView {
    let update_stat = move |id: String, offset: i32| write_char
    .update(|f| {
        f.attributes.set_stat(&id, f.attributes.get_stat(&id).unwrap().value + offset);
    });
    view! {
        <div style="display: flex; flex-direction: row; gap: 10px">
            <For
                each=move || {
                    read_char.with(|c|
                        c.attributes
                        .as_vec()
                        .into_iter()
                        .map(|f| (String::from(f.get_id()), String::from(f.get_abbr())))
                    )
                }
                key=|(id, _)| id.clone()
                children=move |(id, abbr)| {
                    let idForLeftClick = id.clone();
                    let idForrightClick = id.clone();
                    let val = create_memo(move |_| {
                        let idClone = id.clone();
                        read_char.with(move |cha_obj| cha_obj.attributes.get_stat_val(&idClone).unwrap())
                    });
                    view! { 
                        <div 
                            on:click=move |_| {update_stat(idForLeftClick.clone(), 1)}
                            on:contextmenu=move |_| update_stat(idForrightClick.clone(), -1)
                        >
                            {abbr}: {val}
                        </div> }
                }
            />
        </div>
    }
}


#[component]
pub fn EditMainstatsView(
    read_character: ReadSignal<Character>,
    write_character: WriteSignal<Character>
) -> impl IntoView {
    view! {
        <For
            each=move || {
                read_character.with(|k| k.attributes.as_vec().into_iter().map(|f| f.get_id().to_string()))
            }
            key=|id| id.clone()
            children=move |id| {
                let idForInput = id.clone();
                let trueVal = create_memo(move |_| {
                    let idClone = id.clone();
                    read_character.with(move |k| k.attributes.get_stat_val(&idClone)).unwrap()
                });
                view! {
                    <input
                        type="number"
                        value=trueVal
                        style="width:35px; margin-right: 10px;"
                        on:change=move |event| {
                            write_character
                                .update(|f| {
                                    let val: i32 = event_target_value(&event).parse().unwrap();
                                    f.attributes.set_stat(&idForInput, val);
                                })
                        }
                    />
                }
            }
        />
    }
}


#[component]
pub fn SkillView(
    read_character: ReadSignal<Character>,
    write_character: WriteSignal<Character>
) -> impl IntoView {
    view! {
        <div style="display:flex; flex-direction: column">
        <For
            each=move || {
                read_character.with(|k| k.skills.clone())
            }
            key=|skill| skill.name.clone()
            children=move |skill| {
                let name = skill.name.clone();
                let skill_value = create_memo({move |_| read_character.with(|c| skill.calculate_stat(&c))});

                view! {
                    <div style="display:flex; flex-direction: row">
                        <label>
                            {move || name.clone()}
                        </label>

                        <label style="margin-left=10px">
                            {move || skill_value}
                        </label>
                        <select name="proficiency" id="profs">
                            <option value=ProficiencyLevel::Untrained.to_string()>{ProficiencyLevel::Untrained.to_string()}</option>
                            <option value=ProficiencyLevel::Half.to_string()>{ProficiencyLevel::Half.to_string()}</option>
                            <option value=ProficiencyLevel::Trained.to_string()>{ProficiencyLevel::Trained.to_string()}</option>
                            <option value=ProficiencyLevel::Expert.to_string()>{ProficiencyLevel::Expert.to_string()}</option>
                            <option value=ProficiencyLevel::Master.to_string()>{ProficiencyLevel::Master.to_string()}</option>
                            <option value=ProficiencyLevel::Legendary.to_string()>{ProficiencyLevel::Legendary.to_string()}</option>
                        </select>
                    </div>
                }
            }
        />
        </div>
    }
}