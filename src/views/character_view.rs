use crate::char_data::character::*;
use crate::char_data::proficiency::*;
use crate::server_side::server_functions::*;
use super::stats_views::{EditMainstatsView, MainStatsView, SkillView, NonSkillDebugView};

use leptos::*;
use leptos::logging::log;


#[component]
pub fn FeatView(
    read_character: ReadSignal<Character>,
    write_character: WriteSignal<Character>
) -> impl IntoView {
    view!{
        <h3>Feats</h3>
        <For
            each={move || read_character.with(|c| c.feats.clone())}
            key={move |feat| feat.name.clone()}
            children=move |feat| {
                view!{
                    <h4>{move || feat.name.clone()}</h4>
                    <p>{move || feat.description.clone()}</p>
                }
            }
        />
    }
}

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
    create_effect(move |_| {
        let _getUp = read_ketra();
        upload_ketra.dispatch(0);
    });
    let (prof_read, prof_write) = create_signal(ProficiencyLevel::Half);
    view! {
        <h2>{move || read_ketra.with(|k| k.name.clone())}</h2>
        <button
            on:click=move |_| {write_ketra.update(move |c| c.level += 1)}
            on:contextmenu=move |_| {write_ketra.update(move |c| c.level -= 1)}
        >
            Level {move || read_ketra.with(|k| k.level)}
        </button>
        <div style="display:flex; flex-direction:row; justify-content:space-evenly">
            <div style="flex-grow:1">
                <p style="color: red;">{move || read_save_error()}</p>
                <p>This is a test value {move || prof_read().get_bonus(read_ketra.with(|k| k.level))}</p>
                //<EditMainstatsView read_character=read_ketra write_character=write_ketra/>
                <MainStatsView read_char=read_ketra write_char=write_ketra/>
                <SkillView read_character=read_ketra write_character=write_ketra/>
                <NonSkillDebugView read_character=read_ketra write_character=write_ketra/>
            </div>
            <div style="flex-grow:1">
                <FeatView read_character=read_ketra write_character=write_ketra/>
            </div>
        </div>
    }
}