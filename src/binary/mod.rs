mod encode;

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
