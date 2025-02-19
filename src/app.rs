use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use std::collections::HashMap;
use std::fmt::Debug;

use crate::char_data::character::Character;
use crate::char_data::conditions::ConditionData;
use crate::char_data::feats::Feat;
use crate::views::character_view::BaseView;
use crate::server_side::server_functions::{get_char, get_conditions, get_traits, get_feats};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body oncontextmenu="return false;" style="height:inherit">
                <App/>
            </body>
        </html>
    }.into_any()
}

pub async fn try_load_source<T:Clone + std::fmt::Debug + std::marker::Sync + std::marker::Send + 'static>(source: OnceResource<Result<T, ServerFnError>>, name: &str) -> Result<T, View<String>> {
    let result = source.await;
    if result.is_err() {
        let err = result.unwrap_err().clone();
        return Err(format!("Could not load {0} from server<br/>{1}</p>", name.to_string(), err.to_string()).into_view())
    }

    return Ok(result.unwrap())
}

pub fn map_resource_ok_to_view<T: Debug>(res: Result<T, View<String>>, name: &str) -> View<String>{
    if res.is_ok() {
        format!("{name} ok").into_view()
    }
    else {
        res.expect_err("app.rs mapresource ok to view expected error")
    }
}

pub async fn load_all_data(
        ketra_source: OnceResource<Result<Character, ServerFnError>>, 
        feats_source: OnceResource<Result<Vec<Feat>, ServerFnError>>, 
        conditions_source: OnceResource<Result<Vec<ConditionData>, ServerFnError>>,
        traits_source: OnceResource<Result<HashMap<String, String>, ServerFnError>>
    ) -> Result<(Character, Vec<Feat>, Vec<ConditionData>, HashMap<String, String>),AnyView> {
    let ketra_res = try_load_source(ketra_source, "Ketra").await;
    let feats_res = try_load_source(feats_source, "Feats").await;
    let cond_res = try_load_source(conditions_source, "Conditions").await;
    let traits_res = try_load_source(traits_source, "Traits").await;

    if ketra_res.is_err() || feats_res.is_err() || traits_res.is_err() || cond_res.is_err() {
        Err(view! {
            {map_resource_ok_to_view(ketra_res, "Ketra")}
            {map_resource_ok_to_view(feats_res, "Feats")}
            {map_resource_ok_to_view(cond_res, "Conditions")}
            {map_resource_ok_to_view(traits_res, "Traits")}
        }.into_any())
    }
    else {
        let errmsg = "expected unwrap of source load result to be possible";
        Ok((
            ketra_res.expect(errmsg),
            feats_res.expect(errmsg),
            cond_res.expect(errmsg),
            traits_res.expect(errmsg),
        ))   
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let ketra_source: OnceResource<Result<Character, ServerFnError>> = OnceResource::new(get_char());
    let feats_source: OnceResource<Result<Vec<Feat>, ServerFnError>> = OnceResource::new(get_feats());
    let conditions_source: OnceResource<Result<Vec<ConditionData>, ServerFnError>> = OnceResource::new(get_conditions());
    let traits_source: OnceResource<Result<HashMap<String, String>, ServerFnError>> = OnceResource::new(get_traits());

    view! {
        <Stylesheet id="google_icons_style" href="https://fonts.googleapis.com/icon?family=Material+Icons"/>
        <Stylesheet id="pf2echar_style" href="/pkg/pf2e-char-sheet.css"/>
        // sets the document title
        <Title text="Pf2e char sheet" />
        <Suspense 
            fallback=move || {view! { <p>"Loading ...."</p> }}
        >
            {move || Suspend::new( async move {
                let data = load_all_data(ketra_source, feats_source, conditions_source, traits_source).await;
                match data {
                    Ok((
                        ketra_char_data,
                        feat_data,
                        cond_data,
                        trait_data
                    )) => view! { <BaseView char=ketra_char_data feats=feat_data conditions=cond_data trait_data=trait_data/> }.into_any(),
                    Err(error) => error.into_any()
                }
            })}
        </Suspense>
    }.into_any()
}