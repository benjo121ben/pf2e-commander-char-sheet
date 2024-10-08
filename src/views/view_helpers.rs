use leptos::{use_context, ReadSignal, RwSignal, WriteSignal};

use crate::{char_data::character::Character, error_template::SheetError};

pub fn get_base_context(view_name: &str) -> (ReadSignal<Character>, WriteSignal<Character>){
    let name = String::from(view_name);
    (
        use_context::<ReadSignal<Character>>().expect(&format!("{name}: Expect Char read to be set")),
        use_context::<WriteSignal<Character>>().expect(&format!("{name}: Expect Char write to be set")),
    )
}

pub fn get_sheet_error_context(view_name: &str) -> RwSignal<SheetError> {
    let name = String::from(view_name);
    use_context::<RwSignal<SheetError>>().expect(&format!("{name}: Expect error rw to be set"))
}

pub fn get_prefix(val:i32) -> String {
    if val > 0 {"+"} else {""}.to_string()
}