use crate::char_data::character::*;
use crate::char_data::conditions::Condition;
use crate::char_data::stats::ProficiencyType;
use crate::server_side::server_functions::*;
use super::stats_views::*;
use super::equip_views::*;

use leptos::*;
use leptos::logging::log;

#[component]
pub fn CharacterView(
    char: Character,
    conditions: Vec<Condition>
) -> impl IntoView {
    log!("Char on init {char:#?}");
    let (read_ketra, write_ketra) = create_signal(char);
    let (read_save_error, write_save_error) = create_signal(String::from(""));
    let upload_ketra = create_action( move |_:&i32| async move {
        let ketra = read_ketra.get_untracked();
        write_save_error.set("".to_string());
        let ret_val = set_char(ketra).await;
        match ret_val {
            Ok(_) => {},
            Err(err) => {log!("Error saving "); write_save_error.set(err.to_string())},
        }
    });
    create_effect(move |prev| {
        let _getUp = read_ketra.with(|c| c.name.clone());
        log!("pushed");
        match prev {
            Some(_) => {
                upload_ketra.dispatch(0);
                return Some(());
            },
            None => Some(())
        }
        
    });
    provide_context(read_ketra);
    provide_context(write_ketra);
    provide_context(conditions.clone());
    view! {
        <h2>{move || read_ketra.with(|k| k.name.clone())}</h2>
        <div id="top_div" class="flex-row flex-wrap no-grow-children">
            <section>
                <div class="flex-row align-center no-grow-children">   
                    <button
                        on:click=move |_| {write_ketra.update(move |c| {
                            c.level += 1;
                            c.hp_info.calculate_max_hp(c.level, c.attributes.get_stat("con").expect("There should be a con stat").value);
                        })}
                        on:contextmenu=move |_| {write_ketra.update(move |c| {
                            c.level -= 1;
                            c.hp_info.calculate_max_hp(c.level, c.attributes.get_stat("con").expect("There should be a con stat").value);
                        })}
                    >
                        Level {move || read_ketra.with(|k| k.level)}
                    </button>
                    <div>SIZE<br/>Medium</div>
                    <div>SPEED<br/>30ft.</div>
                </div>
            </section>
            <section class="align-center">
                <DefenseView/>
            </section>
            <section class="align-center">
                <MainStatsView/>
            </section>
            <section class="align-center" id="hp_section">
                <HpView/>
            </section>
        </div>
        <Show
            when=move || read_save_error() != String::from("")
        >
            <p style="color: red;">{move || read_save_error()}</p>
        </Show>
        <div class="flex-row space-between">
            <ProficiencySidebar/>
            <section class="flex-col flex-grow-4" style="flex-shrink: 4">
                <TacticsView/>

                <textarea 
                    class="no-grow center-text-area" 
                    id="test"
                    on:change=move |event| {
                        write_ketra
                        .update(|c| {
                            let val: String = event_target_value(&event);
                            c.text = val;
                        })
                    }
                    prop:value={move || read_ketra.with(|c| c.text.clone())}
                />
            </section>
            <section class="flex-col right-side-col text-right no-grow">
                <FeatView/>
            </section>
        </div>
        <div class="flex-row">
        
        </div>
    }
}

#[component]
pub fn ProficiencySidebar(
) -> impl IntoView {
    view! {
        <section class="flex-col flex-wrap" style="flex-grow: 0; flex-shrink: 0">
            <b>
                <SwitchProfView types=vec![ProficiencyType::ClassDC]/>
                <SwitchProfView types=vec![ProficiencyType::Perception]/>
            </b>
            <div class="flex-col">
                <h5>Saves</h5>
                <SwitchProfView types=vec![ProficiencyType::Save]/>
            </div>
            <div class="flex-col">
                <h5>Attack</h5>
                <SwitchProfView types=vec![ProficiencyType::Weapon]/>
            </div>
            <div class="flex-col">
                <h5>Skills</h5>
                <SwitchProfView types=vec![ProficiencyType::Skill, ProficiencyType::Lore]/>
            </div>
            
            <div class="flex-col">
                <h5>Armor</h5>
                <SwitchProfView types=vec![ProficiencyType::Armor]/>
            </div>
        </section>
    }
}


/* <section class="flex-row flex-grow-1 flex-shrink" style="justify-content:center">
    {
        move || {
            let condition_clone: Condition = conditions[0].clone();
            view!{
                <h3 style="no-grow">{move || condition_clone.name.clone()}</h3>
                <p style="flex-grow-1">{move || condition_clone.description.clone()}</p>
            }
        }
    }
</section> */