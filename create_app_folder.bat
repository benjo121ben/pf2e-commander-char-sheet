rd "%~dp0\pf2e_app" /q /s
mkdir "%~dp0\pf2e_app"
copy "%~dp0\target\release\pf2e-char-sheet.exe" "%~dp0\pf2e_app\pf2e-char-sheet.exe" /Y
copy "%~dp0\Cargo_deploy.toml" "%~dp0\pf2e_app\Cargo.toml" /Y
xcopy "%~dp0\target\site\*.*" "%~dp0\pf2e_app\site\" /sy
xcopy "%~dp0\resources\*.*" "%~dp0\pf2e_app\resources\" /sy
xcopy "%~dp0\saves\*.*" "%~dp0\pf2e_app\dev_saves\" /sy