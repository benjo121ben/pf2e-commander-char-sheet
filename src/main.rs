mod character;
use core::borrow;

use character::Character;
use html::data;
use leptos::*;
use leptos::logging::log;




fn main() { 
    mount_to_body(move || view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let (read_ketra, write_ketra) = create_signal(Character::new("Ketra"));
    view! {
        <h3>{move || read_ketra().get_name()} Level {move || read_ketra().level}</h3>
        <p>
            This is a test value
        </p> 
        <input on:change={move |event| write_ketra.update(|f| {
            let val: String = event_target_value(&event);
            f.set_name(&val);
            log!("{event:?}");
        })}/>
    }
}
