mod encode;

use crate::ast::WebidlBindingsSection;
use std::io;

/// Encode the given Web IDL bindings section into the given write-able.
pub fn encode<W>(section: &WebidlBindingsSection, into: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    use self::encode::Encode;
    section.encode(into)
}
