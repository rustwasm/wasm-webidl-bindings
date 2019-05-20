use crate::ast;
use std::io;

pub(crate) trait Encode<W: io::Write> {
    fn encode(&self, w: &mut W) -> io::Result<()>;
}

impl<W: io::Write> Encode<W> for ast::WebidlBindingsSection {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        unimplemented!()
    }
}
