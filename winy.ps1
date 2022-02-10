Param (
    [string] $command,
    [Parameter(
        ValueFromRemainingArguments=$true
    )][string[]]
    $otherArgs
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
        # If we exit before this, the ErrorActionPreference will not be reset
        Exit
    }
}

Function Check-Retval {
    if (-Not ($?)) {
        Exit
    }
}

# Otherwise, the condition below will often be triggered
if ($command -eq "setup") {
    python y.py $command $otherArgs 
    Exit
}

if (-Not ( Test-Path -Path "build")) {
    "Please run ./winy setup first"
    Exit
}

Test-IfCommandExists "wsl"
Test-IfCommandExists "cargo"
Test-IfCommandExists "rustc"

switch ($command) {
    "run" {
        Test-IfCommandExists "qemu-system-x86-64"
        python y.py $command $otherArgs 
        break
    }
    "clean" {
        python y.py $command $otherArgs 
        break
    }
    "format" {
        python y.py $command $otherArgs 
        break
    }
    "build" {
        python y.py initramfs $otherArgs
        Check-Retval
        python y.py format $otherArgs
        Check-Retval
        python y.py build-kernel $otherArgs
        Check-Retval
        python y.py build-boot $otherArgs
        Check-Retval
        wsl python y.py strip $otherArgs
        Check-Retval
        wsl python y.py image $otherArgs
        Check-Retval
    }
    "setup" {
        python y.py $command $otherArgs 
    }
    "wsl" {
        wsl python y.py $otherArgs
    }
    default {
        Write-Output "Running $command with $otherArgs in WSL"
        wsl python y.py $command $otherArgs 
        Write-Output "WSL exited..."
    }
}