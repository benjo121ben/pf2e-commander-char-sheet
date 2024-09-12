use crate::char_data::character::*;
use crate::char_data::proficiency::*;
use crate::server_side::server_functions::*;

use leptos::*;
use leptos::logging::log;

#[component]
pub fn CharacterView(
    char: Character,
) -> impl IntoView {
    let (read_ketra, write_ketra) = create_signal(char);
    let (read_save_error, write_save_error) = create_signal(String::from(""));
    let upload_ketra = create_action( move |_:&i32| async move {
        log!("SEND ketra");
        let ret_val = set_char(read_ketra.get_untracked()).await;
        match ret_val {
            Ok(_) => log!("Save successful"),
            Err(err) => {log!("Error saving "); write_save_error.set(err.to_string())},
        }
    });
    create_effect(move |_| {
        let _getUp = read_ketra();
        log!("effect upload");
        upload_ketra.dispatch(0);
    });
    let (prof_read, prof_write) = create_signal(ProficiencyLevel::Half);
    view! {
        <h3>{move || read_ketra().name}Level {move || read_ketra().level}</h3>
        <p>{move || read_save_error()}</p>
        <input
            type="number"
            on:change=move |event| {
                write_ketra
                    .update(move |c| {
                        let val: i32 = event_target_value(&event).parse().unwrap();
                        c.level = val;
                    })
            }
        />
        <p>This is a test value {move || prof_read().get_bonus(read_ketra().level)}</p>
        <For
            each=move || {
                read_ketra().main_stats.as_vec().into_iter().map(|f| f.get_id().to_string())
            }
            key=|id| id.clone()
            children=move |id| {
                let idForInput = id.clone();
                let trueVal = create_memo(move |_| {
                    let idClone = id.clone();
                    read_ketra.with(move |k| k.main_stats.get_stat_val(&idClone)).unwrap()
                });
                view! {
                    <input
                        type="number"
                        value=trueVal
                        on:change=move |event| {
                            write_ketra
                                .update(|f| {
                                    let val: i32 = event_target_value(&event).parse().unwrap();
                                    f.main_stats.set_stat(&idForInput, val);
                                })
                        }
                    />
                }
            }
        />
        <crate::views::mainstats_view::MainStatsView char=read_ketra />
        <input
            type="number"
            on:change=move |event| {
                write_ketra
                    .update(|c| {
                        let val: i32 = event_target_value(&event).parse().unwrap();
                        c.main_stats.strength.value = val;
                    })
            }
        />

        <button type="submit" on:click=move |_| upload_ketra.dispatch(0)>
            "saveChar"
        </button>
    }
}