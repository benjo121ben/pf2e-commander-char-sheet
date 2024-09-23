use crate::char_data::character::*;
use crate::char_data::proficiency::ProficiencyLevel;
use crate::char_data::stats::ProficiencyType;
use ev::Event;
use leptos::*;
use leptos::logging::log;


#[component]
pub fn MainStatsView() -> impl IntoView {
    let unlocked = create_rw_signal(false);
    let read_char= use_context::<ReadSignal<Character>>().expect("Edit skill List expects a character to be set");
    let write_char = use_context::<WriteSignal<Character>>().expect("Edit skill List expects a character to be set");
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
pub fn EditMainstatsView() -> impl IntoView {
    let read_character= use_context::<ReadSignal<Character>>().expect("Edit skill List expects a character to be set");
    let write_character = use_context::<WriteSignal<Character>>().expect("Edit skill List expects a character to be set");
    view! {
        <For
            each=move || {
                read_character.with(|k| k.attributes.as_vec().into_iter().map(|f| f.get_id().to_string()))
            }
            key=|id| id.clone()
            children=move |id| {
                let idForInput = id.clone();
                let current_val = create_memo(move |_| {
                    let id_clone = id.clone();
                    read_character.with(move |k| k.attributes.get_stat_val(&id_clone)).unwrap()
                });
                view! {
                    <input
                        type="number"
                        value=current_val
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
pub fn EditProfListView(
    types: Vec<ProficiencyType> 
) -> impl IntoView {
    let read_character= use_context::<ReadSignal<Character>>().expect("Edit skill List expects a character to be set");
    let write_character = use_context::<WriteSignal<Character>>().expect("Edit skill List expects a character to be set");
    view! {
        <div class="skill-grid">
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
                        <div style="display:flex; flex: 1">
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
    let character_data = use_context::<ReadSignal<Character>>().expect("Character should be set at this point");
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
                        <div style="display:flex; flex: 1">{move || name_clone.clone()}</div>
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
    let read_character = use_context::<ReadSignal<Character>>().expect("Feat view expects character to be set");
    view!{
        <h3 style="margin: 0">AC: {move || read_character.with(|c| c.calculate_ac())}</h3>
    }
}


#[component]
pub fn FeatView() -> impl IntoView {
    let read_character = use_context::<ReadSignal<Character>>().expect("Feat view expects character to be set");
    view!{
        <div class="flex-col right-side-col">
            <h3>Feats</h3>
            <For
                each={move || read_character.with(|c| c.feats.clone())}
                key={move |feat| feat.name.clone()}
                children=move |feat| {
                    view!{
                        <div class="flex-col right-side-col">
                            <h4>{move || feat.name.clone()}</h4>
                            <p>{move || feat.description.clone()}</p>
                        </div>
                    }
                }
            />
        </div>
    }
}