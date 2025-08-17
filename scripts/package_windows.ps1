# Build Windows app with Flutter and package into an executable installer.
# Requires tooling like MSIX or Inno Setup.
$AppDir = "apps/desktop"
Push-Location $AppDir
flutter build windows
Pop-Location
# Placeholder packaging step
Write-Host "Package Windows executable using your preferred installer tool"
