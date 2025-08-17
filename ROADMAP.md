# Roadmap

## 1. Product Track — "Skeleton → Survival Kit"
- **Goal:** ship a minimal feature that is useful in a real scenario.
- **v0.2 (4–6 weeks)**
  - Mobile parity: stable Android builds and iOS builds passing CI (TestFlight).
  - CRDT Notes: add a "Notebook" feature in the Vault. Local-first Automerge docs that sync across trusted peers.
  - Headless relay: Raspberry Pi node forwards messages between peers without direct connectivity.
  - Identity UX: printable recovery sheet with words and QR.
- **v0.3 (2–3 months)**
  - Ledger MVP: barter contracts with signatures, CSV/PDF exports.
  - Trust UX: "circle of trust" screen to manage mesh peers.
  - Content pipeline: publish/import Vault Packs (starter knowledge, local guides).
- **v1.0 (~6 months)**
  - Emergency mode: bulletins over low-power mesh.
  - Installer polish: signed desktop apps, packaged Android APK, Pi image.
  - Audit & hardening: external code review, fuzzing, reproducible builds.

## 2. Engineering Track — "Toy → Trustworthy"
- Mesh reality check: fallback QR-code handshake and opportunistic relay for mobile BLE/mDNS challenges.
- Crypto correctness: move to full Noise handshake and Double Ratchet; integration tests with 1000 messages to ensure forward secrecy.
- Storage durability: integrate SQLCipher and provide backup/export/import flows.
- Developer experience: `make dev-up` to spin local Docker nodes; expand Rust unit tests and Flutter widget/integration tests.
- CI/CD expansion: release binaries on tags for Linux (x86_64/aarch64), macOS (Intel/ARM), and Windows; set up nightly builds for dogfooding.

## 3. Community Track — "Code → Movement"
- Governance: publish CONTRIBUTING.md, SECURITY.md with PGP contact, and CODEOWNERS; expose roadmap milestones.
- Early testers: recruit 5–10 people willing to install the alpha and break it; provide a starter Vault Pack.
- Narrative: frame SOVR as a digital survival kit with clear disclaimers that it's experimental and not audited.
- Community infrastructure: Matrix or self-hosted forum instead of centralized platforms, GitHub Discussions, monthly "state of the stack" updates.

## 4. Hardening Track — Ongoing
- Threat modeling workshops with documentation updated each milestone.
- Fuzz tests for message parsing and CRDT merge.
- Reproducible builds using Nix or Docker.
- External audit once core flows (chat, ledger, vault) are stable.

