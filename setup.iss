; Script Inno Setup para NX NEXTECH PDV
[Setup]
AppId={{D1A3E2F4-5678-90AB-CDEF-1234567890AB}}
AppName=NX NEXTECH PDV
AppVersion=0.1.0
AppPublisher=NX NEXTECH
AppPublisherURL=https://instagram.com/celulares_nextech
DefaultDirName={autopf}\NX NEXTECH
DefaultGroupName=NX NEXTECH
OutputDir=dist
OutputBaseFilename=Instalador_NX_NEXTECH_v0.1.0
SetupIconFile=assets\icon.ico
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
LanguageDetectionMethod=none
DefaultLanguageName=Portuguese

[Languages]
Name: "portuguese"; MessagesFile: "compiler:Languages\Portuguese.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "target\x86_64-pc-windows-msvc\release\nextech.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\NX NEXTECH PDV"; Filename: "{app}\nextech.exe"; IconFilename: "{app}\nextech.exe"
Name: "{autodesktop}\NX NEXTECH PDV"; Filename: "{app}\nextech.exe"; Tasks: desktopicon; IconFilename: "{app}\nextech.exe"

[Run]
Filename: "{app}\nextech.exe"; Description: "{cm:LaunchProgram,NX NEXTECH PDV}"; Flags: nowait postinstall skipifsilent
