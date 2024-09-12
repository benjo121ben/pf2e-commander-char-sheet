use leptos::*;
use leptos_meta::*;

use crate::views::character_view::CharacterView;
use crate::server_side::server_functions::get_char;


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let ketra_source = create_resource(||(), move |_| async move { get_char().await });
    view! {
        <Stylesheet id="leptos" href="/pkg/pf2e-char-sheet.css" />
        // sets the document title
        <Title text="Welcome to Leptos" />
        <body oncontextmenu="return false;" style="height:inherit">
            <Suspense fallback=move || {
                view! { <p>"Loading ...."</p> }
            }>
                {move || {
                    ketra_source
                        .get()
                        .map(|ketra| {
                            match ketra {
                                Ok(ketra_char_data) => view! { <CharacterView char=ketra_char_data /> },
                                Err(error) => {
                                    view! {
                                        <p>"Could not load Ketra"<br /> {move || error.to_string()}</p>
                                    }
                                        .into_view()
                                }
                            }
                        })
                }}
            </Suspense>
        </body>
    }
}