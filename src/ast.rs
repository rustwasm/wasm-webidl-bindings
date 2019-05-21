use crate::text::Actions;

#[derive(Debug, Default)]
pub struct BuildAstActions;

impl Actions for BuildAstActions {
    type WebidlBindingsSection = WebidlBindingsSection;

    fn webidl_bindings_section(
        &mut self,
        types: WebidlTypeSubsection,
        bindings: WebidlFunctionBindingsSubsection,
    ) -> WebidlBindingsSection {
        WebidlBindingsSection { types, bindings }
    }

    type WebidlTypeSubsection = WebidlTypeSubsection;

    type WebidlType = WebidlType;
    fn webidl_type(&mut self, name: Option<&str>, ty: WebidlCompoundType) -> WebidlType {
        let name = name.map(ToString::to_string);
        WebidlType { name, ty }
    }

    type WebidlCompoundType = WebidlCompoundType;

    type WebidlFunction = WebidlFunction;
    fn webidl_function(
        &mut self,
        kind: Option<WebidlFunctionKind>,
        params: Option<Vec<WebidlTypeRef>>,
        result: Option<WebidlTypeRef>,
    ) -> WebidlFunction {
        let kind = kind.unwrap_or(WebidlFunctionKind::Static);
        let params = params.unwrap_or(vec![]);
        WebidlFunction {
            kind,
            params,
            result,
        }
    }

    type WebidlFunctionKind = WebidlFunctionKind;

    type WebidlFunctionKindMethod = WebidlFunctionKindMethod;
    fn webidl_function_kind_method(&mut self, ty: WebidlTypeRef) -> WebidlFunctionKindMethod {
        WebidlFunctionKindMethod { ty }
    }

    type WebidlFunctionKindConstructor = WebidlFunctionKind;
    fn webidl_function_kind_constructor_default_new_target(&mut self) -> WebidlFunctionKind {
        WebidlFunctionKind::Constructor
    }

    type WebidlFunctionParams = Vec<WebidlTypeRef>;
    fn webidl_function_params(&mut self, tys: Vec<WebidlTypeRef>) -> Vec<WebidlTypeRef> {
        tys
    }

    type WebidlFunctionResult = WebidlTypeRef;
    fn webidl_function_result(&mut self, ty: WebidlTypeRef) -> WebidlTypeRef {
        ty
    }

    type WebidlDictionary = WebidlDictionary;
    fn webidl_dictionary(&mut self, fields: Vec<WebidlDictionaryField>) -> WebidlDictionary {
        WebidlDictionary { fields }
    }

    type WebidlDictionaryField = WebidlDictionaryField;
    fn webidl_dictionary_field(
        &mut self,
        name: String,
        ty: WebidlTypeRef,
    ) -> WebidlDictionaryField {
        WebidlDictionaryField { name, ty }
    }

    type WebidlDictionaryFieldName = String;
    fn webidl_dictionary_field_name(&mut self, name: &str) -> String {
        name.into()
    }

    type WebidlEnumeration = WebidlEnumeration;
    fn webidl_enumeration(&mut self, values: Vec<String>) -> WebidlEnumeration {
        WebidlEnumeration { values }
    }

    type WebidlEnumerationValue = String;
    fn webidl_enumeration_value(&mut self, value: &str) -> String {
        value.into()
    }

    type WebidlUnion = WebidlUnion;
    fn webidl_union(&mut self, members: Vec<WebidlTypeRef>) -> WebidlUnion {
        WebidlUnion { members }
    }

    type WebidlFunctionBindingsSubsection = WebidlFunctionBindingsSubsection;
    fn webidl_function_bindings_subsection(
        &mut self,
        bindings: Vec<FunctionBinding>,
        binds: Vec<Bind>,
    ) -> WebidlFunctionBindingsSubsection {
        WebidlFunctionBindingsSubsection { bindings, binds }
    }

