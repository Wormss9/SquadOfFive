pyinstaller --icon graphics/icon.ico --noconsole --noconfirm SquadOfFive.py
echo D|xcopy "./graphics" "./dist/SquadOfFive/graphics" /s
7z a -tzip ./dist/SquadOfFive.7z ./dist/SquadOfFive