import { test } from 'node:test';
import assert from 'node:assert';
import { encrypt, decrypt } from '../src/crypto.js';

test('encrypt and decrypt round trip', () => {
  const msg = 'secret message';
  const key = 'passphrase';
  const enc = encrypt(msg, key);
  assert.notStrictEqual(enc, msg);
  const dec = decrypt(enc, key);
  assert.strictEqual(dec, msg);
});
