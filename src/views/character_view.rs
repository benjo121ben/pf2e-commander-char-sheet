use std::collections::HashMap;

use crate::char_data::character::*;
use crate::char_data::conditions::ConditionData;
use crate::char_data::feats::Feat;
use crate::char_data::proficiency::ProficiencyLevel;
use crate::char_data::stats::ProficiencyType;
use crate::error_template::SheetError;
use crate::server_side::server_functions::*;
use crate::views::condition_view::ConditionSection;
use crate::views::info_modal_view::SimpleModalData;
use crate::views::info_modal_view::SimpleModalView;
use super::view_helpers::*;
use super::stats_views::*;
use super::equip_views::*;

use leptos::*;
use leptos::logging::log;

#[component]
pub fn BaseView(
    char: Character,
    feats: Vec<Feat>,
    conditions: Vec<ConditionData>,
    trait_data: HashMap<String, String>
) -> impl IntoView {
    let (read_ketra, write_ketra) = create_signal(char);
    let simple_modal_data = create_rw_signal(SimpleModalData::default());
    let sheet_error: RwSignal<SheetError> = create_rw_signal(SheetError::new(""));
    let upload_ketra = create_action( move |_| async move {
        let ketra = read_ketra.get_untracked();
        sheet_error.set(SheetError::new(""));
        let ret_val = set_char(ketra).await;
        if ret_val.is_err() {
            sheet_error.set(SheetError::new(&ret_val.unwrap_err().to_string()));
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
    provide_context(simple_modal_data);
    provide_context(sheet_error);
    provide_context(write_ketra);
    provide_context(trait_data.clone());
    let conditions_map: HashMap<String, ConditionData> = conditions.into_iter().map(|cond: ConditionData| (cond.name.clone(), cond)).collect();
    let feat_map: HashMap<String, Feat> = feats.into_iter().map(|feat: Feat| (feat.name.clone(), feat)).collect();
    provide_context(conditions_map);
    provide_context(feat_map);
    view!{
        <SimpleModalView data=simple_modal_data/>
        <CharView/>
        <HorseSection/>
    }
}

#[component]
pub fn CharView() -> impl IntoView {
    let (read_char, write_char) = get_base_context("CharView");
    let center_text_memo = create_memo(move|_| read_char.with(|c|c.text.clone()));
    view! {
        <TopCharViewSection/>
        <div class="flex-row space-between">
            <ProficiencySidebar/>
            <section class="flex-col center-col-layout">
                <textarea 
                    class="center-text-area" 
                    id="test"
                    on:change=move |event| {
                        write_char.update(|c| {
                            let val: String = event_target_value(&event);
                            c.text = val;
                        })
                    }
                    prop:value={move || center_text_memo.get()}
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
    }
}

#[component]
pub fn TopCharViewSection() -> impl IntoView {
    let (read_char, set_char) = get_base_context("TopCharView");
    let sheet_error = get_sheet_error_context("TopCharView");
    let send_debug = create_action( move |_:&i32| async move {
        sheet_error.set(SheetError::new(""));
        let ret_val = ping_server().await;
        match ret_val {
            Ok(val) => sheet_error.set(SheetError::new(&format!("{val}"))),
            Err(err) => sheet_error.set(SheetError::new(&err.to_string())),
        }
    });
    view!{
        <div class="flex-row">
            <div id="top_left_div">
                <div id="header_div" class="flex-row no-grow-children">
                    <h2 on:click=move|_|send_debug.dispatch(0)>
                        {move || read_char.with(|k| k.name.clone())}
                    </h2>
                    <Show when=move || sheet_error.get().msg != String::from("")>
                        <p style="color: red; margin-left: 40px">{move || sheet_error.get().msg.clone()}</p>
                    </Show>
                </div>
                <div id="top_stat_div" class="flex-row no-grow-children">
                    <section>
                        <div class="flex-row align-center no-grow-children">   
                            <button
                                on:click=move |_| {set_char.update(move |c| {
                                    c.level += 1;
                                    c.hp_info.calculate_max_hp(c.level, c.attributes.get_stat_val("con").expect("There should be a con stat"));
                                    c.horse_hp_info.calculate_max_hp(c.level, 2);
                                })}
                                on:contextmenu=move |_| {set_char.update(move |c| {
                                    c.level -= 1;
                                    c.hp_info.calculate_max_hp(c.level, c.attributes.get_stat_val("con").expect("There should be a con stat"));
                                    c.horse_hp_info.calculate_max_hp(c.level, 2);
                                })}
                            >
                                Level {move || read_char.with(|k| k.level)}
                            </button>
                            <div>SIZE<br/>Medium</div>
                            <div>SPEED<br/>30ft.</div>
                        </div>
                    </section>
                    <section class="align-center">
                        <DefenseView/>
                    </section>
                    <section class="align-center">
                        <AttributeView/>
                    </section>
                    <section class="align-center" id="hp_section">
                        <HpView is_horse=false/>
                    </section>
                    <section class="align-center" id="shield_section">
                        <ShieldView/>
                    </section>
                </div>
            </div>
            <div id="top_right_div" class="flex-row" style="flex: 1 1 0; align-items: flex-end;">
                <ConditionSection/>
            </div>
        </div>
    }
}

#[component]
pub fn ProficiencySidebar() -> impl IntoView {
    let (read_char, _): (ReadSignal<Character>, WriteSignal<Character>) = get_base_context("ProficiencySidebar");
    let show_edit_stats= create_rw_signal(false);
    let has_incr_init_memo = create_memo(move |_| {
        read_char.with(|c| {
            check_character_flag(c, "incred_init")
        })
    });
    view! {
        <section class="flex-col flex-wrap" style="flex-grow: 0; flex-shrink: 0">
            <b><SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::ClassDC]/></b>
            <b><SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Perception]/></b>
            <Show when={move||has_incr_init_memo.get()}>
                <div class="skill-grid">
                    <div style="display:flex; flex: 1 0 0">initiative</div>
                    <div></div>
                    <div>+2</div>
                
                </div>
            </Show>
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

#[component]
pub fn HorseSection(
) -> impl IntoView {
    let (read_c, _) = get_base_context("HorseSection");
    let get_bonus = move || {
        let level = read_c.with(|c| c.level);
        ProficiencyLevel::Trained.get_bonus(level)
    };
    let get_att = move || {
        get_bonus() + 3
    };
    let get_ac = move || {
        get_bonus() + 2 + 10
    };
    view! {
        <div class="flex-row">
            <section class="align-center">
                <HpView is_horse=true/>
            </section>
            <section class="align-center">
                <label>AC: {get_ac}</label>
            </section>
            <section class="align-center">
                <label>Attack: {get_att}</label>
            </section>
        </div>
        <img src="horse.png" style="display:flex"/>
        <img src="support.png" style="display:flex"/>
        <img src="mature_horse.png" style="display:flex"/>
    }
}

