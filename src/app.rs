use leptos::*;
use leptos_meta::*;
use std::fmt::Debug;

use crate::views::character_view::BaseView;
use crate::server_side::server_functions::{get_char, get_conditions, get_traits, get_feats};

pub fn try_load_source<T:Clone,D:Clone>(source: Resource<T, Result<D, ServerFnError>>, name: &str) -> Option<Result<D, View>> {
    source.get().map(|ketra| {
        match ketra {
            Ok(ketra_char_data) => Ok(ketra_char_data),
            Err(error) => {
                let err = error.clone();
                Err(view! {
                    <p>Could not load{name.to_string()}<br />{move || err.to_string()}</p>
                }.into_view())
            }
        }
    })
}

pub fn map_resource_ok_to_view<T: Debug>(res: Result<T, View>, name: &str) -> View{
    if res.is_ok() {
        format!("{name} ok").into_view()
    }
    else {
        res.expect_err("app.rs mapresource ok to view expected error")
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let ketra_source = create_resource(||(), move |_| async move { get_char().await });
    let feats_source = create_resource(||(), move |_| async move { get_feats().await });
    let conditions_source = create_resource(||(), move |_| async move { get_conditions().await });
    let traits_source = create_resource(||(), move |_| async move { get_traits().await });
    


    let load_all_data = move || {
        let ketra_res = try_load_source(ketra_source, "Ketra")?;
        let feats_res = try_load_source(feats_source, "Feats")?;
        let traits_res = try_load_source(traits_source, "Traits")?;
        let cond_res = try_load_source(conditions_source, "Conditions")?;

        if ketra_res.is_err() || feats_res.is_err() || traits_res.is_err() || cond_res.is_err() {
            Some(Err(view! {
                {map_resource_ok_to_view(ketra_res, "Ketra")}
                {map_resource_ok_to_view(feats_res, "Feats")}
                {map_resource_ok_to_view(traits_res, "Traits")}
                {map_resource_ok_to_view(cond_res, "Conditions")}
            }))
        }
        else {
            let errmsg = "expected unwrap of source load result to be possible";
            Some(Ok((
                ketra_res.expect(errmsg),
                feats_res.expect(errmsg),
                traits_res.expect(errmsg),
                cond_res.expect(errmsg),
            )))   
        }
    };
    view! {
        <Stylesheet id="google_icons_style" href="https://fonts.googleapis.com/icon?family=Material+Icons"/>
        <Stylesheet id="pf2echar_style" href="/pkg/pf2e-char-sheet.css"/>
        // sets the document title
        <Title text="Pf2e char sheet" />
        <body oncontextmenu="return false;" style="height:inherit">
            <Suspense fallback=move || {
                view! { <p>"Loading ...."</p> }
            }>
                {move || {
                    load_all_data().map(|result|
                        match result {
                            Ok((
                                ketra_char_data,
                                feat_data,
                                trait_data,
                                cond_data
                            )) => view! { <BaseView char=ketra_char_data feats=feat_data conditions=cond_data trait_data=trait_data/> },
                            Err(error) => error.into_view()
                        }
                    )
                }}
            </Suspense>
        </body>
    }
}