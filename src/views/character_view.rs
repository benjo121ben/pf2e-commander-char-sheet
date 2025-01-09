use std::collections::HashMap;

use crate::char_data::bonus_penalty::StatBonusPenalties;
use crate::char_data::character::*;
use crate::char_data::conditions::get_stat_bonus_penalty_map;
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

#[component]
pub fn BaseView(
    char: Character,
    feats: Vec<Feat>,
    conditions: Vec<ConditionData>,
    trait_data: HashMap<String, String>
) -> impl IntoView {
    let (read_char, write_char) = create_signal(char);
    let simple_modal_data = create_rw_signal(SimpleModalData::default());
    let sheet_error: RwSignal<SheetError> = create_rw_signal(SheetError::new(""));
    let upload_ketra = create_action( move |_| async move {
        let character = read_char.get_untracked();
        sheet_error.set(SheetError::new(""));
        let ret_val = set_char(character).await;
        if ret_val.is_err() {
            sheet_error.set(SheetError::new(&ret_val.unwrap_err().to_string()));
        }
    });
    create_effect(move |prev| {
        let _getUp = read_char.with(|c| c.name.clone());
        match prev {
            Some(_) => {
                upload_ketra.dispatch(0);
                return Some(0);
            },
            None => Some(0)
        }
        
    });
    provide_context(read_char);
    provide_context(simple_modal_data);
    provide_context(sheet_error);
    provide_context(write_char);
    provide_context(trait_data.clone());

    let conditions_map: HashMap<String, ConditionData> = conditions.into_iter().map(|cond: ConditionData| (cond.name.clone(), cond)).collect();
    let feat_map: HashMap<String, Feat> = feats.into_iter().map(|feat: Feat| (feat.name.clone(), feat)).collect();
    provide_context(conditions_map.clone());
    provide_context(feat_map.clone());

    let char_level_memo = create_memo(move |_| read_char.with(|c| c.level));
    let shield_raised_memo = create_memo(move |_| read_char.with(|c| c.shield_info.raised));
    let condition_memo = create_memo(move |_| {
        match read_char.with(|c| c.get_all_conditions(&conditions_map)) {
            Ok(condition_list) => condition_list.clone(),
            Err(error) => {panic!("ConditionSection: error getting character conditions: {error}");}
        }
    });
    let bonus_penalty_memo: Memo<HashMap<String, StatBonusPenalties>> = create_memo(move |_| {
        let conditions = condition_memo.get();
        let char_level = char_level_memo();
        let shield_raised = shield_raised_memo();
        let bonus_penalty_map = match get_stat_bonus_penalty_map(char_level, shield_raised, &conditions) {
            Ok(map) => map,
            Err(err) => panic!("{}",err),
        };
        bonus_penalty_map
    });

    provide_context(condition_memo);
    provide_context(bonus_penalty_memo);


    view!{
        <SimpleModalView data=simple_modal_data/>
        <CharView/>
        <HorseSection/>
    }
}

