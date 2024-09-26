use std::collections::HashMap;

use crate::char_data::character::*;
use crate::char_data::proficiency::ProficiencyLevel;
use crate::char_data::stats::ProficiencyType;
use leptos::ev::Event;
use leptos::*;
use leptos::logging::log;




#[component]
pub fn MainStatsView() -> impl IntoView {
    let unlocked = create_rw_signal(false);
    let read_char= use_context::<ReadSignal<Character>>().expect("MainstatsView expects a character to be set");
    let write_char = use_context::<WriteSignal<Character>>().expect("MainstatsView expects a character to be set");
    let update_stat = move |id: String, offset: i32| write_char
    .update(|f| {
        f.attributes.set_stat(&id, f.attributes.get_stat(&id).unwrap().value + offset);
        f.hp_info.calculate_max_hp(f.level, f.attributes.get_stat("con").expect("There should be a con stat").value);
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
                    let id_for_left_click = id.clone();
                    let id_for_right_click = id.clone();
                    let val = create_memo(move |_| {
                        let id_clone = id.clone();
                        read_char.with(move |cha_obj| cha_obj.attributes.get_stat_val(&id_clone).unwrap())
                    });
                    view! { 
                        <div 
                            on:click=move |_| {
                                if unlocked.get() {
                                    update_stat(id_for_left_click.clone(), 1)
                                }
                            }
                            on:contextmenu=move |_| {
                                if unlocked.get() {
                                    update_stat(id_for_right_click.clone(), -1)
                                }
                            }
                        >
                            {abbr}: {val}
                        </div> }
                }
            />
            <button
                on:click=move |_| unlocked.update(|l| *l = !*l)
            >
            {
                move || if unlocked.get() {
                    "Unlocked"
                } else {
                    "Locked"
                }
            }
            </button>
        </div>
    }
}


#[component]
pub fn HpView() -> impl IntoView {
    let read_char= use_context::<ReadSignal<Character>>().expect("MainstatsView expects a character to be set");
    let write_char = use_context::<WriteSignal<Character>>().expect("MainstatsView expects a character to be set");
    let reset_input = create_rw_signal(true);
    let hp_view = move || read_char.with(|c| {
        let hp = c.hp_info.get_hp();
        let maxhp = c.hp_info.get_max_hp();
        format!("{hp}/{maxhp}").to_string()
    });
    view! {
        <div class="flex-row">
            <label name="maxhp" id="maxhp"
                on:click=move |_| write_char.update(|c| c.hp_info.change_hp(1))
                on:contextmenu=move |_| write_char.update(|c| c.hp_info.change_hp(-1))
            >
                {move || hp_view()}
            </label>
            <label style="color: blue" name="temphp" id="temphp">
                {move || read_char.with(|c| c.hp_info.get_temp())}
            </label>
            <label name="label_temphp" id="label_temphp" class="scaling-text">
                set temp
            </label>
            <input 
                type="number" 
                name="temphp_inp" 
                id="temphp_inp" 
                class="hp-input" 
                prop:value=move || {read_char.with(|c| c.hp_info.get_temp())} 
                on:change=move |event: Event| write_char.update(|c|{
                    let val: i32 = event_target_value(&event).parse().unwrap();
                    c.hp_info.set_temp(val);
                })
            />

            <label name="label_hp" id="label_hp" class="scaling-text">
                change hp
            </label>
            <input 
                type="number" 
                name="hp_inp" 
                id="hp_inp" 
                class="hp-input"
                prop:value=move || {let _ = reset_input(); return String::from("")} 
                on:change=move |event: Event| write_char.update(|c|{
                    let val: i32 = event_target_value(&event).parse().unwrap();
                    c.hp_info.change_hp(val);
                    reset_input.update(|f| *f=!*f);
                })
            />
        </div>
    }

}


