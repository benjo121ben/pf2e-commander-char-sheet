sudo systemctl stop char-sheet.service
base_dir=~/programs/pf2e-char-sheet &&
rm -r $base_dir/site
mkdir -p $base_dir &&
mv ./pf2e-app.tar.gz $base_dir/pf2e-app.tar.gz &&
cd $base_dir &&
tar -xf pf2e-app.tar.gz &&
rm pf2e-app.tar.gz &&
sudo systemctl start char-sheet.service &&
echo done with deploy
