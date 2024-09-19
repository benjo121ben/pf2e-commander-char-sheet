use crate::char_data::character::*;
use crate::char_data::stats::ProficiencyType;
use crate::server_side::server_functions::*;
use super::stats_views::*;

use leptos::*;
use leptos::logging::log;

#[component]
pub fn CharacterView(
    char: Character,
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
    let show_edit_stats = create_rw_signal(false);
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
        <div id="top_div" class="flex-row" style="align-items:stretch">
            <div class="flex-col no-grow">
                <div class="flex-row align-center">   
                    <button
                        on:click=move |_| {write_ketra.update(move |c| c.level += 1)}
                        on:contextmenu=move |_| {write_ketra.update(move |c| c.level -= 1)}
                    >
                        Level {move || read_ketra.with(|k| k.level)}
                    </button>
                    <div>SIZE<br/>Medium</div>
                    <div>SPEED<br/>30ft.</div>
                </div>
                <MainStatsView/>
                <DefenseView/>
            </div>
            <div class="flex-row flex-grow-1" style="justify-content:center">
                CONDITIONS
            </div>
        </div>
        <p style="color: red;">{move || read_save_error()}</p>
        <div class="flex-row space-between">
            <div class="flex-col" style="flex-grow: 0">
                {
                    move || if !show_edit_stats.get() {
                        view! {<ProficiencyListView types=vec![ProficiencyType::Skill, ProficiencyType::Lore]/>}
                    }
                    else {
                        view! {
                            <EditProfListView types=vec![ProficiencyType::Skill, ProficiencyType::Lore]/>
                        }.into_view()
                    } 
                }
                <ProficiencyListView types=vec![ProficiencyType::Save, ProficiencyType::Armor]/>
                <button on:click=move |_| show_edit_stats.update(|b| *b=!*b) style="justify-content:center">Edit</button>
            </div>
            <div class="flex-col" style="text-align: right">
                <FeatView/>
            </div>
        </div>
    }
}