    type FunctionBinding = FunctionBinding;

    type ImportBinding = ImportBinding;
    fn import_binding(
        &mut self,
        name: Option<&str>,
        wasm_ty: WasmFuncTypeRef,
        webidl_ty: WebidlTypeRef,
        params: OutgoingBindingMap,
        result: IncomingBindingMap,
    ) -> ImportBinding {
        let name = name.map(ToString::to_string);
        ImportBinding {
            name,
            wasm_ty,
            webidl_ty,
            params,
            result,
        }
    }

    type ExportBinding = ExportBinding;
    fn export_binding(
        &mut self,
        name: Option<&str>,
        wasm_ty: WasmFuncTypeRef,
        webidl_ty: WebidlTypeRef,
        params: IncomingBindingMap,
        result: OutgoingBindingMap,
    ) -> ExportBinding {
        let name = name.map(ToString::to_string);
        ExportBinding {
            name,
            wasm_ty,
            webidl_ty,
            params,
            result,
        }
    }

    type Bind = Bind;
    fn bind(&mut self, func: WasmFuncRef, binding: BindingRef) -> Bind {
        Bind { func, binding }
    }

    type OutgoingBindingMap = OutgoingBindingMap;
    fn outgoing_binding_map(
        &mut self,
        bindings: Vec<OutgoingBindingExpression>,
    ) -> OutgoingBindingMap {
        OutgoingBindingMap { bindings }
    }

    type IncomingBindingMap = IncomingBindingMap;
    fn incoming_binding_map(
        &mut self,
        bindings: Vec<IncomingBindingExpression>,
    ) -> IncomingBindingMap {
        IncomingBindingMap { bindings }
    }

    type OutgoingBindingExpression = OutgoingBindingExpression;

    type OutgoingBindingExpressionAs = OutgoingBindingExpressionAs;
    fn outgoing_binding_expression_as(
        &mut self,
        ty: WebidlTypeRef,
        idx: u32,
    ) -> OutgoingBindingExpressionAs {
        OutgoingBindingExpressionAs { ty, idx }
    }

    type OutgoingBindingExpressionUtf8Str = OutgoingBindingExpressionUtf8Str;
    fn outgoing_binding_expression_utf8_str(
        &mut self,
        ty: WebidlTypeRef,
        offset: u32,
        length: u32,
    ) -> OutgoingBindingExpressionUtf8Str {
        OutgoingBindingExpressionUtf8Str { ty, offset, length }
    }

    type OutgoingBindingExpressionUtf8CStr = OutgoingBindingExpressionUtf8CStr;
    fn outgoing_binding_expression_utf8_c_str(
        &mut self,
        ty: WebidlTypeRef,
        offset: u32,
    ) -> OutgoingBindingExpressionUtf8CStr {
        OutgoingBindingExpressionUtf8CStr { ty, offset }
    }

    type OutgoingBindingExpressionI32ToEnum = OutgoingBindingExpressionI32ToEnum;
    fn outgoing_binding_expression_i32_to_enum(
        &mut self,
        ty: WebidlTypeRef,
        idx: u32,
    ) -> OutgoingBindingExpressionI32ToEnum {
        OutgoingBindingExpressionI32ToEnum { ty, idx }
    }

    type OutgoingBindingExpressionView = OutgoingBindingExpressionView;
    fn outgoing_binding_expression_view(
        &mut self,
        ty: WebidlTypeRef,
        offset: u32,
        length: u32,
    ) -> OutgoingBindingExpressionView {
        OutgoingBindingExpressionView { ty, offset, length }
    }

    type OutgoingBindingExpressionCopy = OutgoingBindingExpressionCopy;
    fn outgoing_binding_expression_copy(
        &mut self,
        ty: WebidlTypeRef,
        offset: u32,
        length: u32,
    ) -> OutgoingBindingExpressionCopy {
        OutgoingBindingExpressionCopy { ty, offset, length }
    }

