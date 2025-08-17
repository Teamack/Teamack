import { createLibp2p } from 'libp2p';
import { tcp } from '@libp2p/tcp';
import { noise } from '@chainsafe/libp2p-noise';
import { mplex } from '@libp2p/mplex';
import { multiaddr } from '@multiformats/multiaddr';
import { encrypt, decrypt } from './crypto.js';
import { handleMessage } from './messages.js';

const protocol = '/commons/1.0.0';
const secret = process.env.COMMONS_SECRET || 'commons-secret';

async function start () {
  const node = await createLibp2p({
    addresses: {
      listen: ['/ip4/0.0.0.0/tcp/0']
    },
    transports: [tcp()],
    connectionEncryption: [noise()],
    streamMuxers: [mplex()]
  });

  await node.start();
  console.log('Peer ID:', node.peerId.toString());
  console.log('Listening on:');
  node.getMultiaddrs().forEach(addr => {
    console.log(`  ${addr.toString()}`);
  });

  node.handle(protocol, async ({ stream }) => {
    let raw = '';
    for await (const chunk of stream.source) {
      raw += new TextDecoder().decode(chunk.subarray());
    }
    try {
      const plaintext = decrypt(raw, secret);
      console.log('Received:', plaintext);
      const response = handleMessage(plaintext);
      const encrypted = encrypt(JSON.stringify(response), secret);
      await stream.sink([Uint8Array.from(Buffer.from(encrypted))]);
    } catch (err) {
      console.error('Failed to process message:', err);
    }
  });

  const args = process.argv.slice(2);
  if (args[0] === 'connect') {
    const addr = args[1];
    const message = args.slice(2).join(' ') || '{"type":"ping"}';
    const connection = await node.dial(multiaddr(addr));
    const stream = await connection.newStream(protocol);
    const encrypted = encrypt(message, secret);
    await stream.sink([Uint8Array.from(Buffer.from(encrypted))]);
    let reply = '';
    for await (const chunk of stream.source) {
      reply += new TextDecoder().decode(chunk.subarray());
    }
    await stream.close();
    const decrypted = decrypt(reply, secret);
    console.log('Response:', decrypted);
  } else {
    console.log('Waiting for incoming messages...');
  }
}

start().catch(err => {
  console.error(err);
  process.exit(1);
});
