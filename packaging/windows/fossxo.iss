; FossXO Inno Setup script for creating Windows a installer.
; Note: this script requires some environmental variables to
; be set; the cargo make package command does ths automatically.

#define NAME "FossXO"
#define DESCRIPTION GetEnv("FOSSXO_DESCRIPTION")
#define EXE_NAME "fossxo.exe"
#define VERSION GetEnv("FOSSXO_VERSION")
#define HOMEPAGE GetEnv("FOSSXO_HOMEPAGE")
#define PROJECT_DIR GetEnv("CRATE_ROOT_DIR")
#define TARGET_DIR AddBackslash(GetEnv("CRATE_ROOT_DIR")) + "target"

[Setup]
AppName={#NAME}
AppVersion={#VERSION}
AppPublisherURL={#HOMEPAGE}

WizardStyle=modern
OutputBaseFilename="{#NAME}-setup"
OutputDir="{#TARGET_DIR}"

; Support installing in non-admin mode
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
UsePreviousPrivileges=yes

DefaultDirName="{autopf}\{#NAME}"
DefaultGroupName="{#NAME}"

[Files]
Source: "{#TARGET_DIR}\release\{#EXE_NAME}"; DestDir: "{app}"
Source: "{#PROJECT_DIR}\config\*"; DestDir: "{app}\config"; Flags: ignoreversion
; Uncomment the following when adding assets
; Source: "{#PROJECT_DIR}\assets\*"; DestDir: "{app}\assets"; Flags: ignoreversion recursesubdirs

; User manual files
; Source: "{#TARGET_DIR}\user-manual\*"; DestDir: "{app}\user-manual"; Flags: ignoreversion recursesubdirs

[Icons]
Name: "{group}\{#NAME}"; Filename: "{app}\{#EXE_NAME}"; Comment: "{#DESCRIPTION}"
; Name: "{group}\{#NAME} User Manual"; Filename: "{app}\user-manual\index.html"
Name: "{autodesktop}\{#NAME}"; Filename: "{app}\{#EXE_NAME}"; Comment: "{#DESCRIPTION}"; Tasks: desktopicon

[Tasks]
Name: desktopicon; Description: "Create a &desktop icon"

[run]
Filename: "{app}\{#EXE_NAME}"; Description: "Start {#NAME}"; Flags: postinstall nowait skipifsilent unchecked


; Uncomment the following line to get the preprocessed output of this
; script --- useful for debugging.
; #expr SaveToFile(AddBackslash(SourcePath) + "fossxo.preprocessed.iss")
