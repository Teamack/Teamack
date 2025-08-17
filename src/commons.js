import { createLibp2p } from 'libp2p'
import { tcp } from '@libp2p/tcp'
import { noise } from '@chainsafe/libp2p-noise'
import { mplex } from '@libp2p/mplex'
import { multiaddr } from '@multiformats/multiaddr'

const protocol = '/commons/1.0.0'

async function start () {
  const node = await createLibp2p({
    addresses: {
      listen: ['/ip4/0.0.0.0/tcp/0']
    },
    transports: [tcp()],
    connectionEncryption: [noise()],
    streamMuxers: [mplex()]
  })

  await node.start()
  console.log('Peer ID:', node.peerId.toString())
  console.log('Listening on:')
  node.getMultiaddrs().forEach(addr => {
    console.log(`  ${addr.toString()}`)
  })

  node.handle(protocol, async ({ stream }) => {
    let data = ''
    for await (const chunk of stream.source) {
      data += new TextDecoder().decode(chunk.subarray())
    }
    console.log('Received:', data)
  })

  const args = process.argv.slice(2)
  if (args[0] === 'connect') {
    const addr = args[1]
    const message = args.slice(2).join(' ') || '{"type":"ping"}'
    const connection = await node.dial(multiaddr(addr))
    const stream = await connection.newStream(protocol)
    await stream.sink([Uint8Array.from(Buffer.from(message))])
    await stream.close()
    console.log('Sent:', message)
  } else {
    console.log('Waiting for incoming messages...')
  }
}

start().catch(err => {
  console.error(err)
  process.exit(1)
})
