#!/usr/bin/env coffee
> ./_.pb.js > T0Encode:CallEncode T0Decode:CallDecode T1Encode:CallLiEncode T1Decode:CallLiDecode

bin = CallEncode [
    2
    Buffer.from [1,2,3]
]

console.log bin
console.log CallDecode bin.buffer

bin = CallLiEncode [
  [
    2
    Buffer.from [1,2,3]
  ]
  [
    3
    Buffer.from [4,5,6]
  ]
]

console.log bin
console.log ...CallLiDecode bin.buffer
