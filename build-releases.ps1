#If logs directory does not exist
if (-not (Test-Path ./target/logs -PathType Container)) {
    #Create logs folder
    New-Item -Path ./target/logs -ItemType Directory -Force;
}

#If executables directory does not exist
if (-not (Test-Path ./target/executables -PathType Container)) {
    #Create executables folder
    New-Item -Path ./target/executables -ItemType Directory -Force;
}

#Check if there are files in the executables directory
$files = Get-ChildItem -Path ./target/executables -Include * -File -Recurse | Measure-Object
#If there is
if ($files.Count -gt 0) {
    #Remove old executables
    Get-ChildItem -Path ./target/executables -Include * -File -Recurse | ForEach-Object { $_.Delete()};
    Write-Output "Deleted old executables.";
} else {
    #Do nothing
    Write-Output "No old executables.";
}

#Build x86_64 Windows executable
& {cargo build --target x86_64-pc-windows-msvc --release} 2>&1 | Out-String -OutVariable x64WindowsLog | Out-Null;
$x64WindowsLog | Out-File -FilePath ./target/logs/x86_64-windows-build.log;

if(-not $LASTEXITCODE -ne 0) {
    Copy-Item -Path ./target/x86_64-pc-windows-msvc/release/squid-vm.exe -Destination ./target/executables/squid-vm-x86_64-pc-windows-msvc.exe
    Write-Output "x86_64-pc-windows-msvc built successfully!";
} else {
    Write-Output "x86_64-pc-windows-msvc building process failed! Check x86_64-windows-build.log file";
}

#Build x86_64 Linux executable
& {cross build --target x86_64-unknown-linux-gnu --release} 2>&1 | Out-String -OutVariable x64LinuxLog | Out-Null;
$x64LinuxLog | Out-File -FilePath ./target/logs/x86_64-linux-build.log;

if (-not $LASTEXITCODE -ne 0) {
    Copy-Item -Path ./target/x86_64-unknown-linux-gnu/release/squid-vm -Destination ./target/executables/squid-vm-x86_64-unknown-linux-gnu
    Write-Output "x86_64-unknown-linux-gnu built successfully!";
} else {
    Write-Output "x86_64-unknown-linux-gnu building process failed! Check x86_64-linux-build.log file";
}
#Build aarch64 Linux executable
& {cross build --target aarch64-unknown-linux-gnu --release} 2>&1 | Out-String -OutVariable aarch64LinuxLog | Out-Null;
$aarch64LinuxLog | Out-File -FilePath ./target/logs/aarch64-linux-build.log;

if (-not $LASTEXITCODE -ne 0) {
    Copy-Item -Path ./target/aarch64-unknown-linux-gnu/release/squid-vm -Destination ./target/executables/squid-vm-aarch64-unknown-linux-gnu
    Write-Output "aarch64-unknown-linux-gnu built successfully!";
} else {
    Write-Output "aarch64-unknown-linux-gnu building process failed! Check aarch64-linux-build.log file";
}
#Build armv7hf Linux executable
& {cross build --target armv7-unknown-linux-gnueabihf --release} 2>&1 | Out-String -OutVariable armv7LinuxLog | Out-Null;
$armv7LinuxLog | Out-File -FilePath ./target/logs/armv7-linux-build.log;

if (-not $LASTEXITCODE -ne 0) {
    Copy-Item -Path ./target/armv7-unknown-linux-gnueabihf/release/squid-vm -Destination ./target/executables/squid-vm-armv7-unknown-linux-gnueabihf
    Write-Output "armv7-unknown-linux-gnueabihf built successfully!";
} else {
    Write-Output "armv7-unknown-linux-gnueabihf building process failed! Check armv7-linux-build.log file";
}

#Done!
Write-Output "Done!";