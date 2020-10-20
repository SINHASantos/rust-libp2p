// Copyright 2020 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use multihash::{self, Multihash as MultihashGeneric, U32, U64, typenum::U42};
use multihash::derive::Multihash;
pub use multihash::MultihashDigest;

pub type Multihash = MultihashGeneric<U64>;

#[derive(Copy, Clone, Debug, Eq, Multihash, PartialEq)]
#[mh(alloc_size = U64)]
pub enum Code {
    #[mh(code = 0x00, hasher = multihash::IdentityHasher::<U64>, digest = multihash::IdentityDigest<U42>)]
    Identity,
    #[mh(code = 0x12, hasher = multihash::Sha2_256, digest = multihash::Sha2Digest<U32>)]
    Sha2_256,
}
