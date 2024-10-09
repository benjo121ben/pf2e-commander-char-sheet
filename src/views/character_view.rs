use std::collections::HashMap;

use crate::char_data::character::*;
use crate::char_data::conditions::Condition;
use crate::char_data::stats::ProficiencyType;
use crate::error_template::SheetError;
use crate::server_side::server_functions::*;
use super::view_helpers::*;
use super::stats_views::*;
use super::equip_views::*;

use leptos::*;
use leptos::logging::log;

#[component]
pub fn BaseView(
    char: Character,
    conditions: Vec<Condition>,
    trait_data: HashMap<String, String>
) -> impl IntoView {
    //log!("Char on init {char:#?}");
    let (read_ketra, write_ketra) = create_signal(char);
    let sheet_error = create_rw_signal(SheetError::new(""));
    let upload_ketra = create_action( move |_:&i32| async move {
        let ketra = read_ketra.get_untracked();
        sheet_error.set(SheetError::new(""));
        let ret_val = set_char(ketra).await;
        match ret_val {
            Ok(_) => {},
            Err(err) => {log!("Error saving "); sheet_error.set(SheetError::new(&err.to_string()))},
        }
    });
    create_effect(move |prev| {
        let _getUp = read_ketra.with(|c| c.name.clone());
        match prev {
            Some(_) => {
                upload_ketra.dispatch(0);
                return Some(0);
            },
            None => Some(0)
        }
        
    });
    provide_context(read_ketra);
    provide_context(sheet_error);
    provide_context(write_ketra);
    provide_context(conditions.clone());
    provide_context(trait_data.clone());
    view!{
        <CharView/>
    }
}

#[component]
pub fn CharView() -> impl IntoView {
    let (read_ketra, write_ketra) = get_base_context("CharView");
    let sheet_error = get_sheet_error_context("CharView");
    let send_debug = create_action( move |_:&i32| async move {
        sheet_error.set(SheetError::new(""));
        let ret_val = ping_server().await;
        match ret_val {
            Ok(val) => {log!("Arrived successfully "); sheet_error.set(SheetError::new(&format!("{val}")))},
            Err(err) => {log!("Error saving "); sheet_error.set(SheetError::new(&err.to_string()))},
        }
    });
    view! {
        <h2
            on:click=move|_|send_debug.dispatch(0)
        >{move || read_ketra.with(|k| k.name.clone())}</h2>
        <TopCharViewSection/>
        <Show
            when=move || sheet_error.get().msg != String::from("")
        >
            <p style="color: red;">{move || sheet_error.get().msg.clone()}</p>
        </Show>
        <div class="flex-row space-between">
            <ProficiencySidebar/>
            <section class="flex-col center-col-layout">
                <textarea 
                    class="center-text-area" 
                    id="test"
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
            <section class="flex-col equip-section">
                <EquipView/>
            </section>
            <section class="flex-col right-side-col">
                <TacticsView/>
                <FeatView/>
            </section>
        </div>
        <div class="flex-row">
        
        </div>
    }
}

#[component]
pub fn TopCharViewSection() -> impl IntoView {
    let (read_ketra, write_ketra) = get_base_context("TopCharView");
    view!{
        <div id="top_div" class="flex-row flex-wrap no-grow-children">
            <section>
                <div class="flex-row align-center no-grow-children">   
                    <button
                        on:click=move |_| {write_ketra.update(move |c| {
                            c.level += 1;
                            c.hp_info.calculate_max_hp(c.level, c.attributes.get_stat_val("con").expect("There should be a con stat"));
                        })}
                        on:contextmenu=move |_| {write_ketra.update(move |c| {
                            c.level -= 1;
                            c.hp_info.calculate_max_hp(c.level, c.attributes.get_stat_val("con").expect("There should be a con stat"));
                        })}
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
            <section class="align-center" id="hp_section">
                <HpView/>
            </section>
            <section class="align-center" id="shield_section">
                <ShieldView/>
            </section>
        </div>
    }
}

#[component]
pub fn ProficiencySidebar(
) -> impl IntoView {
    let show_edit_stats = create_rw_signal(false);
    view! {
        <section class="flex-col flex-wrap" style="flex-grow: 0; flex-shrink: 0">
            <b><SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::ClassDC]/></b>
            <b><SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Perception]/></b>
            <div class="flex-col">
                <h5>Saves</h5>
                <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Save]/>
            </div>
            <div class="flex-col">
                <h5>Skills</h5>
                <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Skill, ProficiencyType::Lore]/>
            </div>
            <Show when=move || show_edit_stats.get()>
                <div class="flex-col">
                    <h5>Attack</h5>
                    <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Weapon]/>
                </div>
                <div class="flex-col">
                    <h5>Armor</h5>
                    <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Armor]/>
                </div>
            </Show>
            <button on:click=move |_| show_edit_stats.update(|b| *b=!*b) style="justify-content:center">Edit</button>
        </section>
    }
}


/* <section class="flex-row flex-grow-1 flex-shrink" style="justify-content:center">
    {
        move || {
            let condition_clone: Condition = conditions[0].clone();
            view!{
                <h3 style="no-grow">{move || condition_clone.name.clone()}</h3>
                <p style="flex-grow-1">{move || condition_clone.description.clone()}</p>
            }
        }
    }
</section> */