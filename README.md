# Commons Engine Prototype

This repository hosts a minimal peer-to-peer resource and skill exchange. It spins up a [libp2p](https://libp2p.io/) node that can exchange JSON messages with other peers. Messages are end-to-end encrypted using AES-256-GCM and acknowledged by the receiver. The goal is to explore infrastructure for mutual aid without central control.

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

## License

This project is licensed under the ISC License. See [LICENSE](LICENSE) for details.
