sudo systemctl stop char-sheet.service
base_dir=~/programs/pf2e-char-sheet &&
rm -r $base_dir/site
mkdir -p $base_dir &&
mv ./pf2e-app.tar.gz $base_dir/pf2e-app.tar.gz &&
cd $base_dir &&
tar -xf pf2e-app.tar.gz &&
mkdir -p $base_dir/saves &&
sudo cp /usr/share/fonts/custom/Pathfinder-Icons.tff $base_dir/site/pkg/Pathfinder-Icons.tff &&
sudo chmod o+w $base_dir/site/pkg/Pathfinder-Icons.tff &&
rm pf2e-app.tar.gz &&
sudo systemctl start char-sheet.service &&
echo done with deploy
