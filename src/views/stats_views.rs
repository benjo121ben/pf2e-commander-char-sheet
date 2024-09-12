use crate::char_data::character::*;
use leptos::*;
use leptos::logging::log;


#[component]
pub fn MainStatsView(
    read_char: ReadSignal<Character>,
    write_char: WriteSignal<Character>
) -> impl IntoView {
    let update_stat = move |id: String, offset: i32| write_char
    .update(|f| {
        f.main_stats.set_stat(&id, f.main_stats.get_stat(&id).unwrap().value + offset);
    });
    view! {
        <div style="display: flex; flex-direction: row; gap: 10px">
            <For
                each=move || {
                    read_char.with(|c|
                        c.main_stats
                        .as_vec()
                        .into_iter()
                        .map(|f| (String::from(f.get_id()), String::from(f.get_abbr())))
                    )
                }
                key=|(id, _)| id.clone()
                children=move |(id, abbr)| {
                    let idForLeftClick = id.clone();
                    let idForrightClick = id.clone();
                    let val = create_memo(move |_| {
                        let idClone = id.clone();
                        read_char.with(move |cha_obj| cha_obj.main_stats.get_stat_val(&idClone).unwrap())
                    });
                    view! { 
                        <div 
                            on:click=move |_| {update_stat(idForLeftClick.clone(), 1)}
                            on:contextmenu=move |_| update_stat(idForrightClick.clone(), -1)
                        >
                            {abbr}: {val}
                        </div> }
                }
            />
        </div>
    }
}


#[component]
pub fn EditMainstatsView(
    read_character: ReadSignal<Character>,
    write_character: WriteSignal<Character>
) -> impl IntoView {
    view! {
        <For
            each=move || {
                read_character.with(|k| k.main_stats.as_vec().into_iter().map(|f| f.get_id().to_string()))
            }
            key=|id| id.clone()
            children=move |id| {
                let idForInput = id.clone();
                let trueVal = create_memo(move |_| {
                    let idClone = id.clone();
                    read_character.with(move |k| k.main_stats.get_stat_val(&idClone)).unwrap()
                });
                view! {
                    <input
                        type="number"
                        value=trueVal
                        style="width:35px; margin-right: 10px;"
                        on:change=move |event| {
                            write_character
                                .update(|f| {
                                    let val: i32 = event_target_value(&event).parse().unwrap();
                                    f.main_stats.set_stat(&idForInput, val);
                                })
                        }
                    />
                }
            }
        />
    }
}