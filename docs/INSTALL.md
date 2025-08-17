# Install Options

## Option 1: Download from Releases
- Grab the installer for your OS, run, done.

## Option 2: One-liner
Linux/macOS:
```bash
curl -fsSL https://raw.githubusercontent.com/YOURORG/sovr/main/scripts/install.sh | bash
```

Windows (Admin PowerShell):
```powershell
iwr https://raw.githubusercontent.com/YOURORG/sovr/main/scripts/install.ps1 -UseBasicParsing | iex
```

## Option 3: Build from source
```bash
./scripts/setup_dev.sh
make build-all
```
