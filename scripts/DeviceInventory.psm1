Set-StrictMode -Version Latest

function ConvertTo-DeviceHubRecord {
    [CmdletBinding()]
    param([Parameter(Mandatory)][string]$InstanceId)
    # Parse only documented USB/PnP identifier tokens. Never return the input path.
    if ($InstanceId -notmatch '^(USB|HID)\\') { return }
    if ($InstanceId -notmatch '(?i)VID_([0-9A-F]{4})&PID_([0-9A-F]{4})(?=&|\\|$)') { return }
    $vid = $Matches[1].ToUpperInvariant()
    $pid = $Matches[2].ToUpperInvariant()
    $mi = $null
    $revision = $null
    if ($InstanceId -match '(?i)&MI_([0-9A-F]{2})(?=&|\\|$)') {
        $mi = [Convert]::ToInt32($Matches[1], 16)
    }
    if ($InstanceId -match '(?i)&REV_([0-9A-F]{4})(?=&|\\|$)') {
        $revision = $Matches[1].ToUpperInvariant()
    }
    $vendor = switch ($vid) { '046D' { 'Logitech / Logicool' } default { $null } }
    [pscustomobject][ordered]@{
        vendor_id = $vid
        product_id = $pid
        manufacturer_candidate = $vendor
        product_name = $null
        pnp_revision_token = $revision
        interface_number = $mi
        transport = 'USB PnP (may be a wireless receiver)'
        usage_page = $null
        usage = $null
        firmware = $null
        hardware_revision = $null
        verification = 'unverified'
    }
}

function Get-DeviceHubInventory {
    [CmdletBinding()]
    param()
    # Enumeration only: no HID handle, report, driver update, or keyboard input.
    $devices = @(Get-PnpDevice -PresentOnly -ErrorAction Stop)
    $records = @($devices | ForEach-Object {
        ConvertTo-DeviceHubRecord -InstanceId $_.InstanceId
    })
    # Aggregate identical public identifiers; do not imply physical-device counts.
    $unique = @($records | Sort-Object vendor_id, product_id, interface_number, pnp_revision_token -Unique)
    [pscustomobject][ordered]@{
        schema_version = 1
        source = 'Windows PnP, present devices only'
        identity_scope = 'aggregated public identifiers, not physical device identity'
        devices = $unique
    }
}
Export-ModuleMember -Function ConvertTo-DeviceHubRecord, Get-DeviceHubInventory
