use crate::char_data::gear::{Gear, GearType, WeaponType};
use crate::char_data::tactics::Tactic;
use crate::views::action_view::ActionView;
use crate::views::view_helpers::*;
use super::stats_views::TraitView;
use leptos::*;
use leptos::logging::log;

#[component]
pub fn EquipView() -> impl IntoView {
    let (character_data, _) = get_base_context("EquipView");
    view! {
        <For
            each=move ||character_data.with(|k| k.gear_list.clone())
            key=|gear_item| gear_item.name.clone()
            children=move |gear_item| {
                let item_name = gear_item.name.clone();
                if gear_item.g_type == GearType::Weapon {
                    return view! {<WeaponView item=gear_item/>}.into_view();
                } 
                let collapse = create_rw_signal(false);
                view! {
                    <div class="flex-col align-flex-start bright-bg" 
                        on:click=move |_| collapse.update(|c| *c = !*c)
                    >
                        <h4 style="margin:unset">{
                            move || format!("{item_name}")
                        }</h4>
                        <Show when=move || collapse.get()>
                            <TraitView trait_names=gear_item.traits.clone()/>
                            <hr/>
                            <div class="tiny-text" inner_html={let desc = gear_item.description.clone(); move || desc.clone()}></div>
                        </Show>
                    </div>
                }.into_view()
            }
        />
    }
}

#[component]
pub fn WeaponView(
    item: Gear
) -> impl IntoView {
    let (character_data, _) = get_base_context("WeaponView");
    let debug_name_clone = item.name.clone();
    let mut err_text = String::from("");

    if item.g_type != GearType::Weapon {
        err_text = format!("WeaponView: This item is not a weapon: {debug_name_clone}");
    }
    if item.weap_info.is_none() {
        err_text = format!("WeaponView: This item does not have Weapon Info: {debug_name_clone}");
    }
    if item.proficiency.is_none() {
        err_text = format!("WeaponView: This item does not have a proficiency: {debug_name_clone}");
    }

    if &err_text != "" {
        log!("Logged error {err_text}");
        return err_text.into_view();
    }

    let get_weapon = move || {
        let weapon_Item = character_data.with(|c| c.gear_list.iter().find(|i| i.name == item.name.clone()).cloned());
        match weapon_Item {
            Some(weapon) => {
                Ok(weapon)
            },
            None => {
                let second_clone = item.name.clone();
                Err(format!("WeaponView: Could not find a weapon with name: {second_clone}. weird synchro error"))
            }
        }
    };

    let get_proficicency = move |weapon: &Gear| {
        let prof_copy = weapon.proficiency.clone().expect("WeaponView: There should be a weapon proficiency");
        let stat_opt = character_data.with(|c| c.get_prof_obj_from_name(&prof_copy));
        match stat_opt {
            Some(prof) => Ok(prof),
            None => {let name_copy = prof_copy.clone(); Err(format!("WeaponView: Could not find a proficiency with name {name_copy}"))}
        }
    };

    let get_weapon_view = move || -> Result<View, String> {
        let weapon = get_weapon()?;
        let mut stat = get_proficicency(&weapon)?;
        let weap_info = weapon.weap_info.expect("WeaponView: We expect weapon_data by now. It was checked before");
        let is_melee = weap_info.w_type == WeaponType::Melee;
        stat.attribute = if is_melee {"str"} else {"dex"}.to_string();
        let attack_bonus = character_data.with(|c|stat.calculate_stat(c));
        let bonus_progression_proficiency = 1; //TODO here automatic bonus progression
        let prefix = String::from(
            if attack_bonus + bonus_progression_proficiency > 0 {"+"} else {""}
        );
        let dice_amount = 1; //TODO here automatic bonus progression
        let dice_size = weap_info.damage;
        let stat_bonus_dmg = if weap_info.w_type == WeaponType::Melee {
            let val = character_data.with(|c| c.attributes.get_stat_val("str"))?;
            let prefix = get_prefix(val);
            format!(" {prefix}{val}")
        } else {"".to_string()};
        let dam_type = weap_info.d_type;
        let full_damage_text = format!("{dice_amount}d{dice_size}{stat_bonus_dmg}{dam_type}");
        let total_bonus = format!("{0}{1} {2}", prefix, attack_bonus + bonus_progression_proficiency, full_damage_text); //TODO str bonus
        Ok(view!{
            <div class="flex-col bright-bg">
                <div class="flex-row">
                    <h4>{let name_clone = weapon.name.clone(); move|| name_clone.clone()}</h4>
                    <p>{
                        move || total_bonus.clone()
                    }</p>
                    <p inner_html={move|| weapon.description.clone()}/>
                </div>
                <TraitView trait_names=weapon.traits.clone()/>
            </div>
        }.into_view())
    };

    view!{   
        {move || match get_weapon_view() {
            Ok(w_view) => w_view,
            Err(error_str) => {
                view!{
                    <p class="error">{error_str}</p>
                }.into_view()
            }
        }}    
    }.into_view()
}

#[component]
pub fn TacticsView() -> impl IntoView {
    let (character_data, character_write) = get_base_context("TacticsView");
    let max_tactics = 2;
    let count_tactics = {
        move || character_data.with(|c| {
            c.tactics.iter().filter(|tactic| tactic.selected).count()
        })
    };
    let tactics_header = move || {
        let count = count_tactics();
        format!("Tactics [{count} / {max_tactics}]")
    };
    view! {
        <div>
            <h4>{tactics_header}</h4>
            <div class="flex-col no-grow" style="gap:10px">
                <For
                    each=move ||character_data.with(|k| k.tactics.clone())
                    key=|tactic| tactic.name.clone()
                    children=move |tactic| {
                        let tac_name = tactic.name.clone();
                        let collapse = create_rw_signal(false);
                        let get_selected_on_tactic = {
                            let tac_name2 = tactic.name.clone();
                            move || character_data.with(|c|{
                                c.tactics.iter().find(|val| val.name == tac_name2).expect("TacticView: get_selected - There should be a tactic of the same name").selected
                            })
                        };
                        let update_selected_on_tactic = {
                            let tac_name2 = tactic.name.clone();
                            let get_selected_on_tactic_clone = get_selected_on_tactic.clone();
                            move |_| {
                                if count_tactics() >= max_tactics && !get_selected_on_tactic_clone() {
                                    return;
                                }
                                character_write.update(|c|{
                                    let mut_ref: &mut Tactic = c.tactics.iter_mut().find(|val| val.name == tac_name2).expect("TacticView: update_selected_on_tactic - There should be a tactic of the same name"); 
                                    mut_ref.selected = !mut_ref.selected
                                });
                            }
                        };
                        view! {
                            <div class="flex-col align-flex-start bright-bg" 
                                on:click=move |_| collapse.update(|c| *c = !*c)
                                on:contextmenu=update_selected_on_tactic
                                class:selected-tactic=get_selected_on_tactic
                            >
                                <div class="flex-row feat-title-row">
                                    <h4>{
                                        move || format!("{tac_name}")
                                    }</h4>
                                    <Show when=move || tactic.actions != 0>
                                        <ActionView number=tactic.actions/>
                                    </Show>
                                </div>
                                <Show when=move || collapse.get()>
                                    <TraitView trait_names=tactic.traits.clone()/>
                                    <hr/>
                                    <div class="tiny-text" inner_html={let desc = tactic.description.clone(); move || desc.clone()}></div>
                                </Show>
                            </div>
                        }.into_view()
                    }
                />
            </div>
        </div>
    }
}