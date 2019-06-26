mod decode;
mod encode;

use self::decode::{Decode, DecodeContext};
use self::encode::{Encode, EncodeContext};
use crate::ast::WebidlBindings;
use std::io;

/// Encode the given Web IDL bindings section into the given write-able.
pub fn encode<W>(
    section: &WebidlBindings,
    indices: &walrus::IdsToIndices,
    into: &mut W,
) -> io::Result<()>
where
    W: io::Write,
{
    let cx = &mut EncodeContext::new(indices);
    section.encode(cx, into)
}

/// Decode the Web IDL bindings custom section data from the given input stream.
///
/// This does *not* parse the custom section discriminant and "webidl-bindings"
/// custom section name, just the inner data.
pub fn decode(ids: &walrus::IndicesToIds, from: &[u8]) -> Result<WebidlBindings, failure::Error> {
    let mut cx = DecodeContext::new(ids);
    let mut from = from;
    WebidlBindings::decode(&mut cx, &mut from)?;
    Ok(cx.webidl_bindings)
}
