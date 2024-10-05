#!/usr/bin/env sh
cp Cargo_linux.toml Cargo.toml &&
cargo leptos build --release &&
t_dir=pf2e-sheet &&
mkdir -p $t_dir &&
mkdir -p $t_dir/saves &&
cp -r target/site $t_dir/site &&
cp target/armv7-unknown-linux-gnueabihf/release/pf2e-char-sheet $t_dir/pf2e-char-sheet &&
cp Cargo_deploy.toml $t_dir/Cargo.toml &&
cp -r resources $t_dir/resources &&
cp -r saves $t_dir/dev_saves &&
tar czf pf2e-app.tar.gz -C $t_dir . &&
echo project built and zipped into tar file

