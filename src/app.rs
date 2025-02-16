use leptos::prelude::*;
use leptos_meta::*;
use std::fmt::Debug;

use crate::views::character_view::BaseView;
use crate::server_side::server_functions::{get_char, get_conditions, get_traits, get_feats};

pub fn try_load_source<T:Clone + std::fmt::Debug + std::marker::Sync + std::marker::Send + 'static>(source: OnceResource<Result<T, ServerFnError>>, name: &str) -> Option<Result<T, View<String>>> {
    source.get().map( |result| {
        if result.is_err() {
            let err = result.unwrap_err().clone();
            return Err(format!("Could not load {0} from server<br/>{1}</p>", name.to_string(), err.to_string()).into_view())
        }

        return Ok(result.unwrap())
    })
}

pub fn map_resource_ok_to_view<T: Debug>(res: Result<T, View<String>>, name: &str) -> View<String>{
    if res.is_ok() {
        format!("{name} ok").into_view()
    }
    else {
        res.expect_err("app.rs mapresource ok to view expected error")
    }
}

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
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let ketra_source = OnceResource::new(get_char());
    let feats_source = OnceResource::new(get_feats());
    let conditions_source = OnceResource::new(get_conditions());
    let traits_source = OnceResource::new(get_traits());
    


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
                            )) => view! { <BaseView char=ketra_char_data feats=feat_data conditions=cond_data trait_data=trait_data/> }.into_any(),
                            Err(error) => error.into_any()
                        }
                    )
                }}
            </Suspense>
        </body>
    }
}