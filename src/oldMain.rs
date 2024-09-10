use leptos::*;
use leptos::logging::log;
use mainstats_view::*;

mod mainstats_view;

use char_data::proficiency::ProficiencyLevel;

use char_data::character::Character;


fn main() { 
    mount_to_body(move || view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let (read_ketra, write_ketra) = create_signal(Character::new("Ketrania Valenzia Adriaste Uth Viharin VII"));
    let prof = ProficiencyLevel::Half;
    view! {
        <h3>{move || read_ketra().name} Level {move || read_ketra().level}</h3>
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
        <MainStatsView char=read_ketra/>
        <input type="number" on:change={move|event| write_ketra.update(|c| {
            let val: i32 = event_target_value(&event).parse().unwrap();
            c.main_stats.strength.value = val;
        })}/>
    }
}
