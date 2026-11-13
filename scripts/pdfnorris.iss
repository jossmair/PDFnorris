; Offline, per-user Windows installer. Only the EXE and the consolidated legal file.
#ifndef AppVersion
  #error AppVersion must be supplied by package-setup.ps1
#endif
[Setup]
AppId={{C37B4976-A267-4A3F-A19B-7BD916526746}
AppName=PDFnorris
AppVerName=PDFnorris V1.01
AppVersion={#AppVersion}
AppPublisher=Ergonatus
AppCopyright=Ergonatus
DefaultDirName={localappdata}\Programs\Ergonatus\PDFnorris
DefaultGroupName=Ergonatus
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0.10240
OutputDir=..\dist
OutputBaseFilename=PDFnorris-{#AppVersion}-Setup
SetupIconFile=..\assets\pdfnorris.ico
UninstallDisplayIcon={app}\PDFnorris.exe
UninstallDisplayName=PDFnorris
CreateUninstallRegKey=not IsSmokeTest
WizardStyle=modern
DisableProgramGroupPage=yes
DisableWelcomePage=no
Compression=lzma2/max
SolidCompression=yes
CloseApplications=no
RestartApplications=no
SetupLogging=yes
VersionInfoVersion={#AppVersion}
VersionInfoDescription=Installation de PDFnorris

[Languages]
Name: french; MessagesFile: compiler:Languages\French.isl

[Tasks]
Name: desktopicon; Description: Créer un raccourci sur le Bureau; Flags: unchecked; Check: not IsSmokeTest

[Files]
Source: ..\runtime\pdfnorris.exe; DestDir: {app}; DestName: PDFnorris.exe; Flags: ignoreversion
Source: ..\licenses\LICENCES.txt; DestDir: {app}; Flags: ignoreversion

[Icons]
Name: {userprograms}\Ergonatus\PDFnorris; Filename: {app}\PDFnorris.exe; IconFilename: {app}\PDFnorris.exe; IconIndex: 0; WorkingDir: {app}; Check: not IsSmokeTest
Name: {userdesktop}\PDFnorris; Filename: {app}\PDFnorris.exe; IconFilename: {app}\PDFnorris.exe; IconIndex: 0; WorkingDir: {app}; Tasks: desktopicon; Check: not IsSmokeTest

[Run]
Filename: {app}\PDFnorris.exe; Description: Ouvrir PDFnorris; Flags: nowait postinstall skipifsilent

[Code]
function IsSmokeTest: Boolean;
begin
  Result := ExpandConstant('{param:TESTINSTALL|0}') = '1';
end;
