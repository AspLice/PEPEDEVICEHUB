Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
Import-Module (Join-Path $PSScriptRoot 'DeviceInventory.psm1') -Force
function Assert-True([bool]$Condition, [string]$Message) {
    if (-not $Condition) { throw $Message }
}
$checks = 0
$a = ConvertTo-DeviceHubRecord 'HID\VID_046D&PID_C094&MI_02\PRIVATE_SERIAL'
Assert-True ($a.vendor_id -eq '046D' -and $a.product_id -eq 'C094') 'VID/PID'; $checks++
Assert-True ($a.interface_number -eq 2) 'Interface'; $checks++
Assert-True ($null -eq $a.product_name -and $a.verification -eq 'unverified') 'No model inference'; $checks++
Assert-True ($null -eq $a.firmware -and $null -eq $a.hardware_revision) 'Unknown metadata'; $checks++
$b = ConvertTo-DeviceHubRecord 'usb\vid_abcd&pid_1234&rev_0200\ANOTHER_PRIVATE_SERIAL'
Assert-True ($b.vendor_id -eq 'ABCD' -and $b.pnp_revision_token -eq '0200') 'Case and revision'; $checks++
Assert-True ($null -eq $b.interface_number -and $null -eq $b.manufacturer_candidate) 'Unknown vendor'; $checks++
foreach ($id in @('ROOT\VID_046D&PID_C094\SECRET','HID\VID_ZZZZ&PID_C094\SECRET','HID\VID_046D&PID_C0945\SECRET')) {
    Assert-True ($null -eq (ConvertTo-DeviceHubRecord $id)) 'Reject malformed or unrelated identifier'; $checks++
}
$json = @($a, $b) | ConvertTo-Json -Depth 5
Assert-True ($json -notmatch 'PRIVATE|SERIAL|InstanceId|DevicePath') 'Private identity is not exported'; $checks++
# Replace the OS provider inside module scope to test aggregation deterministically.
$module = Get-Module DeviceInventory
& $module {
    function script:Get-PnpDevice {
        param([switch]$PresentOnly)
        if (-not $PresentOnly) { throw 'Expected present-only enumeration' }
        [pscustomobject]@{ InstanceId = 'HID\VID_046D&PID_C094&MI_02\SECRET_A' }
        [pscustomobject]@{ InstanceId = 'HID\VID_046D&PID_C094&MI_02\SECRET_B' }
        [pscustomobject]@{ InstanceId = 'ROOT\UNRELATED' }
    }
}
$inventory = Get-DeviceHubInventory
Assert-True ($inventory.devices.Count -eq 1) 'Aggregate public identifiers'; $checks++
Assert-True ($inventory.schema_version -eq 1) 'Schema'; $checks++
Remove-Module DeviceInventory
# Real OS enumeration validates the PnpDevice integration on the Windows runner.
$output = & (Join-Path $PSScriptRoot 'Get-DeviceInventory.ps1')
$parsed = ($output -join "`n") | ConvertFrom-Json
Assert-True ($parsed.schema_version -eq 1 -and $parsed.source -like 'Windows PnP*') 'Native enumeration'; $checks++
Write-Output "$checks inventory checks passed. Native runner enumeration is not target-device verification."
