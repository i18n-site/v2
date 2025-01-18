import {
  T0Encode as CallEncode,
  T1Encode as CallLiEncode,
  T2Decode as BinLiDecode
} from './_.pb.js'

import ireq from '@3-/ireq'

const [_setUrl, _req, _batch] = ireq(
  // err catch
  (code, res)=>{
    console.log('TODO err catch', code)
  },
  CallEncode, CallLiEncode, BinLiDecode,
)

export default req = _req;
export const batch = _batch;
export const setUrl = _setUrl;
