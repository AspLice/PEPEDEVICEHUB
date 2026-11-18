# Run from a normal Windows PowerShell session; no elevation required.
[CmdletBinding()]
param()
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
try {
    Import-Module (Join-Path $PSScriptRoot 'DeviceInventory.psm1') -Force
    Get-DeviceHubInventory | ConvertTo-Json -Depth 5
} catch {
    # Do not echo provider exceptions: they can contain private instance paths.
    [Console]::Error.WriteLine('Device inventory failed. Use Windows 11 with the PnpDevice module; no raw device details were exported.')
    exit 1
}
