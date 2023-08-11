<#
.SYNOPSIS
    Start the API with environment checks

.PARAMETER logLevel
    The max log level written to the stdout

    expected values are:
    - error
    - warn
    - info
    - debug
    - trace

    If an invalid value is given the INFO is assumed

#>
[CmdletBinding()]
param (
    [Parameter(Mandatory = $false)]
    [Alias("ll")]
    [String]
    $logLevel = "info"
)

function local:Set-LogLevel {
    # List of valid log levels
    # https://github.com/rust-cli/env_logger for valid log levels 
    $validLogLevels = 'error', 'warn', 'info', 'debug', 'trace'

    # If the provided log level is valid then we use it
    if ($validLogLevels -contains $logLevel.ToLower()) {
        "Set log level to $logLevel"
        $env:RUST_LOG = $logLevel.ToLower()
    }
    # If an invalid log level is given the we default to INFO
    else {
        "$logLevel is not a valid value. Seting the log level to INFO"
        $env:RUST_LOG = 'info'
    }
}

# Set the log level befor running the application
Set-LogLevel

# run the application
# cargo will find the right package even if it is in the super folder
cargo run 