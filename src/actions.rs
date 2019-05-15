pub trait Actions {
    type WebidlBindingsSection;
    fn webidl_bindings_section(
        &mut self,
        types: Self::WebidlTypeSubsection,
        bindings: Self::WebidlFunctionBindingsSubsection,
    ) -> Self::WebidlBindingsSection;

    type WebidlTypeSubsection: From<Vec<Self::WebidlType>>;

    type WebidlType;
    fn webidl_type(&mut self, name: Option<&str>, ty: Self::WebidlCompoundType)
        -> Self::WebidlType;

    type WebidlCompoundType: From<Self::WebidlFunction>
        + From<Self::WebidlDictionary>
        + From<Self::WebidlEnumeration>
        + From<Self::WebidlUnion>;

    type WebidlFunction;
    fn webidl_function(
        &mut self,
        kind: Option<Self::WebidlFunctionKind>,
        params: Option<Self::WebidlFunctionParams>,
        result: Option<Self::WebidlFunctionResult>,
    ) -> Self::WebidlFunction;

    type WebidlFunctionKind: From<Self::WebidlFunctionKindMethod>
        + From<Self::WebidlFunctionKindConstructor>;

    type WebidlFunctionKindMethod;
    fn webidl_function_kind_method(
        &mut self,
        ty: Self::WebidlTypeRef,
    ) -> Self::WebidlFunctionKindMethod;

    type WebidlFunctionKindConstructor;
    fn webidl_function_kind_constructor_default_new_target(
        &mut self,
    ) -> Self::WebidlFunctionKindConstructor;

    type WebidlFunctionParams;
    fn webidl_function_params(
        &mut self,
        tys: Vec<Self::WebidlTypeRef>,
    ) -> Self::WebidlFunctionParams;

    type WebidlFunctionResult;
    fn webidl_function_result(&mut self, ty: Self::WebidlTypeRef) -> Self::WebidlFunctionResult;

    type WebidlDictionary;
    fn webidl_dictionary(
        &mut self,
        fields: Vec<Self::WebidlDictionaryField>,
    ) -> Self::WebidlDictionary;

    type WebidlDictionaryField;
    fn webidl_dictionary_field(
        &mut self,
        name: Self::WebidlDictionaryFieldName,
        ty: Self::WebidlTypeRef,
    ) -> Self::WebidlDictionaryField;

    type WebidlDictionaryFieldName;
    fn webidl_dictionary_field_name(&mut self, name: &str) -> Self::WebidlDictionaryFieldName;

    type WebidlEnumeration;
    fn webidl_enumeration(
        &mut self,
        values: Vec<Self::WebidlEnumerationValue>,
    ) -> Self::WebidlEnumeration;

    type WebidlEnumerationValue;
    fn webidl_enumeration_value(&mut self, value: &str) -> Self::WebidlEnumerationValue;

    type WebidlUnion;
    fn webidl_union(&mut self, members: Vec<Self::WebidlTypeRef>) -> Self::WebidlUnion;

    type WebidlFunctionBindingsSubsection: From<Vec<Self::FunctionBinding>>;

    type FunctionBinding: From<Self::ImportBinding>;

    type ImportBinding;
    fn import_binding(
        &mut self,
        name: Option<&str>,
        wasm_ty: Self::WasmTypeRef,
        webidl_ty: Self::WebidlTypeRef,
        params: Self::OutgoingBindingMap,
        result: Self::IncomingBindingMap,
    ) -> Self::ImportBinding;

    type OutgoingBindingMap;
    fn outgoing_binding_map(
        &mut self,
        bindings: Vec<Self::OutgoingBindingExpression>,
    ) -> Self::OutgoingBindingMap;

    type IncomingBindingMap;
    fn incoming_binding_map(
        &mut self,
        bindings: Vec<Self::IncomingBindingExpression>,
    ) -> Self::IncomingBindingMap;

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
        + From<Self::IncomingBindingExpressionAllocUtf8Str>
        + From<Self::IncomingBindingExpressionAllocCopy>
        + From<Self::IncomingBindingExpressionEnumToI32>
        + From<Self::IncomingBindingExpressionField>
        + From<Self::IncomingBindingExpressionBindImport>;

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

    type IncomingBindingExpressionAllocCopy;
    fn incoming_binding_expression_alloc_copy(
        &mut self,
        alloc_func_name: &str,
        expr: Self::IncomingBindingExpression,
    ) -> Self::IncomingBindingExpressionAllocCopy;

    type IncomingBindingExpressionEnumToI32;
    fn incoming_binding_expression_enum_to_i32(
        &mut self,
        ty: Self::WebidlTypeRef,
        expr: Self::IncomingBindingExpression,
    ) -> Self::IncomingBindingExpressionEnumToI32;

    type IncomingBindingExpressionField;
    fn incoming_binding_expression_field(
        &mut self,
        idx: u32,
        expr: Self::IncomingBindingExpression,
    ) -> Self::IncomingBindingExpressionField;

    type IncomingBindingExpressionBindImport;
    fn incoming_binding_expression_bind_import(
        &mut self,
        ty: Self::WasmTypeRef,
        binding: Self::ImportBindingRef,
        expr: Self::IncomingBindingExpression,
    ) -> Self::IncomingBindingExpressionBindImport;

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

    type ImportBindingRef: From<Self::ImportBindingRefNamed> + From<Self::ImportBindingRefIndexed>;

    type ImportBindingRefNamed;
    fn import_binding_ref_named(&mut self, name: &str) -> Self::ImportBindingRefNamed;

    type ImportBindingRefIndexed;
    fn import_binding_ref_indexed(&mut self, idx: u32) -> Self::ImportBindingRefIndexed;
}
