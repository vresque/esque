Param (
    [string] $command
)

Function Test-IfCommandExists
{
    Param([string] $commandName)
    $oldPreference = $ErrorActionPreference
    $shouldExitAfterwards = 1
    $ErrorActionPreference = "stop"
    try {
        if (Get-Command $commandName) {
            "$commandName was found"
        }
    } Catch {
        "$commandName was not found. Please install it"
        $shouldExitAfterwards = 0
    }
    Finally {
        $ErrorActionPreference = $oldPreference
    }
    if ($shouldExitAfterwards -eq "0") {
        Exit
    }
}

if ( Test-Path -Path "build" -ne "True" ) {
    "Please run ./winy setup first"
    Exit
}

Test-IfCommandExists "wsl"
Test-IfCommandExists "cargo"
Test-IfCommandExists "rustc"

switch ($command) {
    "run" {
        Test-IfCommandExists "qemu-system-x86-64"
        ./y.py $command
        break
    }
    "clean" {
        ./y.py $command
        break
    }
    "format" {
        ./y.py $command
        break
    }
    "build" {
        ./y.py initramfs
        ./y.py format
        ./y.py build-kernel
        ./y.py build-boot
        wsl "python y.py strip; python y.py image"
    }
    "setup" {
        ./y.py $command
    }
    default {
        Write-Output "Running $command in WSL"
        wsl "python y.py $command"
        Write-Output "WSL exited..."
    }
}