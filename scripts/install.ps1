param([string]$Version = "latest")
$ErrorActionPreference = "Stop"

if ($Version -eq "latest") {
  $resp = Invoke-RestMethod https://api.github.com/repos/YOURORG/sovr/releases/latest
  $Version = $resp.tag_name
}

$asset = "SOVR-windows-x64.exe"
$url = "https://github.com/YOURORG/sovr/releases/download/$Version/$asset"
$out = "$PSScriptRoot\$asset"
Invoke-WebRequest -Uri $url -OutFile $out
Write-Host "Downloaded $out. Run the installer."
