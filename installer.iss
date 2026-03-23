; 流量监控 - Inno Setup 安装脚本
; 工具下载: https://jrsoftware.org/isinfo.php

#define AppName "流量监控"
#define AppVersion "1.0.0"
#define AppPublisher "lsw"
#define AppExeName "流量监控.exe"

[Setup]
AppId={{E3A2F1B4-7C8D-4E9F-A123-56789ABCDEF0}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
AppPublisherURL=https://github.com
DefaultDirName={autopf}\TrafficMonitor
DefaultGroupName={#AppName}
AllowNoIcons=yes
OutputDir=dist\installer
OutputBaseFilename=流量监控-v{#AppVersion}-安装程序
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
PrivilegesRequiredOverridesAllowed=dialog
UninstallDisplayName={#AppName}
UninstallDisplayIcon={app}\{#AppExeName}
MinVersion=10.0
ArchitecturesInstallIn64BitMode=x64
ArchitecturesAllowed=x64

[Languages]
Name: "chinesesimp"; MessagesFile: "compiler:Languages\ChineseSimplified.isl"

[Tasks]
Name: "desktopicon";    Description: "创建桌面快捷方式";    GroupDescription: "附加快捷方式:"; Flags: unchecked
Name: "startupicon";   Description: "开机自动启动";        GroupDescription: "附加快捷方式:"; Flags: unchecked

[Files]
Source: "target\release\traffic-monitor.exe"; DestDir: "{app}"; DestName: "{#AppExeName}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#AppName}";            Filename: "{app}\{#AppExeName}"
Name: "{group}\卸载 {#AppName}";        Filename: "{uninstallexe}"
Name: "{commondesktop}\{#AppName}";    Filename: "{app}\{#AppExeName}"; Tasks: desktopicon

[Registry]
; 开机自启
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; \
  ValueType: string; ValueName: "{#AppName}"; \
  ValueData: """{app}\{#AppExeName}"""; \
  Flags: uninsdeletevalue; Tasks: startupicon

[Run]
Filename: "{app}\{#AppExeName}"; \
  Description: "立即运行 {#AppName}"; \
  Flags: nowait postinstall skipifsilent

[UninstallRun]
Filename: "taskkill"; Parameters: "/f /im ""{#AppExeName}"""; Flags: runhidden; RunOnceId: "KillApp"

[Code]
// 安装前先结束正在运行的旧版本
procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssInstall then begin
    Exec('taskkill', '/f /im "流量监控.exe"', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
  end;
end;
