# This software is nowhere near finished. A lot of important UI is still missing

If you're just interested in running the release, download the zip, unzip and create a saves folder inside the app folder. An example for how the character data should look like is in the dev_saves folder.
A lot of stuff still needs to be manually added to the json and cannot yet be edited in the browser.
# how to get this code running

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
 - create folder app/saves
 - run executable

once that is done, you can access the software in any browser at localhost on port 45000 or the port you configure inside the Cargo_deploy.toml/Cargo.toml
