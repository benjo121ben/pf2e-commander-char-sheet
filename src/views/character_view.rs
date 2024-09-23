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
    let (read_ketra, write_ketra) = create_signal(char);
    let (read_save_error, write_save_error) = create_signal(String::from(""));
    let upload_ketra = create_action( move |_:&i32| async move {
        let ketra = read_ketra.get_untracked();
        let ret_val = set_char(ketra).await;
        match ret_val {
            Ok(_) => {write_save_error.set("".to_string())},
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
    view! {
        <h2>{move || read_ketra.with(|k| k.name.clone())}</h2>
        <div id="top_div" class="flex-row flex-wrap no-grow-children">
            <section>
                <div class="flex-row align-center no-grow-children">   
                    <button
                        on:click=move |_| {write_ketra.update(move |c| c.level += 1)}
                        on:contextmenu=move |_| {write_ketra.update(move |c| c.level -= 1)}
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
            <section class="flex-row flex-grow-1" style="justify-content:center">
                {
                    move || {
                        let condition_clone: Condition = conditions[0].clone();
                        view!{
                            <h3 style="no-grow">{move || condition_clone.name.clone()}</h3>
                            <p style="flex-grow-1">{move || condition_clone.description.clone()}</p>
                        }
                    }
                }
            </section>
        </div>
        <p style="color: red;">{move || read_save_error()}</p>
        <div class="flex-row flex-wrap space-between">
            <section class="flex-col flex-wrap" style="flex-grow: 0">
                <SwitchProfView types=vec![ProficiencyType::Perception]/>
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
            <section class="flex-col flex-grow-4">
                <textarea class="flex-grow-1 center-text-area" id="test"
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
            <section class="flex-col right-side-col flex-grow-1 text-right">
                <FeatView/>
                <p>We need to test a really long text, why does it behave this way?</p>
            </section>
        </div>
        <div class="flex-row">
        
        </div>
    }
}