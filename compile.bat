pyinstaller --icon src/graphics/icon.ico --noconsole --noconfirm src/SquadOfFive.py
echo D|xcopy "./src/graphics" "./dist/SquadOfFive/src/graphics" /s
7z a -tzip ./dist/SquadOfFive.7z ./dist/SquadOfFive