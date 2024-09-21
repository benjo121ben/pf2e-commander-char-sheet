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
    log!("{conditions:#?}");
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
            <section>
                <DefenseView/>
            </section>
            <section class="align-center">
                <MainStatsView/>
            </section>
            <section class="flex-row flex-grow-1" style="justify-content:center">
                CONDITIONS
            </section>
        </div>
        <p style="color: red;">{move || read_save_error()}</p>
        <div class="flex-row flex-wrap space-between">
            <section class="flex-col" style="flex-grow: 0">
                <div style="flex-direction:column">
                    <h5>Skills</h5>
                    <SwitchProfView types=vec![ProficiencyType::Skill, ProficiencyType::Lore]/>
                </div>
                <div style="flex-direction:column">
                    <h5>Saves</h5>
                    <SwitchProfView types=vec![ProficiencyType::Save]/>
                </div>
                <div style="flex-direction:column">
                    <h5>Armor</h5>
                    <SwitchProfView types=vec![ProficiencyType::Armor]/>
                </div>
            </section>
            <section class="flex-col flex-grow-4">
                <WeaponView/>
            </section>
            <section class="flex-col right-side-col flex-grow-1 text-right">
                <FeatView/>
                <p>We need to test a really long text, why does it behave this way?</p>
            </section>
        </div>
    }
}