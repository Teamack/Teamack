import { test } from 'node:test';
import assert from 'node:assert';
import { handleMessage } from '../src/messages.js';

test('responds to ping with pong', () => {
  const res = handleMessage('{"type":"ping"}');
  assert.deepStrictEqual(res, { type: 'pong' });
});

test('acknowledges unknown messages', () => {
  const res = handleMessage('{"type":"foo"}');
  assert.deepStrictEqual(res, { type: 'ack' });
});

test('invalid json returns error', () => {
  const res = handleMessage('not json');
  assert.deepStrictEqual(res, { type: 'error', error: 'invalid_json' });
});
