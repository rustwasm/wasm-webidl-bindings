pub trait Actions {
    type OutgoingBindingExpression: From<Self::OutgoingBindingExpressionAs>
        + From<Self::OutgoingBindingExpressionUtf8Str>;

    type OutgoingBindingExpressionAs;
    fn outgoing_binding_expression_as(
        &mut self,
        ty: Self::WebidlTypeRef,
        idx: u32,
    ) -> Self::OutgoingBindingExpressionAs;

    type OutgoingBindingExpressionUtf8Str;
    fn outgoing_binding_expression_utf8_str(
        &mut self,
        ty: Self::WebidlTypeRef,
        offset: u32,
        length: u32,
    ) -> Self::OutgoingBindingExpressionUtf8Str;

    type WebidlTypeRef: From<Self::WebidlTypeRefNamed> + From<Self::WebidlTypeRefIndexed>;

    type WebidlTypeRefNamed;
    fn webidl_type_ref_named(&mut self, name: &str) -> Self::WebidlTypeRefNamed;

    type WebidlTypeRefIndexed;
    fn webidl_type_ref_indexed(&mut self, idx: u32) -> Self::WebidlTypeRefIndexed;
}
