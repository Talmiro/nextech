[Setup]
AppName=NX NEXTECH
AppVersion=0.1.0
DefaultDirName={autopf}\NX NEXTECH
DefaultGroupName=NX NEXTECH
UninstallDisplayIcon={app}\nextech.exe
Compression=lzma2
SolidCompression=yes
OutputDir=dist
OutputBaseFilename=Instalador_NX_NEXTECH_v0.1.0
SetupIconFile=assets\icon.ico

[Languages]
Name: "brazilianportuguese"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"

[Files]
Source: "target\x86_64-pc-windows-msvc\release\nextech.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\NX NEXTECH"; Filename: "{app}\nextech.exe"
Name: "{autodesktop}\NX NEXTECH"; Filename: "{app}\nextech.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\nextech.exe"; Description: "{cm:LaunchProgram,NX NEXTECH}"; Flags: nowait postinstall skipifsilent
