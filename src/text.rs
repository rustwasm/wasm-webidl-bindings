//! Working with the text format.

pub use wasm_webidl_bindings_text_parser::*;

/// Parse the given straw proposal text format input into an AST.
pub fn parse(
    module: &walrus::Module,
    indices_to_ids: &walrus::IndicesToIds,
    input: &str,
) -> anyhow::Result<crate::ast::WebidlBindings> {
    let mut bindings = crate::ast::WebidlBindings::default();
    let mut actions = crate::ast::BuildAstActions::new(&mut bindings, module, indices_to_ids);
    parse_with_actions(&mut actions, input)?;
    Ok(bindings)
}
