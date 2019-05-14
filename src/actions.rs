pub trait Actions {
    type OutgoingBindingExpression: From<Self::OutgoingBindingExpressionAs>
        + From<Self::OutgoingBindingExpressionUtf8Str>
        + From<Self::OutgoingBindingExpressionUtf8CStr>
        + From<Self::OutgoingBindingExpressionI32ToEnum>
        + From<Self::OutgoingBindingExpressionView>
        + From<Self::OutgoingBindingExpressionCopy>
        + From<Self::OutgoingBindingExpressionDict>
        + From<Self::OutgoingBindingExpressionBindExport>;

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

    type OutgoingBindingExpressionUtf8CStr;
    fn outgoing_binding_expression_utf8_c_str(
        &mut self,
        ty: Self::WebidlTypeRef,
        offset: u32,
    ) -> Self::OutgoingBindingExpressionUtf8CStr;

    type OutgoingBindingExpressionI32ToEnum;
    fn outgoing_binding_expression_i32_to_enum(
        &mut self,
        ty: Self::WebidlTypeRef,
        idx: u32,
    ) -> Self::OutgoingBindingExpressionI32ToEnum;

    type OutgoingBindingExpressionView;
    fn outgoing_binding_expression_view(
        &mut self,
        ty: Self::WebidlTypeRef,
        offset: u32,
        length: u32,
    ) -> Self::OutgoingBindingExpressionView;

    type OutgoingBindingExpressionCopy;
    fn outgoing_binding_expression_copy(
        &mut self,
        ty: Self::WebidlTypeRef,
        offset: u32,
        length: u32,
    ) -> Self::OutgoingBindingExpressionCopy;

    type OutgoingBindingExpressionDict;
    fn outgoing_binding_expression_dict(
        &mut self,
        ty: Self::WebidlTypeRef,
        fields: Vec<Self::OutgoingBindingExpression>,
    ) -> Self::OutgoingBindingExpressionDict;

    type OutgoingBindingExpressionBindExport;
    fn outgoing_binding_expression_bind_export(
        &mut self,
        ty: Self::WebidlTypeRef,
        binding: Self::ExportBindingRef,
        idx: u32,
    ) -> Self::OutgoingBindingExpressionBindExport;

    type IncomingBindingExpression: From<Self::IncomingBindingExpressionGet>
        + From<Self::IncomingBindingExpressionAs>
        + From<Self::IncomingBindingExpressionAllocUtf8Str>;

    type IncomingBindingExpressionGet;
    fn incoming_binding_expression_get(&mut self, idx: u32) -> Self::IncomingBindingExpressionGet;

    type IncomingBindingExpressionAs;
    fn incoming_binding_expression_as(
        &mut self,
        ty: Self::WasmTypeRef,
        expr: Self::IncomingBindingExpression,
    ) -> Self::IncomingBindingExpressionAs;

    type IncomingBindingExpressionAllocUtf8Str;
    fn incoming_binding_expression_alloc_utf8_str(
        &mut self,
        alloc_func_name: &str,
        expr: Self::IncomingBindingExpression,
    ) -> Self::IncomingBindingExpressionAllocUtf8Str;

    type WebidlTypeRef: From<Self::WebidlTypeRefNamed> + From<Self::WebidlTypeRefIndexed>;

    type WebidlTypeRefNamed;
    fn webidl_type_ref_named(&mut self, name: &str) -> Self::WebidlTypeRefNamed;

    type WebidlTypeRefIndexed;
    fn webidl_type_ref_indexed(&mut self, idx: u32) -> Self::WebidlTypeRefIndexed;

    type WasmTypeRef: From<Self::WasmTypeRefNamed> + From<Self::WasmTypeRefIndexed>;

    type WasmTypeRefNamed;
    fn wasm_type_ref_named(&mut self, name: &str) -> Self::WasmTypeRefNamed;

    type WasmTypeRefIndexed;
    fn wasm_type_ref_indexed(&mut self, idx: u32) -> Self::WasmTypeRefIndexed;

    type ExportBindingRef: From<Self::ExportBindingRefNamed> + From<Self::ExportBindingRefIndexed>;

    type ExportBindingRefNamed;
    fn export_binding_ref_named(&mut self, name: &str) -> Self::ExportBindingRefNamed;

    type ExportBindingRefIndexed;
    fn export_binding_ref_indexed(&mut self, idx: u32) -> Self::ExportBindingRefIndexed;
}
