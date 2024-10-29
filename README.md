# how to get this shit running

you can either: 
- install cargo leptos
- run "cargo leptos watch"

or for the release build:

 - install cargo leptos
 - run "cargo leptos build --release"
 - create /app folder
 - copy the executable into app folder from target/release folder 
 - copy the "Cargo_deploy.toml" file into app folder with the name "Cargo.toml"
 - copy target/site folder into app/site
 - copy resources folder into app/resources
 - run executable
