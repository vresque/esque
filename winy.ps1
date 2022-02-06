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
        python y.py $command
        break
    }
    "clean" {
        python y.py $command
        break
    }
    "format" {
        python y.py $command
        break
    }
    "build" {
        python y.py initramfs
        python y.py format
        python y.py build-kernel
        python y.py build-boot
        wsl "python y.py strip; python y.py image"
    }
    "setup" {
        python y.py $command
    }
    default {
        Write-Output "Running $command in WSL"
        wsl "python y.py $command"
        Write-Output "WSL exited..."
    }
}