use crate::char_data::character::*;
use leptos::*;
use leptos::logging::log;

#[component]
pub fn MainStatsView(
    char: ReadSignal<Character>
) -> impl IntoView {
    
    view! {
        <div style="display: flex; flex-direction: row; gap: 10px">
            <For
                each=move || {
                    char()
                        .main_stats
                        .as_vec()
                        .into_iter()
                        .map(|f| (String::from(f.get_id()), String::from(f.get_abbr())))
                }
                key=|(id, _)| id.clone()
                children=move |(id, abbr)| {
                    let val = create_memo(move |_| {
                        let idClone = id.clone();
                        char.with(move |cha_obj| cha_obj.main_stats.get_stat_val(&idClone).unwrap())
                    });
                    view! { <div>{abbr}: {val}</div> }
                }
            />
        </div>
    }
}