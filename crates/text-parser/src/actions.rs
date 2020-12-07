pub trait Actions {
    type WebidlBindingsSection;
    fn webidl_bindings_section(
        &mut self,
        types: Self::WebidlTypeSubsection,
        bindings: Self::WebidlFunctionBindingsSubsection,
    ) -> Self::WebidlBindingsSection;

    type WebidlTypeSubsection;
    fn webidl_type_subsection(
        &mut self,
        types: Vec<Self::WebidlType>,
    ) -> Self::WebidlTypeSubsection;

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

    type WebidlFunctionBindingsSubsection;
    fn webidl_function_bindings_subsection(
        &mut self,
        bindings: Vec<Self::FunctionBinding>,
        binds: Vec<Self::Bind>,
    ) -> Self::WebidlFunctionBindingsSubsection;

    type FunctionBinding: From<Self::ImportBinding> + From<Self::ExportBinding>;

    type ImportBinding;
    fn import_binding(
        &mut self,
        name: Option<&str>,
        wasm_ty: Self::WasmFuncTypeRef,
        webidl_ty: Self::WebidlTypeRef,
        params: Option<Self::OutgoingBindingMap>,
        result: Option<Self::IncomingBindingMap>,
    ) -> Self::ImportBinding;

    type ExportBinding;
    fn export_binding(
        &mut self,
        name: Option<&str>,
        wasm_ty: Self::WasmFuncTypeRef,
        webidl_ty: Self::WebidlTypeRef,
        params: Option<Self::IncomingBindingMap>,
        result: Option<Self::OutgoingBindingMap>,
    ) -> Self::ExportBinding;

    type Bind;
    fn bind(&mut self, func: Self::WasmFuncRef, binding: Self::BindingRef) -> Self::Bind;

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
        binding: Self::BindingRef,
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
        ty: Self::WasmValType,
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
        ty: Self::WasmFuncTypeRef,
        binding: Self::BindingRef,
        expr: Self::IncomingBindingExpression,
    ) -> Self::IncomingBindingExpressionBindImport;

    type WebidlTypeRef: From<Self::WebidlTypeRefNamed>
        + From<Self::WebidlTypeRefIndexed>
        + From<Self::WebidlScalarType>;

    type WebidlTypeRefNamed;
    fn webidl_type_ref_named(&mut self, name: &str) -> Option<Self::WebidlTypeRefNamed>;

    type WebidlTypeRefIndexed;
    fn webidl_type_ref_indexed(&mut self, idx: u32) -> Option<Self::WebidlTypeRefIndexed>;

    type WebidlScalarType;
    fn webidl_scalar_type_any(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_boolean(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_byte(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_octet(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_long(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_unsigned_long(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_short(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_unsigned_short(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_long_long(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_unsigned_long_long(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_float(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_unrestricted_float(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_double(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_unrestricted_double(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_dom_string(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_byte_string(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_usv_string(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_object(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_symbol(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_array_buffer(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_data_view(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_int8_array(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_int16_array(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_int32_array(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_uint8_array(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_uint16_array(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_uint32_array(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_uint8_clamped_array(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_float32_array(&mut self) -> Self::WebidlScalarType;
    fn webidl_scalar_type_float64_array(&mut self) -> Self::WebidlScalarType;

    type WasmValType;
    fn wasm_val_type_i32(&mut self) -> Self::WasmValType;
    fn wasm_val_type_i64(&mut self) -> Self::WasmValType;
    fn wasm_val_type_f32(&mut self) -> Self::WasmValType;
    fn wasm_val_type_f64(&mut self) -> Self::WasmValType;
    fn wasm_val_type_v128(&mut self) -> Self::WasmValType;
    fn wasm_val_type_anyref(&mut self) -> Self::WasmValType;

    type WasmFuncTypeRef: From<Self::WasmFuncTypeRefNamed> + From<Self::WasmFuncTypeRefIndexed>;

    type WasmFuncTypeRefNamed;
    fn wasm_func_type_ref_named(&mut self, name: &str) -> Option<Self::WasmFuncTypeRefNamed>;

    type WasmFuncTypeRefIndexed;
    fn wasm_func_type_ref_indexed(&mut self, idx: u32) -> Option<Self::WasmFuncTypeRefIndexed>;

    type WasmFuncRef: From<Self::WasmFuncRefNamed> + From<Self::WasmFuncRefIndexed>;

    type WasmFuncRefNamed;
    fn wasm_func_ref_named(&mut self, name: &str) -> Option<Self::WasmFuncRefNamed>;

    type WasmFuncRefIndexed;
    fn wasm_func_ref_indexed(&mut self, idx: u32) -> Option<Self::WasmFuncRefIndexed>;

    type BindingRef: From<Self::BindingRefNamed> + From<Self::BindingRefIndexed>;

    type BindingRefNamed;
    fn binding_ref_named(&mut self, name: &str) -> Option<Self::BindingRefNamed>;

    type BindingRefIndexed;
    fn binding_ref_indexed(&mut self, idx: u32) -> Option<Self::BindingRefIndexed>;
}
