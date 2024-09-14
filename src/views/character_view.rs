use crate::char_data::character::*;
use crate::char_data::proficiency::*;
use crate::server_side::server_functions::*;
use super::stats_views::{EditMainstatsView, MainStatsView, SkillView};

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
        log!("SEND ketra:\n{ketra:#?}");
        let ret_val = set_char(ketra).await;
        match ret_val {
            Ok(_) => {log!("Save successful"); write_save_error.set("".to_string())},
            Err(err) => {log!("Error saving "); write_save_error.set(err.to_string())},
        }
    });
    create_effect(move |_| {
        let _getUp = read_ketra();
        upload_ketra.dispatch(0);
    });
    let (prof_read, prof_write) = create_signal(ProficiencyLevel::Half);
    view! {
        <h2>{move || read_ketra.with(|k| k.name.clone())}</h2>
        <button>Level {move || read_ketra.with(|k| k.level)}</button>
        
        <input
            type="number"
            style="width: auto"
            value={move || read_ketra.with(|k| k.level)}
            on:change=move |event| {
                write_ketra
                    .update(move |c| {
                        let val: i32 = event_target_value(&event).parse().unwrap();
                        c.level = val;
                    })
            }
        />
        <p style="color: red;">{move || read_save_error()}</p>
        <p>This is a test value {move || prof_read().get_bonus(read_ketra.with(|k| k.level))}</p>
        //<EditMainstatsView read_character=read_ketra write_character=write_ketra/>
        <MainStatsView read_char=read_ketra write_char=write_ketra/>
        <SkillView read_character=read_ketra write_character=write_ketra/>
    }
}