<#
.SYNOPSIS
    Simplified start script

.PARAMETER environment
    The environment for which the start command should be execuded.
    
    Default "DEV" is assumed
.NOTES
    The folowing error codes are returned in case of an error:
    - -1 -> Docker not found on the system
    - -2 -> The provided environment identifier is invalid
#>
[CmdletBinding()]
param (
    [Parameter(Mandatory = $false)]
    [Alias("e")]
    [String]
    $environment = "dev"
)


function Start-Development {
    Write-Header "Starting development env"

    docker run --rm --pull always --name surrealdb -p 8000:8000 surrealdb/surrealdb:latest start --log trace --user root --pass root memory
}

function Start-Test {
    Write-Header "Starting Test Environment"

    docker compose  -f $PSScriptRoot\..\docker-compose.yml up
}

function local:Write-Header {
    param (
        [string]$text
    )
    '*' * 80
    "{0,49}" -f $text
    '*' * 80
}

function local:Write-Warning {
    param (
        [string]$text
    )
    '`a' # ring the terminal bell -> https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_special_characters?view=powershell-7.3
    '!' * 80
    "{0,49}" -f $text
    '!' * 80
}

# Determine if docker is installed on the system
# For this we call the docker version command and ignore the output
# After that we take a look at the exit code of the last command run and expect it to not be an error
docker version | Out-Null
if (!$?){
    Write-Warning "Docker not installed"
    Exit -1
}


switch ($environment.ToLower()) {
    "dev" { Start-Development }
    "test" { Start-Test }
    Default {
        Write-Warning "Environment $environment is not known...Valid values are: dev, test"
        Exit -2
    }
}
