export function handleMessage(message) {
  try {
    const parsed = typeof message === 'string' ? JSON.parse(message) : message;
    switch (parsed.type) {
      case 'ping':
        return { type: 'pong' };
      default:
        return { type: 'ack' };
    }
  } catch (err) {
    return { type: 'error', error: 'invalid_json' };
  }
}
