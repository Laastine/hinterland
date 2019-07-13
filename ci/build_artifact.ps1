mkdir hinterland
mkdir hinterland\assets
copy assets\*.png hinterland\assets
copy assets\*.json hinterland\assets
copy -Recurse assets\maps hinterland\assets\maps
copy -Recurse assets\audio hinterland\assets\audio
copy target\release\hinterland.exe hinterland\hinterland.exe
7z a -tzip hinterland-windows.zip hinterland
