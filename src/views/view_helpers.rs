use std::collections::HashMap;

use leptos::{use_context, Memo, ReadSignal, RwSignal, WriteSignal};

use crate::{char_data::{bonus_penalty::StatBonusPenalties, character::Character, conditions::FullConditionView}, error_template::SheetError};

use super::info_modal_view::SimpleModalData;

pub fn get_base_context(view_name: &str) -> (ReadSignal<Character>, WriteSignal<Character>){
    let name = String::from(view_name);
    (
        use_context::<ReadSignal<Character>>().expect(&format!("{name}: Expect Char read to be set")),
        use_context::<WriteSignal<Character>>().expect(&format!("{name}: Expect Char write to be set")),
    )
}

pub fn get_all_conditions_vector_memo_from_context(view_name: &str) -> Memo<Vec<FullConditionView>> {
    use_context::<Memo<Vec<FullConditionView>>>().expect(&format!("{view_name}: Expect conditions to be added to context"))
}

pub fn get_bonus_penalty_map_from_context(view_name: &str) ->Memo<HashMap<String, StatBonusPenalties>> {
    use_context::<Memo<HashMap<String, StatBonusPenalties>>>().expect(&format!("{view_name}: Expect bonus_penalties to be calculated"))
}

pub fn get_sheet_error_context(view_name: &str) -> RwSignal<SheetError> {
    let name = String::from(view_name);
    use_context::<RwSignal<SheetError>>().expect(&format!("{name}: Expect error rw to be set"))
}

pub fn get_modal_context(view_name: &str) -> RwSignal<SimpleModalData> {
    let name = String::from(view_name);
    use_context::<RwSignal<SimpleModalData>>().expect(&format!("{name}: Expect modal rw to be set"))
}

pub fn pretty_print_int(val: i32) -> String {
    format!("{0}{1}", get_prefix(val), val)
}

pub fn get_prefix(val:i32) -> String {
    if val > 0 {"+"} else {""}.to_string()
}

pub fn check_character_flag(character: &Character, flag: &str) -> bool{
    match character.flags.get(flag) {
        Some(flag_v) => *flag_v,
        None => false,
    }
    
}