    type OutgoingBindingExpressionDict = OutgoingBindingExpressionDict;
    fn outgoing_binding_expression_dict(
        &mut self,
        ty: WebidlTypeRef,
        fields: Vec<OutgoingBindingExpression>,
    ) -> OutgoingBindingExpressionDict {
        OutgoingBindingExpressionDict { ty, fields }
    }

    type OutgoingBindingExpressionBindExport = OutgoingBindingExpressionBindExport;
    fn outgoing_binding_expression_bind_export(
        &mut self,
        ty: WebidlTypeRef,
        binding: BindingRef,
        idx: u32,
    ) -> OutgoingBindingExpressionBindExport {
        OutgoingBindingExpressionBindExport { ty, binding, idx }
    }

    type IncomingBindingExpression = IncomingBindingExpression;

    type IncomingBindingExpressionGet = IncomingBindingExpressionGet;
    fn incoming_binding_expression_get(&mut self, idx: u32) -> IncomingBindingExpressionGet {
        IncomingBindingExpressionGet { idx }
    }

    type IncomingBindingExpressionAs = IncomingBindingExpressionAs;
    fn incoming_binding_expression_as(
        &mut self,
        ty: walrus::ValType,
        expr: IncomingBindingExpression,
    ) -> IncomingBindingExpressionAs {
        let expr = Box::new(expr);
        IncomingBindingExpressionAs { ty, expr }
    }

    type IncomingBindingExpressionAllocUtf8Str = IncomingBindingExpressionAllocUtf8Str;
    fn incoming_binding_expression_alloc_utf8_str(
        &mut self,
        alloc_func_name: &str,
        expr: IncomingBindingExpression,
    ) -> IncomingBindingExpressionAllocUtf8Str {
        let alloc_func_name = alloc_func_name.into();
        let expr = Box::new(expr);
        IncomingBindingExpressionAllocUtf8Str {
            alloc_func_name,
            expr,
        }
    }

    type IncomingBindingExpressionAllocCopy = IncomingBindingExpressionAllocCopy;
    fn incoming_binding_expression_alloc_copy(
        &mut self,
        alloc_func_name: &str,
        expr: IncomingBindingExpression,
    ) -> IncomingBindingExpressionAllocCopy {
        let alloc_func_name = alloc_func_name.into();
        let expr = Box::new(expr);
        IncomingBindingExpressionAllocCopy {
            alloc_func_name,
            expr,
        }
    }

    type IncomingBindingExpressionEnumToI32 = IncomingBindingExpressionEnumToI32;
    fn incoming_binding_expression_enum_to_i32(
        &mut self,
        ty: WebidlTypeRef,
        expr: IncomingBindingExpression,
    ) -> IncomingBindingExpressionEnumToI32 {
        let expr = Box::new(expr);
        IncomingBindingExpressionEnumToI32 { ty, expr }
    }

    type IncomingBindingExpressionField = IncomingBindingExpressionField;
    fn incoming_binding_expression_field(
        &mut self,
        idx: u32,
        expr: IncomingBindingExpression,
    ) -> IncomingBindingExpressionField {
        let expr = Box::new(expr);
        IncomingBindingExpressionField { idx, expr }
    }

    type IncomingBindingExpressionBindImport = IncomingBindingExpressionBindImport;
    fn incoming_binding_expression_bind_import(
        &mut self,
        ty: WasmFuncTypeRef,
        binding: BindingRef,
        expr: IncomingBindingExpression,
    ) -> IncomingBindingExpressionBindImport {
        let expr = Box::new(expr);
        IncomingBindingExpressionBindImport { ty, binding, expr }
    }

    type WebidlTypeRef = WebidlTypeRef;

    type WebidlTypeRefNamed = WebidlTypeRefNamed;
    fn webidl_type_ref_named(&mut self, name: &str) -> WebidlTypeRefNamed {
        let name = name.to_string();
        WebidlTypeRefNamed { name }
    }

