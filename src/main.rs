#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use std::env;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use pf2e_char_sheet::app::*;
    use pf2e_char_sheet::fileserv::file_and_error_handler;

    env::set_var("RUST_BACKTRACE", "1");
    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(Some("./Cargo.toml")).await.expect("Expecting Cargo.toml to be there");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.expect("main.rs expected listener");
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .expect("main.rs expected service");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
