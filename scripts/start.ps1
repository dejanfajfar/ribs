<#
.SYNOPSIS
    Simplified start script

.PARAMETER environment
    The environment for which the start command should be execuded.
    
    Default "DEV" is assumed
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

function Write-Header {
    param (
        $text
    )
    
    '*' * 80
    "{0,49}" -f $text
    '*' * 80
}

switch ($environment.ToLower()) {
    "dev" { Start-Development }
}