    type WebidlTypeRefIndexed = WebidlTypeRefIndexed;
    fn webidl_type_ref_indexed(&mut self, idx: u32) -> WebidlTypeRefIndexed {
        WebidlTypeRefIndexed { idx }
    }

    type WebidlScalarType = WebidlScalarType;
    fn webidl_scalar_type_any(&mut self) -> WebidlScalarType {
        WebidlScalarType::Any
    }
    fn webidl_scalar_type_boolean(&mut self) -> WebidlScalarType {
        WebidlScalarType::Boolean
    }
    fn webidl_scalar_type_byte(&mut self) -> WebidlScalarType {
        WebidlScalarType::Byte
    }
    fn webidl_scalar_type_octet(&mut self) -> WebidlScalarType {
        WebidlScalarType::Octet
    }
    fn webidl_scalar_type_long(&mut self) -> WebidlScalarType {
        WebidlScalarType::Long
    }
    fn webidl_scalar_type_unsigned_long(&mut self) -> WebidlScalarType {
        WebidlScalarType::UnsignedLong
    }
    fn webidl_scalar_type_short(&mut self) -> WebidlScalarType {
        WebidlScalarType::Short
    }
    fn webidl_scalar_type_unsigned_short(&mut self) -> WebidlScalarType {
        WebidlScalarType::UnsignedShort
    }
    fn webidl_scalar_type_long_long(&mut self) -> WebidlScalarType {
        WebidlScalarType::LongLong
    }
    fn webidl_scalar_type_unsigned_long_long(&mut self) -> WebidlScalarType {
        WebidlScalarType::UnsignedLongLong
    }
    fn webidl_scalar_type_float(&mut self) -> WebidlScalarType {
        WebidlScalarType::Float
    }
    fn webidl_scalar_type_unrestricted_float(&mut self) -> WebidlScalarType {
        WebidlScalarType::UnrestrictedFloat
    }
    fn webidl_scalar_type_double(&mut self) -> WebidlScalarType {
        WebidlScalarType::Double
    }
    fn webidl_scalar_type_unrestricted_double(&mut self) -> WebidlScalarType {
        WebidlScalarType::UnrestrictedDouble
    }
    fn webidl_scalar_type_dom_string(&mut self) -> WebidlScalarType {
        WebidlScalarType::DomString
    }
    fn webidl_scalar_type_byte_string(&mut self) -> WebidlScalarType {
        WebidlScalarType::ByteString
    }
    fn webidl_scalar_type_usv_string(&mut self) -> WebidlScalarType {
        WebidlScalarType::UsvString
    }
    fn webidl_scalar_type_object(&mut self) -> WebidlScalarType {
        WebidlScalarType::Object
    }
    fn webidl_scalar_type_symbol(&mut self) -> WebidlScalarType {
        WebidlScalarType::Symbol
    }
    fn webidl_scalar_type_array_buffer(&mut self) -> WebidlScalarType {
        WebidlScalarType::ArrayBuffer
    }
    fn webidl_scalar_type_data_view(&mut self) -> WebidlScalarType {
        WebidlScalarType::DataView
    }
    fn webidl_scalar_type_int8_array(&mut self) -> WebidlScalarType {
        WebidlScalarType::Int8Array
    }
    fn webidl_scalar_type_int16_array(&mut self) -> WebidlScalarType {
        WebidlScalarType::Int16Array
    }
    fn webidl_scalar_type_int32_array(&mut self) -> WebidlScalarType {
        WebidlScalarType::Int32Array
    }
    fn webidl_scalar_type_uint8_array(&mut self) -> WebidlScalarType {
        WebidlScalarType::Uint8Array
    }
    fn webidl_scalar_type_uint16_array(&mut self) -> WebidlScalarType {
        WebidlScalarType::Uint16Array
    }
    fn webidl_scalar_type_uint32_array(&mut self) -> WebidlScalarType {
        WebidlScalarType::Uint32Array
    }
    fn webidl_scalar_type_uint8_clamped_array(&mut self) -> WebidlScalarType {
        WebidlScalarType::Uint8ClampedArray
    }
    fn webidl_scalar_type_float32_array(&mut self) -> WebidlScalarType {
        WebidlScalarType::Float32Array
    }
    fn webidl_scalar_type_float64_array(&mut self) -> WebidlScalarType {
        WebidlScalarType::Float64Array
    }

