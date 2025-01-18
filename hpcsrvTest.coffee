#!/usr/bin/env coffee

r = await fetch(
  'http://0.0.0.0:2025'
  {
    method: 'POST',
    body: new Uint8Array([
        1,
        0,
        8,
        116,
        101,
        115,
        116,
        32,
        115,
        120,
        120,
        1,
    ])
  }
)

console.log 'statusCode',r.status
console.log await r.text()
