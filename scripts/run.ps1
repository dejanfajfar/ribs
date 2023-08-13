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
        
        # Create a lower case log level variable to ease of use
        $lcLogLevel = $logLevel.ToLower()

        # Output the log level used to the stdout
        Write-Host "Set log level to $lcLogLevel"

        $env:RUST_LOG = "$lcLogLevel"
    }
    # If an invalid log level is given the we default to INFO
    else {
        # Output warning information for wrong log level
        Write-Warning "$logLevel is not a valid value. Seting the log level to INFO"
        $env:RUST_LOG = 'info'
    }
}

# Set the backtrace environment variable to see better panick dumps
$env:RUST_BACKTRACE=1

# Set the log level befor running the application
Set-LogLevel

# run the application
# cargo will find the right package even if it is in the super folder
cargo run 