    type WasmValType = walrus::ValType;
    fn wasm_val_type_i32(&mut self) -> walrus::ValType {
        walrus::ValType::I32
    }
    fn wasm_val_type_i64(&mut self) -> walrus::ValType {
        walrus::ValType::I64
    }
    fn wasm_val_type_f32(&mut self) -> walrus::ValType {
        walrus::ValType::F32
    }
    fn wasm_val_type_f64(&mut self) -> walrus::ValType {
        walrus::ValType::F64
    }
    fn wasm_val_type_v128(&mut self) -> walrus::ValType {
        walrus::ValType::V128
    }
    fn wasm_val_type_anyref(&mut self) -> walrus::ValType {
        walrus::ValType::Anyref
    }

    type WasmFuncTypeRef = WasmFuncTypeRef;

    type WasmFuncTypeRefNamed = WasmFuncTypeRefNamed;
    fn wasm_func_type_ref_named(&mut self, name: &str) -> WasmFuncTypeRefNamed {
        let name = name.to_string();
        WasmFuncTypeRefNamed { name }
    }

    type WasmFuncTypeRefIndexed = WasmFuncTypeRefIndexed;
    fn wasm_func_type_ref_indexed(&mut self, idx: u32) -> WasmFuncTypeRefIndexed {
        WasmFuncTypeRefIndexed { idx }
    }

    type WasmFuncRef = WasmFuncRef;

    type WasmFuncRefNamed = WasmFuncRefNamed;
    fn wasm_func_ref_named(&mut self, name: &str) -> WasmFuncRefNamed {
        let name = name.to_string();
        WasmFuncRefNamed { name }
    }

    type WasmFuncRefIndexed = WasmFuncRefIndexed;
    fn wasm_func_ref_indexed(&mut self, idx: u32) -> WasmFuncRefIndexed {
        WasmFuncRefIndexed { idx }
    }

    type BindingRef = BindingRef;

    type BindingRefNamed = BindingRefNamed;
    fn binding_ref_named(&mut self, name: &str) -> BindingRefNamed {
        let name = name.to_string();
        BindingRefNamed { name }
    }