#[component]
pub fn EditProfListView(
    types: Vec<ProficiencyType> 
) -> impl IntoView {
    let read_character= use_context::<ReadSignal<Character>>().expect("Edit skill List expects a character to be set");
    let write_character = use_context::<WriteSignal<Character>>().expect("Edit skill List expects a character to be set");
    view! {
        <div class="skill-grid skill-grid-edit">
            <For
                each=move || {
                    let types_clone = types.clone();
                    read_character.with(
                        |k| k.proficiencies.clone().into_iter().filter(move |s| types_clone.contains(&s.p_type.clone()))
                    )
                }
                key=|skill| skill.name.clone()
                children=move |skill| {
                    let name = skill.name.clone();
                    let name2 = skill.name.clone();
                    let skill_prof = skill.proficiency.clone();
                    let skill_value = create_memo({move |_| read_character.with(|c| c.get_prof_obj_from_name(&(name.clone())).expect("should be able to find proficiency").calculate_stat(&c))});
                    let options = vec![ProficiencyLevel::Untrained, ProficiencyLevel::Trained, ProficiencyLevel::Expert, ProficiencyLevel::Master, ProficiencyLevel::Legendary];
                    let change_proficiency = move |event: Event| {
                        write_character.update(|character| {
                            let val: String = event_target_value(&event);
                            let indx = character.get_prof_indx_from_name(&skill.name);
                            let panic_name = skill.name.clone();
                            match indx {
                                Some(i) => {character.proficiencies[i].proficiency = ProficiencyLevel::from(val)},
                                None => {panic!("Could not get index for {panic_name}")}
                            };
                            
                        })
                    };
                    view! {
                        <div style="display:flex; flex: 1 0 0">
                            {move || name2.clone()}
                        </div>
                        <select name="proficiency" 
                            id="profs"
                            on:change=change_proficiency
                        >
                            {
                                options.into_iter().map(|lvl| view!{
                                    <option selected=lvl.to_string()==skill_prof.clone().to_string() value=lvl.to_string()>{lvl.to_string()}</option>
                                }).collect::<Vec<_>>()
                            }
                        </select>
                        <div style="margin-left=10px">
                            {move || skill_value}
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn ProficiencyListView(
    types: Vec<ProficiencyType> 
) -> impl IntoView {
    let character_data = use_context::<ReadSignal<Character>>().expect("ProfView: Character should be set at this point");
    view! {
        <div class="skill-grid">
            <For
                each=move || {
                    let types_clone = types.clone();
                    character_data.with(
                        |k| k.proficiencies.clone().into_iter().filter(move |s| types_clone.contains(&s.p_type.clone()))
                    )
                }
                key=|skill| skill.name.clone()
                children=move |skill| {
                    let name_clone = skill.name.clone();
                    let skill_name_clone = move || skill.name.clone();
                    let get_skill_data = move || (&character_data).with(|c| c.proficiencies[c.get_prof_indx_from_name(&skill_name_clone()).expect("Expected an index for the proficiency")].clone());
                    let get_skill_prof = {
                        let data = get_skill_data.clone();
                        move || data().proficiency.to_string()[..1].to_string()
                    };
                    let get_skill_val = {
                        let data = get_skill_data.clone();
                        move || character_data.with(|c| data().calculate_stat(c))
                    };
                    let is_proficient = {
                        let get_prof = get_skill_prof.clone();
                        move || get_prof() != String::from("U")
                    };
                    view! {
                        <div style="display:flex; flex: 1 0 0">{move || name_clone.clone()}</div>
                        <div class="proficiency-letter" class:proficiency-letter-trained=is_proficient>{get_skill_prof}</div>
                        <div>{get_skill_val}</div>
                    }.into_view()
                }
            />
        </div>
    }
}

#[component]
pub fn SwitchProfView(
    types: Vec<ProficiencyType> 
) -> impl IntoView {
    let show_edit_stats = create_rw_signal(false);
    view!{
        <div class="flex-col" style="flex-grow: 0">
            {
                let t_clone = types.clone();
                move || if !show_edit_stats.get() {
                    view! {<ProficiencyListView types=t_clone.clone()/>}
                }
                else {
                    view! {
                        <EditProfListView types=t_clone.clone()/>
                    }.into_view()
                } 
            }
            <button on:click=move |_| show_edit_stats.update(|b| *b=!*b) style="justify-content:center">Edit</button>
        </div>
    }
}

#[component]
pub fn DefenseView() -> impl IntoView {
    let read_character = use_context::<ReadSignal<Character>>().expect("Defense view expects character to be set");
    let write_character = use_context::<WriteSignal<Character>>().expect("Defense view expects character to be set");
    let shield_raised = move || read_character.with(|c| c.shield_raised);
    let calc_ac = move || read_character.with(|c| c.calculate_ac());
    let switch_shield_pos = move |_| write_character.update(|c| c.shield_raised=!c.shield_raised);

    view!{
        <div class="flex-col" style="align-items: stretch">
            <h3 style="margin: 0; white-spacce:nowrap" on:click=switch_shield_pos class:boosted-stat=shield_raised.clone()>
                AC: {calc_ac}
            </h3>
            <button on:click=switch_shield_pos style="justify-content:center">
                {
                    move || if shield_raised() {
                        "Lower"
                    }
                    else {
                        "Raise"
                    }
                }
            </button>
        </div>
    }
}


#[component]
pub fn FeatView() -> impl IntoView {
    let read_character = use_context::<ReadSignal<Character>>().expect("Feat view expects character to be set");
    view!{
        <div class="flex-col align-flex-start">
            <h3>Feats</h3>
            <For
                each={move || read_character.with(|c| c.feats.clone())}
                key={move |feat| feat.name.clone()}
                children=move |feat| {
                    let collapse = create_rw_signal(false);
                    view!{
                        <div class="flex-col smaller-gap" on:click=move |_| collapse.update(|c| *c = !*c)>
                            <h4>{move || feat.name.clone()}</h4>
                            <Show when=move || collapse.get()>
                                <TraitView trait_names=feat.traits.clone()/>
                                <p class="tiny-text">{let desc = feat.description.clone(); move || desc.clone()}</p>
                            </Show>
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn TraitView(
    trait_names: Vec<String>
) -> impl IntoView {
    let traitList = use_context::<HashMap<String, String>>().expect("Trait Hashmap should be set by now");
    view!{
        <div class="flex-row">{
            trait_names.into_iter().map(|t| {
                let hash_val = traitList.get(&t).clone();
                let tooltip = match hash_val {
                    Some(description) => String::from(description),
                    None => {log!("No tooltip was set for {0}", t); String::from("No tooltip") },
                };
                view!{
                    <div class="trait tiny-text" title=tooltip>{t}</div>
                }
            }).collect::<Vec<_>>()
        }</div>
    }
}