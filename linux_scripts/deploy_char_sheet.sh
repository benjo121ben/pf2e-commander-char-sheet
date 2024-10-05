base_dir=~/programs/pf2e-char-sheet &&
mkdir -p $base_dir &&
mv ./pf2e-app.tar.gz $base_dir/pf2e-app.tar.gz &&
cd $base_dir &&
tar -xf pf2e-app.tar.gz &&
rm pf2e-app.tar.gz &&
./pf2e-char-sheet
