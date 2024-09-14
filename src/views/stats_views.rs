use crate::char_data::character::*;
use crate::char_data::proficiency::ProficiencyLevel;
use crate::char_data::stats::{CalculatedStat, ProficiencyType};
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
                read_character.with(
                    |k| k.proficiencies.clone().into_iter().filter(move |s| s.p_type == ProficiencyType::Skill || s.p_type == ProficiencyType::Lore)
                )
            }
            key=|skill| skill.name.clone()
            children=move |skill| {
                let name = skill.name.clone();
                let name2 = skill.name.clone();
                let skill_prof = skill.proficiency.clone();
                let skill_value = create_memo({move |_| read_character.with(|c| c.get_skill_obj_from_skill_name(&(name.clone())).unwrap().calculate_stat(&c))});
                let options = vec![ProficiencyLevel::Untrained, ProficiencyLevel::Half, ProficiencyLevel::Trained, ProficiencyLevel::Expert, ProficiencyLevel::Master, ProficiencyLevel::Legendary];
                view! {
                    <div style="display:flex; flex-direction: row">
                        <label>
                            {move || name2.clone()}
                        </label>

                        <label style="margin-left=10px">
                            {move || skill_value}
                        </label>
                        <select name="proficiency" id="profs"
                            on:change=move |event| {
                                write_character.update(|character| {
                                    let val: String = event_target_value(&event);
                                    let indx = character.get_skill_obj_indx_from_skill_name(&skill.name);
                                    let panic_name = skill.name.clone();
                                    match indx {
                                        Some(i) => {character.proficiencies[i].proficiency = ProficiencyLevel::from(val)},
                                        None => {panic!("Could not get index for {panic_name}")}
                                    };
                                    
                                })
                            }
                        >
                            {
                                options.into_iter().map(|lvl| view!{
                                    <option selected=skill_prof.clone()==lvl value=lvl.to_string()>{lvl.to_string()}</option>
                                }).collect::<Vec<_>>()
                            }
                        </select>
                    </div>
                }
            }
        />
        </div>
    }
}



#[component]
pub fn NonSkillDebugView(
    read_character: ReadSignal<Character>,
    write_character: WriteSignal<Character>
) -> impl IntoView {
    view! {
        <div style="display:flex; flex-direction: column; margin-top:20px">
            <For
                each=move || {
                    read_character.with(
                        |k| k.proficiencies.clone().into_iter().filter(move |s| s.p_type != ProficiencyType::Skill && s.p_type != ProficiencyType::Lore)
                    )
                }
                key=|skill| skill.name.clone()
                children=move |skill| {
                    let name = skill.name.clone();
                    let name2 = skill.name.clone();
                    let skill_prof = skill.proficiency.clone();
                    let skill_value = create_memo({move |_| read_character.with(|c| c.get_skill_obj_from_skill_name(&(name.clone())).unwrap().calculate_stat(&c))});
                    let options = vec![ProficiencyLevel::Untrained, ProficiencyLevel::Half, ProficiencyLevel::Trained, ProficiencyLevel::Expert, ProficiencyLevel::Master, ProficiencyLevel::Legendary];
                    view! {
                        <div style="display:flex; flex-direction: row">
                            <label>
                                {move || name2.clone()}
                            </label>

                            <label style="margin-left=10px">
                                {move || skill_value}
                            </label>
                            <select name="proficiency" id="profs"
                                on:change=move |event| {
                                    write_character.update(|character| {
                                        let val: String = event_target_value(&event);
                                        let indx = character.get_skill_obj_indx_from_skill_name(&skill.name);
                                        let panic_name = skill.name.clone();
                                        match indx {
                                            Some(i) => {character.proficiencies[i].proficiency = ProficiencyLevel::from(val)},
                                            None => {panic!("Could not get index for {panic_name}")}
                                        };
                                        
                                    })
                                }
                            >
                                {
                                    options.into_iter().map(|lvl| view!{
                                        <option selected=skill_prof.clone()==lvl value=lvl.to_string()>{lvl.to_string()}</option>
                                    }).collect::<Vec<_>>()
                                }
                            </select>
                        </div>
                    }
                }
            />
        </div>
    }
}