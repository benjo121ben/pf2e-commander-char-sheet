use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;

use leptos::logging::log;

use crate::views::*;
use crate::char_data::proficiency::ProficiencyLevel;

use crate::char_data::character::Character;

#[server(AddTodo, "/api")]
pub async fn get_char() -> Result<Character, ServerFnError> {
    Ok(Character::new("Ketrania Valenzia Adriaste Uth Viharin VII"))
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (read_ketra, write_ketra) = create_signal(Character::new(""));
    let (error_read, error_write) = create_signal(ServerFnError::new("Empty"));
    let (flag_error_read, flag_error_write) = create_signal(false);
    let get_ketra = async {
        match get_char().await {
            Ok(val) => write_ketra.set(val),
            Err(err) => log!("{err}")
        }
    };
    let error_message = move || flag_error_read().then(|| error_read().to_string());
    let prof = ProficiencyLevel::Half;
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/pf2e-char-sheet.css"/>
        // sets the document title
        <Title text="Welcome to Leptos"/>
        <h3>{move || read_ketra().name} Level {move || read_ketra().level}</h3>
        <p>{error_message}</p>
        <input type="number" on:change={move|event| write_ketra.update(|c| {
            let val: i32 = event_target_value(&event).parse().unwrap();
            c.level = val;
        })}/>
        <p>
            This is a test value {move || prof.get_bonus(read_ketra().level)}
        </p> 
        <For
            each=move || read_ketra().main_stats.as_vec().into_iter().map(|f| f.get_id().to_string())
            key=|id| id.clone()
            children=move |id| {
                let idForInput = id.clone();
                let trueVal = create_memo(move |_|{
                    let idClone = id.clone();
                    read_ketra.with(move|k| k.main_stats.get_stat_val(&idClone)).unwrap()
                });
                view!{
                    <input type="number" value={trueVal} on:change={move |event| write_ketra.update(|f| {
                        let val: i32 = event_target_value(&event).parse().unwrap();
                        f.main_stats.set_stat(&idForInput, val);
                    })}/>
                }
            }
        />
        <mainstats_view::MainStatsView char=read_ketra/>
        <input type="number" on:change={move|event| write_ketra.update(|c| {
            let val: i32 = event_target_value(&event).parse().unwrap();
            c.main_stats.strength.value = val;
        })}/>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
