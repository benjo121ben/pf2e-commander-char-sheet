use crate::char_data::gear::*;
use crate::char_data::tactics::Tactic;
use crate::views::action_view::ActionView;
use crate::views::view_helpers::*;
use super::stats_views::TraitView;
use leptos::*;
use leptos::logging::log;

#[component]
pub fn EquipView() -> impl IntoView {
    let (character_data, _) = get_base_context("EquipView");
    let gear_list_memo = create_memo(move|_|character_data.with(|k| k.gear_list.clone()));
    view! {
        <For
            each=move || gear_list_memo.get()
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
                        <div style="margin:unset; font-weight:bold">{
                            move || item_name.clone()
                        }</div>
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
pub fn WeaponView(item: Gear) -> impl IntoView {
    let (character_data, _) = get_base_context("WeaponView");
    let active_bonus_penalties_memo = get_bonus_penalty_map_from_context("WeaponView");
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
        log!("Weaponview: Logged error {err_text}");
        return err_text.into_view();
    }

    let char_weapon_item_memo = create_memo(move |_| {
        let weapon_Item = character_data.with(|c| c.gear_list.iter().find(|i| i.name == item.name.clone()).cloned());
        match weapon_Item {
            Some(weapon) => {
                Ok(weapon)
            },
            None => {
                let second_clone = item.name.clone();
                Err(format!("WeaponView: Could not find a weapon with name: {second_clone}."))
            }
        }
    });

    let collapsed_signal = create_rw_signal(true);

    let get_weapon_view = move || -> Result<View, String> {
        let weapon = char_weapon_item_memo.get()?;
        let attack_data = character_data.with(|c| get_weapon_attack_data(&c, &active_bonus_penalties_memo.get(), &weapon))?;
        let full_attack_bonus = attack_data.get_full_attack_bonus();

        let bonus_damage_text = {
            let bonus = attack_data.get_full_damage_bonus();
            if bonus != 0 {
                let prefix = get_prefix(bonus);
                format!("{prefix}{bonus}")
            }
            else {"".to_string()}
        };
        let att_bonus_text = format!(
            "{0}/{1}/{2}", 
            pretty_print_int(full_attack_bonus), 
            pretty_print_int(full_attack_bonus - attack_data.map), 
            pretty_print_int(full_attack_bonus - 2*attack_data.map)
        );

        let full_damage_text = format!(
            "{0}d{1}{2}{3}",
            attack_data.dice_amount,
            attack_data.dice_size,
            bonus_damage_text,
            attack_data.dam_type
        );

        let weap_description_clone = weapon.description.clone();
        Ok(view!{
            <div class="flex-col bright-bg" style="gap: 2px;"
                on:click=move|_|collapsed_signal.update(|c| *c = !*c)
            >
                <div class="flex-row space-between">
                    <div style="font-weight:bold">
                        {let name_clone = weapon.name.clone(); move|| name_clone.clone()}
                    </div>
                    <div class="weapon-first-row-bonuses">
                        <div
                            class:adjust-up={move||attack_data.bonus_penalty_adjustment > 0}
                            class:adjust-down={move||attack_data.bonus_penalty_adjustment < 0}
                        >{
                            move || att_bonus_text.clone()
                        }</div>
                        <div
                            class:adjust-up={move||attack_data.damage_bonus_penalty > 0}
                            class:adjust-down={move||attack_data.damage_bonus_penalty < 0}
                        >{
                            move || full_damage_text.clone()
                        }</div>
                    </div>
                </div>
                <Show when={let traits = weapon.traits.clone(); move||traits.len() > 0 || !collapsed_signal()}>
                    <hr style="margin-bottom: 5px;"/>
                </Show>  
                <TraitView trait_names=weapon.traits.clone()/>
                <Show when={let desc = weap_description_clone.clone(); move||!collapsed_signal() && desc.len() > 0}>
                    <p inner_html={let desc = weap_description_clone.clone(); move|| desc.clone()}/>
                </Show>
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
    let tactics_memo = create_memo(move|_|character_data.with(|c|c.tactics.clone()));
    let count_tactics = move || {
        tactics_memo.with(|t|t.iter().filter(|tactic| tactic.selected).count())
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
                    each=move || tactics_memo.get()
                    key=|tactic| tactic.name.clone()
                    children=move |tactic| {
                        let tac_name = tactic.name.clone();
                        let collapse = create_rw_signal(false);
                        let get_selected_on_tactic = {
                            let tac_name2 = tactic.name.clone();
                            move || tactics_memo.with(|tactics|{
                                tactics.iter().find(|val| val.name == tac_name2).expect("TacticView: get_selected - There should be a tactic of the same name").selected
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
                                    let char_tactic: &mut Tactic = c.tactics.iter_mut().find(|val| val.name == tac_name2)
                                        .expect("TacticView: update_selected_on_tactic - There should be a tactic of the same name"); 
                                    char_tactic.selected = !char_tactic.selected
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