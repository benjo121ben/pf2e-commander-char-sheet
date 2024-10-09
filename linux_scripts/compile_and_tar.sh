#!/usr/bin/env sh
cp Cargo_linux.toml Cargo.toml &&
cargo leptos build --release &&
t_dir=pf2e-sheet &&
rm -r $t_dir
mkdir $t_dir &&
mkdir $t_dir/saves &&
mkdir $t_dir/dev_saves &&
mkdir $t_dir/resources &&
cp -r target/site/. $t_dir/site/ &&
mv $t_dir/site/pkg/pf2e-char-sheet.wasm $t_dir/site/pkg/pf2e-char-sheet_bg.wasm &&
cp -r resources/. $t_dir/resources/ &&
cp -r saves/. $t_dir/dev_saves/ &&
cp target/armv7-unknown-linux-gnueabihf/release/pf2e-char-sheet $t_dir/pf2e-char-sheet &&
cp Cargo_deploy.toml $t_dir/Cargo.toml &&
tar czf pf2e-app.tar.gz -C $t_dir . &&
echo project built and zipped into tar file

