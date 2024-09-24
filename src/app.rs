use leptos::*;
use leptos_meta::*;

use crate::char_data::traits::Trait;
use crate::views::character_view::CharacterView;
use crate::server_side::server_functions::{get_char, get_conditions, get_traits};


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let conditions_source = create_resource(||(), move |_| async move { get_conditions().await });
    let ketra_source = create_resource(||(), move |_| async move { get_char().await });
    let traits_source = create_resource(||(), move |_| async move { get_traits().await });
    view! {
        <Stylesheet id="leptos" href="/pkg/pf2e-char-sheet.css" />
        // sets the document title
        <Title text="Welcome to Leptos" />
        <body oncontextmenu="return false;" style="height:inherit">
            <Suspense fallback=move || {
                view! { <p>"Loading ...."</p> }
            }>
                {move || {
                    conditions_source.get().map(|conditions_result|
                        match conditions_result {
                            Ok(conditions) => {
                                traits_source.get().map(|t| {
                                    match t {
                                        Ok(trait_data) => {
                                            ketra_source.get().map(|ketra| {
                                                match ketra {
                                                    Ok(ketra_char_data) => view! { <CharacterView char=ketra_char_data conditions=conditions trait_data=trait_data/> },
                                                    Err(error) => {
                                                        let err = error.clone();
                                                        view! {
                                                            <p>"Could not load Ketra"<br /> {move || err.to_string()}</p>
                                                        }.into_view()
                                                    }
                                                }
                                            })
                                        },
                                        Err(error) => {
                                            let err = error.clone();
                                            Some(view! {
                                                <p>"Could not load Traits"<br /> {move || err.to_string()}</p>
                                            }.into_view())
                                        }
                                    }
                                })
                                
                            },
                            Err(error) => {
                                Some(Some(view! {
                                    <p>"Could not load Conditions"<br /> {move || error.to_string()}</p>
                                }.into_view()))
                            }
                        }
                    )   
                }}
            </Suspense>
        </body>
    }
}