    type BindingRefIndexed = BindingRefIndexed;
    fn binding_ref_indexed(&mut self, idx: u32) -> BindingRefIndexed {
        BindingRefIndexed { idx }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebidlBindingsSection {
    pub types: WebidlTypeSubsection,
    pub bindings: WebidlFunctionBindingsSubsection,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebidlTypeSubsection {
    pub types: Vec<WebidlType>,
}

impl From<Vec<WebidlType>> for WebidlTypeSubsection {
    fn from(types: Vec<WebidlType>) -> Self {
        WebidlTypeSubsection { types }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebidlType {
    pub name: Option<String>,
    pub ty: WebidlCompoundType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WebidlCompoundType {
    Function(WebidlFunction),
    Dictionary(WebidlDictionary),
    Enumeration(WebidlEnumeration),
    Union(WebidlUnion),
}

impl From<WebidlFunction> for WebidlCompoundType {
    fn from(a: WebidlFunction) -> Self {
        WebidlCompoundType::Function(a)
    }
}

impl From<WebidlDictionary> for WebidlCompoundType {
    fn from(a: WebidlDictionary) -> Self {
        WebidlCompoundType::Dictionary(a)
    }
}

impl From<WebidlEnumeration> for WebidlCompoundType {
    fn from(a: WebidlEnumeration) -> Self {
        WebidlCompoundType::Enumeration(a)
    }
}

impl From<WebidlUnion> for WebidlCompoundType {
    fn from(a: WebidlUnion) -> Self {
        WebidlCompoundType::Union(a)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebidlFunction {
    pub kind: WebidlFunctionKind,
    pub params: Vec<WebidlTypeRef>,
    pub result: Option<WebidlTypeRef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WebidlFunctionKind {
    Static,
    Method(WebidlFunctionKindMethod),
    Constructor,
}

impl From<WebidlFunctionKindMethod> for WebidlFunctionKind {
    fn from(a: WebidlFunctionKindMethod) -> Self {
        WebidlFunctionKind::Method(a)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebidlFunctionKindMethod {
    pub ty: WebidlTypeRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebidlDictionary {
    pub fields: Vec<WebidlDictionaryField>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebidlDictionaryField {
    pub name: String,
    pub ty: WebidlTypeRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebidlEnumeration {
    pub values: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebidlUnion {
    pub members: Vec<WebidlTypeRef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebidlFunctionBindingsSubsection {
    pub bindings: Vec<FunctionBinding>,
    pub binds: Vec<Bind>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FunctionBinding {
    Import(ImportBinding),
    Export(ExportBinding),
}

impl From<ImportBinding> for FunctionBinding {
    fn from(a: ImportBinding) -> Self {
        FunctionBinding::Import(a)
    }
}

impl From<ExportBinding> for FunctionBinding {
    fn from(a: ExportBinding) -> Self {
        FunctionBinding::Export(a)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportBinding {
    pub name: Option<String>,
    pub wasm_ty: WasmFuncTypeRef,
    pub webidl_ty: WebidlTypeRef,
    pub params: OutgoingBindingMap,
    pub result: IncomingBindingMap,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportBinding {
    pub name: Option<String>,
    pub wasm_ty: WasmFuncTypeRef,
    pub webidl_ty: WebidlTypeRef,
    pub params: IncomingBindingMap,
    pub result: OutgoingBindingMap,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bind {
    pub func: WasmFuncRef,
    pub binding: BindingRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingBindingMap {
    pub bindings: Vec<OutgoingBindingExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncomingBindingMap {
    pub bindings: Vec<IncomingBindingExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutgoingBindingExpression {
    As(OutgoingBindingExpressionAs),
    Utf8Str(OutgoingBindingExpressionUtf8Str),
    Utf8CStr(OutgoingBindingExpressionUtf8CStr),
    I32ToEnum(OutgoingBindingExpressionI32ToEnum),
    View(OutgoingBindingExpressionView),
    Copy(OutgoingBindingExpressionCopy),
    Dict(OutgoingBindingExpressionDict),
    BindExport(OutgoingBindingExpressionBindExport),
}

impl From<OutgoingBindingExpressionAs> for OutgoingBindingExpression {
    fn from(a: OutgoingBindingExpressionAs) -> Self {
        OutgoingBindingExpression::As(a)
    }
}

impl From<OutgoingBindingExpressionUtf8Str> for OutgoingBindingExpression {
    fn from(s: OutgoingBindingExpressionUtf8Str) -> Self {
        OutgoingBindingExpression::Utf8Str(s)
    }
}

impl From<OutgoingBindingExpressionUtf8CStr> for OutgoingBindingExpression {
    fn from(s: OutgoingBindingExpressionUtf8CStr) -> Self {
        OutgoingBindingExpression::Utf8CStr(s)
    }
}

impl From<OutgoingBindingExpressionI32ToEnum> for OutgoingBindingExpression {
    fn from(s: OutgoingBindingExpressionI32ToEnum) -> Self {
        OutgoingBindingExpression::I32ToEnum(s)
    }
}

impl From<OutgoingBindingExpressionView> for OutgoingBindingExpression {
    fn from(s: OutgoingBindingExpressionView) -> Self {
        OutgoingBindingExpression::View(s)
    }
}

impl From<OutgoingBindingExpressionCopy> for OutgoingBindingExpression {
    fn from(s: OutgoingBindingExpressionCopy) -> Self {
        OutgoingBindingExpression::Copy(s)
    }
}

impl From<OutgoingBindingExpressionDict> for OutgoingBindingExpression {
    fn from(s: OutgoingBindingExpressionDict) -> Self {
        OutgoingBindingExpression::Dict(s)
    }
}

impl From<OutgoingBindingExpressionBindExport> for OutgoingBindingExpression {
    fn from(s: OutgoingBindingExpressionBindExport) -> Self {
        OutgoingBindingExpression::BindExport(s)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingBindingExpressionAs {
    pub ty: WebidlTypeRef,
    pub idx: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingBindingExpressionUtf8Str {
    pub ty: WebidlTypeRef,
    pub offset: u32,
    pub length: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingBindingExpressionUtf8CStr {
    pub ty: WebidlTypeRef,
    pub offset: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingBindingExpressionI32ToEnum {
    pub ty: WebidlTypeRef,
    pub idx: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingBindingExpressionView {
    pub ty: WebidlTypeRef,
    pub offset: u32,
    pub length: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingBindingExpressionCopy {
    pub ty: WebidlTypeRef,
    pub offset: u32,
    pub length: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingBindingExpressionDict {
    pub ty: WebidlTypeRef,
    pub fields: Vec<OutgoingBindingExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingBindingExpressionBindExport {
    pub ty: WebidlTypeRef,
    pub binding: BindingRef,
    pub idx: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IncomingBindingExpression {
    Get(IncomingBindingExpressionGet),
    As(IncomingBindingExpressionAs),
    AllocUtf8Str(IncomingBindingExpressionAllocUtf8Str),
    AllocCopy(IncomingBindingExpressionAllocCopy),
    EnumToI32(IncomingBindingExpressionEnumToI32),
    Field(IncomingBindingExpressionField),
    BindImport(IncomingBindingExpressionBindImport),
}

impl From<IncomingBindingExpressionGet> for IncomingBindingExpression {
    fn from(a: IncomingBindingExpressionGet) -> Self {
        IncomingBindingExpression::Get(a)
    }
}

impl From<IncomingBindingExpressionAs> for IncomingBindingExpression {
    fn from(a: IncomingBindingExpressionAs) -> Self {
        IncomingBindingExpression::As(a)
    }
}

impl From<IncomingBindingExpressionAllocUtf8Str> for IncomingBindingExpression {
    fn from(a: IncomingBindingExpressionAllocUtf8Str) -> Self {
        IncomingBindingExpression::AllocUtf8Str(a)
    }
}

impl From<IncomingBindingExpressionAllocCopy> for IncomingBindingExpression {
    fn from(a: IncomingBindingExpressionAllocCopy) -> Self {
        IncomingBindingExpression::AllocCopy(a)
    }
}

impl From<IncomingBindingExpressionEnumToI32> for IncomingBindingExpression {
    fn from(a: IncomingBindingExpressionEnumToI32) -> Self {
        IncomingBindingExpression::EnumToI32(a)
    }
}

impl From<IncomingBindingExpressionField> for IncomingBindingExpression {
    fn from(a: IncomingBindingExpressionField) -> Self {
        IncomingBindingExpression::Field(a)
    }
}

impl From<IncomingBindingExpressionBindImport> for IncomingBindingExpression {
    fn from(a: IncomingBindingExpressionBindImport) -> Self {
        IncomingBindingExpression::BindImport(a)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncomingBindingExpressionGet {
    pub idx: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncomingBindingExpressionAs {
    pub ty: walrus::ValType,
    pub expr: Box<IncomingBindingExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncomingBindingExpressionAllocUtf8Str {
    pub alloc_func_name: String,
    pub expr: Box<IncomingBindingExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncomingBindingExpressionAllocCopy {
    pub alloc_func_name: String,
    pub expr: Box<IncomingBindingExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncomingBindingExpressionEnumToI32 {
    pub ty: WebidlTypeRef,
    pub expr: Box<IncomingBindingExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncomingBindingExpressionField {
    pub idx: u32,
    pub expr: Box<IncomingBindingExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncomingBindingExpressionBindImport {
    pub ty: WasmFuncTypeRef,
    pub binding: BindingRef,
    pub expr: Box<IncomingBindingExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WebidlTypeRef {
    Named(WebidlTypeRefNamed),
    Indexed(WebidlTypeRefIndexed),
    Scalar(WebidlScalarType),
}

impl From<WebidlTypeRefNamed> for WebidlTypeRef {
    fn from(n: WebidlTypeRefNamed) -> Self {
        WebidlTypeRef::Named(n)
    }
}

impl From<WebidlTypeRefIndexed> for WebidlTypeRef {
    fn from(i: WebidlTypeRefIndexed) -> Self {
        WebidlTypeRef::Indexed(i)
    }
}

impl From<WebidlScalarType> for WebidlTypeRef {
    fn from(s: WebidlScalarType) -> Self {
        WebidlTypeRef::Scalar(s)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebidlTypeRefNamed {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebidlTypeRefIndexed {
    pub idx: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WebidlScalarType {
    Any,
    Boolean,
    Byte,
    Octet,
    Long,
    UnsignedLong,
    Short,
    UnsignedShort,
    LongLong,
    UnsignedLongLong,
    Float,
    UnrestrictedFloat,
    Double,
    UnrestrictedDouble,
    DomString,
    ByteString,
    UsvString,
    Object,
    Symbol,
    ArrayBuffer,
    DataView,
    Int8Array,
    Int16Array,
    Int32Array,
    Uint8Array,
    Uint16Array,
    Uint32Array,
    Uint8ClampedArray,
    Float32Array,
    Float64Array,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WasmFuncTypeRef {
    Named(WasmFuncTypeRefNamed),
    Indexed(WasmFuncTypeRefIndexed),
}

impl From<WasmFuncTypeRefNamed> for WasmFuncTypeRef {
    fn from(n: WasmFuncTypeRefNamed) -> Self {
        WasmFuncTypeRef::Named(n)
    }
}

impl From<WasmFuncTypeRefIndexed> for WasmFuncTypeRef {
    fn from(i: WasmFuncTypeRefIndexed) -> Self {
        WasmFuncTypeRef::Indexed(i)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WasmFuncTypeRefNamed {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WasmFuncTypeRefIndexed {
    pub idx: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WasmFuncRef {
    Named(WasmFuncRefNamed),
    Indexed(WasmFuncRefIndexed),
}

impl From<WasmFuncRefNamed> for WasmFuncRef {
    fn from(n: WasmFuncRefNamed) -> Self {
        WasmFuncRef::Named(n)
    }
}

impl From<WasmFuncRefIndexed> for WasmFuncRef {
    fn from(i: WasmFuncRefIndexed) -> Self {
        WasmFuncRef::Indexed(i)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WasmFuncRefNamed {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WasmFuncRefIndexed {
    pub idx: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BindingRef {
    Named(BindingRefNamed),
    Indexed(BindingRefIndexed),
}

impl From<BindingRefNamed> for BindingRef {
    fn from(n: BindingRefNamed) -> Self {
        BindingRef::Named(n)
    }
}

impl From<BindingRefIndexed> for BindingRef {
    fn from(i: BindingRefIndexed) -> Self {
        BindingRef::Indexed(i)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BindingRefNamed {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BindingRefIndexed {
    pub idx: u32,
}
