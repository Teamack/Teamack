# Commons Engine Prototype

This repository hosts a minimal peer-to-peer resource and skill exchange. It spins up a [libp2p](https://libp2p.io/) node that can exchange JSON messages with other peers. Messages are end-to-end encrypted using AES-256-GCM and acknowledged by the receiver. The goal is to explore infrastructure for mutual aid without central control.

## 🚀 Quickstart (2 minutes)

### Desktop (macOS/Windows/Linux)
1) Go to **Releases** → download your OS build:
   - macOS: `SOVR-macos-universal.dmg`
   - Windows: `SOVR-windows-x64.exe`
   - Linux: `SOVR-linux-x64.AppImage`
2) Open SOVR → **Settings → Create Identity** (prints your fingerprint).
3) **Devices → Link New Device** (scan the QR on your phone/second laptop).
4) **Vault → Import Pack** and choose `sovr-starter-pack.zip`.
5) **Feed → Compose** to post your first local message.

### Headless Relay (Raspberry Pi 4/5)
```bash
# Download artifact for aarch64 from Releases
wget https://github.com/YOURORG/sovr/releases/download/v0.1.0/sovr-headless-linux-aarch64.tar.gz
tar xzf sovr-headless-linux-aarch64.tar.gz
sudo ./sovr-headless mesh join --mdns --data-dir /var/lib/sovr
```

Verify downloads (optional, recommended)
```bash
# Download checksums and signature from the Release page
sha256sum -c SHA256SUMS
minisign -Vm SHA256SUMS -P YOUR_MINISIGN_PUBKEY
```

**Linux/macOS**
```bash
curl -fsSL https://raw.githubusercontent.com/YOURORG/sovr/main/scripts/install.sh | bash
```

**Windows (PowerShell as Admin)**
```powershell
iwr https://raw.githubusercontent.com/YOURORG/sovr/main/scripts/install.ps1 -UseBasicParsing | iex
```

## Getting Started

1. Install dependencies:

```bash
npm install
```

2. Start a peer:

```bash
npm start
```

The peer prints its peer ID and listening multiaddresses.

3. Send a message to another peer:

```bash
node src/commons.js connect <multiaddr> '{"type":"offer","resource":"translation"}'
```

Replace `<multiaddr>` with the address printed by another peer. Set `COMMONS_SECRET` to a shared passphrase before running so peers can decrypt messages. Payloads are JSON representing offers or requests. Each message receives an encrypted acknowledgement ("ack" or "pong").

## Running Tests

```bash
npm test
```

This runs a small suite verifying encryption and message handling.

## Roadmap

See [ROADMAP.md](ROADMAP.md) for upcoming plans.

## License

This project is licensed under the ISC License. See [LICENSE](LICENSE) for details.

### Social Layer (alpha)
This branch introduces the social layer:
- Profiles & follows (sovr-graph, sovr-profile)
- Posts & timeline (sovr-feed)
- Groups & Pages (sovr-groups, sovr-pages)
- Moderation (sovr-moderation)
- Flutter tabs: Feed, Explore, Groups, Pages, Profile

Generate FRB bindings from bridges/flutter/bridge.rs before building UIs.