#[component]
pub fn CharView() -> impl IntoView {
    view! {
        <TopCharViewSection/>
        <div class="flex-row space-between">
            <ProficiencySidebar/>
            <TextCenterSection/>
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
    let bp_map = get_bonus_penalty_map_from_context("TopCharViewSection");
    let sheet_error = get_sheet_error_context("TopCharView");
    let send_debug = create_action( move |_:&i32| async move {
        sheet_error.set(SheetError::new(""));
        let ret_val = ping_server().await;
        match ret_val {
            Ok(val) => sheet_error.set(SheetError::new(&format!("{val}"))),
            Err(err) => sheet_error.set(SheetError::new(&err.to_string())),
        }
    });
    let get_speed = move || {
        let penalty_bonus = match bp_map().get("speed") {
            Some(stat_bp) => stat_bp.calculate_total(),
            None => 0,
        };

        (read_char.with(|c|c.speed) + penalty_bonus, penalty_bonus)
    };
    view!{
        <div class="flex-row">
            <div id="top_left_div">
                <div id="header_div" class="flex-row no-grow-children" style="align-items:center; gap: 20px">
                    <h2 on:click=move|_|send_debug.dispatch(0) style="margin: unset;"> 
                        {move || read_char.with(|k| k.name.clone())}
                    </h2>
                    <button style="padding: 5px;"
                        on:click=move |_| set_char.update(move |c| c.level_up_down(1))
                        on:contextmenu=move |_| set_char.update(move |c| c.level_up_down(-1))
                    >
                        Level {move || read_char.with(|k| k.level)}
                    </button>
                    <Show when=move || sheet_error.get().msg != String::from("")>
                        <p style="color: red; margin-left: 40px">{move || sheet_error.get().msg.clone()}</p>
                    </Show>
                </div>
                <div id="top_stat_div" class="flex-row no-grow-children">
                    <section>
                        <div class="flex-row align-center no-grow-children">   
                            <div>Size<br/>Medium</div>
                            <div 
                                class:adjust-up={move||get_speed().1 > 0}
                                class:adjust-down={move||get_speed().1 < 0}
                            >Speed<br/>{move || get_speed().0}ft.</div>
                        </div>
                    </section>
                    <section class="align-center">
                        <AttributeView/>
                    </section>
                    <section class="align-center">
                        <DefenseView/>
                    </section>
                    <section class="flex-row align-center" style="gap: 20px" id="hp_section">
                        <HpView is_horse=false/>
                        <ShieldView/>
                    </section>
                </div>
            </div>
            <div id="top_right_div" class="flex-row">
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
        <section class="flex-col flex-wrap" style="flex-grow: 0; flex-shrink: 0; gap:0.3rem">
            <b><SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::ClassDC] margin=false/></b>
            <b><SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Perception] margin=false/></b>
            <Show when={move||has_incr_init_memo.get()}>
                <div class="skill-grid prof-list" style="font-style:italic">
                    <div style="display:flex; flex: 1 0 0">initiative</div>
                    <div></div>
                    <div>+2</div>
                </div>
            </Show>
            <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Save] margin=true/>
            <Show when=move || !show_edit_stats.get()>
                <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Skill, ProficiencyType::Lore] margin=false/>
            </Show>
            <Show when=move || show_edit_stats.get()>
                <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Skill, ProficiencyType::Lore] margin=true/>
                <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Weapon] margin=true/>
                <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Armor] margin=true/>
            </Show>
            <button on:click=move |_| show_edit_stats.update(|b| *b=!*b) style="justify-content:center">Edit</button>
        </section>
    }
}

#[component]
pub fn HorseSection(
) -> impl IntoView {
    let (read_c, _) = get_base_context("HorseSection");
    let horse_data = create_memo(move |_| read_c.with(|c| c.animal.clone()));
    let attack_proficiency = ProficiencyLevel::Trained;
    let unarmored_defense_proficiency = ProficiencyLevel::Trained;
    let _barding_proficiency = ProficiencyLevel::Trained;

    let get_attack = move || {
        let level = read_c.with(|c| c.level);
        horse_data().attributes.get_stat("str").expect("horse should have str").value + attack_proficiency.get_bonus(level)
    };
    let get_ac = move || {
        let level = read_c.with(|c| c.level);
        10 + horse_data().attributes.get_stat("dex").expect("horse should have str").value + unarmored_defense_proficiency.get_bonus(level)
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
                <label>Attack: {get_attack}</label>
            </section>
        </div>
        <img src="horse.png" style="max-width: 100%; height: auto;"/>
        <img src="support.png" style="max-width: 100%; height: auto;"/>
        <img src="mature_horse.png" style="max-width: 100%; height: auto;"/>
    }
}

#[component]
pub fn TextCenterSection() -> impl IntoView {
    let (read_char, write_char) = get_base_context("TextCenterSection");
    let journal_index: RwSignal<usize> = create_rw_signal(0);
    let center_journal_memo = create_memo(move|_| {
        let index = journal_index.get();
        read_char.with(|c|c.journals.get(index).cloned().expect("journal should exist inside character"))
    });
    view! {
        <section class="flex-col center-col-layout">
            <div class="tabs"> 
                <For each=move|| (0..read_char.with(|c|c.journals.len()))
                    key=move|index|index.to_string()
                    children=move|index| {
                        let header_memo = create_memo(move|_|{
                            read_char.with(|c|c.journals.get(index).expect("journal should exist inside character").name.clone())
                        });
                        view! {
                            <div class:selected-tab=move||journal_index() == index on:click=move|_| journal_index.set(index)>{move||header_memo()}</div>
                        }
                    }    
                />
            </div>
            <textarea 
                class="center-text-area" 
                class:first-tab-selected=move||journal_index() == 0
                id="center-text-area"
                on:change=move |event| {
                    write_char.update(|c| {
                        let val: String = event_target_value(&event);
                        c.journals.get_mut(journal_index.get()).and_then(|journal|{
                            journal.text = val.clone();
                            Some(val)
                        });
                    })
                }
                prop:value={move || center_journal_memo.get().text.clone()}
            />
        </section>
    }
}