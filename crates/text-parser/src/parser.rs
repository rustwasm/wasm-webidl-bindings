#![allow(unused_imports, dead_code, missing_debug_implementations)]

use crate::actions::Actions;
use crate::lexer;

include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

/// Parse the given straw proposal text format input with custom parse actions.
///
/// Supply an `Actions` to do something while parsing. If you want to construct
/// the default AST, use `wasm_webidl_bindings::text::parse` which uses the
/// `wasm_webidl_bindings::ast::BuildAstActions` to construct the default AST.
pub fn parse_with_actions<A>(
    actions: &mut A,
    input: &str,
) -> Result<A::WebidlBindingsSection, failure::Error>
where
    A: Actions,
{
    let lexer_builder = lexer::LexerBuilder::new();
    let lexer = lexer_builder.lexer(input);

    let ast = WebidlBindingsSectionParser::new()
        .parse(input, actions, lexer)
        .map_err(|e| failure::format_err!("{}", e))?;
    Ok(ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! t {
        ( $( $x:expr )* ) => {
            ParseTree::List(vec![ $( $x.into() ),* ])
        };
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum ParseTree {
        List(Vec<ParseTree>),
        Atom(String),
    }

    impl From<&'_ str> for ParseTree {
        fn from(s: &str) -> ParseTree {
            s.to_string().into()
        }
    }

    impl From<String> for ParseTree {
        fn from(s: String) -> ParseTree {
            ParseTree::Atom(s)
        }
    }

    impl From<u32> for ParseTree {
        fn from(x: u32) -> ParseTree {
            ParseTree::Atom(x.to_string())
        }
    }

    impl From<Vec<ParseTree>> for ParseTree {
        fn from(v: Vec<ParseTree>) -> ParseTree {
            ParseTree::List(v)
        }
    }

    impl<T: Into<ParseTree>> From<Option<T>> for ParseTree {
        fn from(t: Option<T>) -> ParseTree {
            if let Some(t) = t {
                t!("Some" t)
            } else {
                t!("None")
            }
        }
    }

    struct BuildParseTree;

    impl crate::actions::Actions for BuildParseTree {
        type WebidlBindingsSection = ParseTree;
        fn webidl_bindings_section(
            &mut self,
            types: Self::WebidlTypeSubsection,
            bindings: Self::WebidlFunctionBindingsSubsection,
        ) -> Self::WebidlBindingsSection {
            t!("WebidlBindingsSection" types bindings)
        }

        type WebidlTypeSubsection = ParseTree;
        fn webidl_type_subsection(
            &mut self,
            types: Vec<Self::WebidlType>,
        ) -> Self::WebidlTypeSubsection {
            t!("WebidlTypeSubsection" types)
        }

        type WebidlType = ParseTree;
        fn webidl_type(
            &mut self,
            name: Option<&str>,
            ty: Self::WebidlCompoundType,
        ) -> Self::WebidlType {
            t!("WebidlType" name ty)
        }

        type WebidlCompoundType = ParseTree;

        type WebidlFunction = ParseTree;
        fn webidl_function(
            &mut self,
            kind: Option<Self::WebidlFunctionKind>,
            params: Option<Self::WebidlFunctionParams>,
            result: Option<Self::WebidlFunctionResult>,
        ) -> Self::WebidlFunction {
            t!("WebidlFunction" kind params result)
        }

        type WebidlFunctionKind = ParseTree;

        type WebidlFunctionKindMethod = ParseTree;
        fn webidl_function_kind_method(
            &mut self,
            ty: Self::WebidlTypeRef,
        ) -> Self::WebidlFunctionKindMethod {
            t!("WebidlFunctionKindMethod" ty)
        }

        type WebidlFunctionKindConstructor = ParseTree;
        fn webidl_function_kind_constructor_default_new_target(
            &mut self,
        ) -> Self::WebidlFunctionKindConstructor {
            t!("WebidlFunctionKindConstructor")
        }

        type WebidlFunctionParams = ParseTree;
        fn webidl_function_params(
            &mut self,
            tys: Vec<Self::WebidlTypeRef>,
        ) -> Self::WebidlFunctionParams {
            t!("WebidlFunctionParams" tys)
        }

        type WebidlFunctionResult = ParseTree;
        fn webidl_function_result(
            &mut self,
            ty: Self::WebidlTypeRef,
        ) -> Self::WebidlFunctionResult {
            t!("WebidlFunctionResult" ty)
        }

        type WebidlDictionary = ParseTree;
        fn webidl_dictionary(
            &mut self,
            fields: Vec<Self::WebidlDictionaryField>,
        ) -> Self::WebidlDictionary {
            t!("WebidlDictionary" fields)
        }

        type WebidlDictionaryField = ParseTree;
        fn webidl_dictionary_field(
            &mut self,
            name: Self::WebidlDictionaryFieldName,
            ty: Self::WebidlTypeRef,
        ) -> Self::WebidlDictionaryField {
            t!("WebidlDictionaryField" name ty)
        }

        type WebidlDictionaryFieldName = ParseTree;
        fn webidl_dictionary_field_name(&mut self, name: &str) -> Self::WebidlDictionaryFieldName {
            t!("WebidlDictionaryFieldName" name)
        }

        type WebidlEnumeration = ParseTree;
        fn webidl_enumeration(
            &mut self,
            values: Vec<Self::WebidlEnumerationValue>,
        ) -> Self::WebidlEnumeration {
            t!("WebidlEnumeration" values)
        }

        type WebidlEnumerationValue = ParseTree;
        fn webidl_enumeration_value(&mut self, value: &str) -> Self::WebidlEnumerationValue {
            t!("WebidlEnumerationValue" value)
        }

        type WebidlUnion = ParseTree;
        fn webidl_union(&mut self, members: Vec<Self::WebidlTypeRef>) -> Self::WebidlUnion {
            t!("WebidlUnion" members)
        }

        type WebidlFunctionBindingsSubsection = ParseTree;
        fn webidl_function_bindings_subsection(
            &mut self,
            bindings: Vec<Self::FunctionBinding>,
            binds: Vec<Self::Bind>,
        ) -> Self::WebidlFunctionBindingsSubsection {
            t!("WebidlFunctionBindingsSubsection" bindings binds)
        }

        type FunctionBinding = ParseTree;

        type ImportBinding = ParseTree;
        fn import_binding(
            &mut self,
            name: Option<&str>,
            wasm_ty: Self::WasmFuncTypeRef,
            webidl_ty: Self::WebidlTypeRef,
            params: Self::OutgoingBindingMap,
            result: Self::IncomingBindingMap,
        ) -> Self::ImportBinding {
            t!("ImportBinding" name wasm_ty webidl_ty params result)
        }

        type ExportBinding = ParseTree;
        fn export_binding(
            &mut self,
            name: Option<&str>,
            wasm_ty: Self::WasmFuncTypeRef,
            webidl_ty: Self::WebidlTypeRef,
            params: Self::IncomingBindingMap,
            result: Self::OutgoingBindingMap,
        ) -> Self::ExportBinding {
            t!("ExportBinding" name wasm_ty webidl_ty params result)
        }

        type Bind = ParseTree;
        fn bind(&mut self, func: Self::WasmFuncRef, binding: Self::BindingRef) -> Self::Bind {
            t!("Bind" func binding)
        }

        type OutgoingBindingMap = ParseTree;
        fn outgoing_binding_map(
            &mut self,
            bindings: Vec<Self::OutgoingBindingExpression>,
        ) -> Self::OutgoingBindingMap {
            t!("OutgoingBindingMap" bindings)
        }

        type IncomingBindingMap = ParseTree;
        fn incoming_binding_map(
            &mut self,
            bindings: Vec<Self::IncomingBindingExpression>,
        ) -> Self::IncomingBindingMap {
            t!("IncomingBindingMap" bindings)
        }

        type OutgoingBindingExpression = ParseTree;

        type OutgoingBindingExpressionAs = ParseTree;
        fn outgoing_binding_expression_as(
            &mut self,
            ty: Self::WebidlTypeRef,
            idx: u32,
        ) -> Self::OutgoingBindingExpressionAs {
            t!("OutgoingBindingExpressionAs" ty idx)
        }

        type OutgoingBindingExpressionUtf8Str = ParseTree;
        fn outgoing_binding_expression_utf8_str(
            &mut self,
            ty: Self::WebidlTypeRef,
            offset: u32,
            length: u32,
        ) -> Self::OutgoingBindingExpressionUtf8Str {
            t!("OutgoingBindingExpressionUtf8Str" ty offset length)
        }

        type OutgoingBindingExpressionUtf8CStr = ParseTree;
        fn outgoing_binding_expression_utf8_c_str(
            &mut self,
            ty: Self::WebidlTypeRef,
            offset: u32,
        ) -> Self::OutgoingBindingExpressionUtf8CStr {
            t!("OutgoingBindingExpressionUtf8CStr" ty offset)
        }

        type OutgoingBindingExpressionI32ToEnum = ParseTree;
        fn outgoing_binding_expression_i32_to_enum(
            &mut self,
            ty: Self::WebidlTypeRef,
            idx: u32,
        ) -> Self::OutgoingBindingExpressionI32ToEnum {
            t!("OutgoingBindingExpressionI32ToEnum" ty idx)
        }

        type OutgoingBindingExpressionView = ParseTree;
        fn outgoing_binding_expression_view(
            &mut self,
            ty: Self::WebidlTypeRef,
            offset: u32,
            length: u32,
        ) -> Self::OutgoingBindingExpressionView {
            t!("OutgoingBindingExpressionView" ty offset length)
        }

        type OutgoingBindingExpressionCopy = ParseTree;
        fn outgoing_binding_expression_copy(
            &mut self,
            ty: Self::WebidlTypeRef,
            offset: u32,
            length: u32,
        ) -> Self::OutgoingBindingExpressionCopy {
            t!("OutgoingBindingExpressionCopy" ty offset length)
        }

        type OutgoingBindingExpressionDict = ParseTree;
        fn outgoing_binding_expression_dict(
            &mut self,
            ty: Self::WebidlTypeRef,
            fields: Vec<Self::OutgoingBindingExpression>,
        ) -> Self::OutgoingBindingExpressionDict {
            t!("OutgoingBindingExpressionDict" ty fields)
        }

        type OutgoingBindingExpressionBindExport = ParseTree;
        fn outgoing_binding_expression_bind_export(
            &mut self,
            ty: Self::WebidlTypeRef,
            binding: Self::BindingRef,
            idx: u32,
        ) -> Self::OutgoingBindingExpressionBindExport {
            t!("OutgoingBindingExpressionBindExport" ty binding idx)
        }

        type IncomingBindingExpression = ParseTree;

        type IncomingBindingExpressionGet = ParseTree;
        fn incoming_binding_expression_get(
            &mut self,
            idx: u32,
        ) -> Self::IncomingBindingExpressionGet {
            t!("IncomingBindingExpressionGet" idx)
        }

        type IncomingBindingExpressionAs = ParseTree;
        fn incoming_binding_expression_as(
            &mut self,
            ty: Self::WasmValType,
            expr: Self::IncomingBindingExpression,
        ) -> Self::IncomingBindingExpressionAs {
            t!("IncomingBindingExpressionAs" ty expr)
        }

        type IncomingBindingExpressionAllocUtf8Str = ParseTree;
        fn incoming_binding_expression_alloc_utf8_str(
            &mut self,
            alloc_func_name: &str,
            expr: Self::IncomingBindingExpression,
        ) -> Self::IncomingBindingExpressionAllocUtf8Str {
            t!("IncomingBindingExpressionAllocUtf8Str" alloc_func_name expr)
        }

        type IncomingBindingExpressionAllocCopy = ParseTree;
        fn incoming_binding_expression_alloc_copy(
            &mut self,
            alloc_func_name: &str,
            expr: Self::IncomingBindingExpression,
        ) -> Self::IncomingBindingExpressionAllocCopy {
            t!("IncomingBindingExpressionAllocCopy" alloc_func_name expr)
        }

        type IncomingBindingExpressionEnumToI32 = ParseTree;
        fn incoming_binding_expression_enum_to_i32(
            &mut self,
            ty: Self::WebidlTypeRef,
            expr: Self::IncomingBindingExpression,
        ) -> Self::IncomingBindingExpressionEnumToI32 {
            t!("IncomingBindingExpressionEnumToI32" ty expr)
        }

        type IncomingBindingExpressionField = ParseTree;
        fn incoming_binding_expression_field(
            &mut self,
            idx: u32,
            expr: Self::IncomingBindingExpression,
        ) -> Self::IncomingBindingExpressionField {
            t!("IncomingBindingExpressionField" idx expr)
        }

        type IncomingBindingExpressionBindImport = ParseTree;
        fn incoming_binding_expression_bind_import(
            &mut self,
            ty: Self::WasmFuncTypeRef,
            binding: Self::BindingRef,
            expr: Self::IncomingBindingExpression,
        ) -> Self::IncomingBindingExpressionBindImport {
            t!("IncomingBindingExpressionBindImport" ty binding expr)
        }

        type WebidlTypeRef = ParseTree;

        type WebidlTypeRefNamed = ParseTree;
        fn webidl_type_ref_named(&mut self, name: &str) -> Option<Self::WebidlTypeRefNamed> {
            Some(t!("WebidlTypeRefNamed" name))
        }

        type WebidlTypeRefIndexed = ParseTree;
        fn webidl_type_ref_indexed(&mut self, idx: u32) -> Option<Self::WebidlTypeRefIndexed> {
            Some(t!("WebidlTypeRefIndexed" idx))
        }

        type WebidlScalarType = ParseTree;
        fn webidl_scalar_type_any(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "any")
        }
        fn webidl_scalar_type_boolean(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "boolean")
        }
        fn webidl_scalar_type_byte(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "byte")
        }
        fn webidl_scalar_type_octet(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "octet")
        }
        fn webidl_scalar_type_long(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "long")
        }
        fn webidl_scalar_type_unsigned_long(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "unsigned long")
        }
        fn webidl_scalar_type_short(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "short")
        }
        fn webidl_scalar_type_unsigned_short(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "unsigned short")
        }
        fn webidl_scalar_type_long_long(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "long long")
        }
        fn webidl_scalar_type_unsigned_long_long(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "unsigned long long")
        }
        fn webidl_scalar_type_float(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "float")
        }
        fn webidl_scalar_type_unrestricted_float(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "unrestricted float")
        }
        fn webidl_scalar_type_double(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "double")
        }
        fn webidl_scalar_type_unrestricted_double(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "unrestricted double")
        }
        fn webidl_scalar_type_dom_string(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "DOMString")
        }
        fn webidl_scalar_type_byte_string(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "ByteString")
        }
        fn webidl_scalar_type_usv_string(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "USVString")
        }
        fn webidl_scalar_type_object(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "object")
        }
        fn webidl_scalar_type_symbol(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "symbol")
        }
        fn webidl_scalar_type_array_buffer(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "ArrayBuffer")
        }
        fn webidl_scalar_type_data_view(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "DataView")
        }
        fn webidl_scalar_type_int8_array(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "Int8Array")
        }
        fn webidl_scalar_type_int16_array(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "Int16Array")
        }
        fn webidl_scalar_type_int32_array(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "Int32Array")
        }
        fn webidl_scalar_type_uint8_array(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "Uint8Array")
        }
        fn webidl_scalar_type_uint16_array(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "Uint16Array")
        }
        fn webidl_scalar_type_uint32_array(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "Uint32Array")
        }
        fn webidl_scalar_type_uint8_clamped_array(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "Uint8ClampedArray")
        }
        fn webidl_scalar_type_float32_array(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "Float32Array")
        }
        fn webidl_scalar_type_float64_array(&mut self) -> Self::WebidlScalarType {
            t!("WebidlScalarType" "Float64Array")
        }

        type WasmValType = ParseTree;
        fn wasm_val_type_i32(&mut self) -> Self::WasmValType {
            t!("WasmValType" "i32")
        }
        fn wasm_val_type_i64(&mut self) -> Self::WasmValType {
            t!("WasmValType" "i64")
        }
        fn wasm_val_type_f32(&mut self) -> Self::WasmValType {
            t!("WasmValType" "f32")
        }
        fn wasm_val_type_f64(&mut self) -> Self::WasmValType {
            t!("WasmValType" "f64")
        }
        fn wasm_val_type_v128(&mut self) -> Self::WasmValType {
            t!("WasmValType" "v128")
        }
        fn wasm_val_type_anyref(&mut self) -> Self::WasmValType {
            t!("WasmValType" "anyref")
        }

        type WasmFuncTypeRef = ParseTree;

        type WasmFuncTypeRefNamed = ParseTree;
        fn wasm_func_type_ref_named(&mut self, name: &str) -> Option<Self::WasmFuncTypeRefNamed> {
            Some(t!("WasmFuncTypeRefNamed" name))
        }

        type WasmFuncTypeRefIndexed = ParseTree;
        fn wasm_func_type_ref_indexed(&mut self, idx: u32) -> Option<Self::WasmFuncTypeRefIndexed> {
            Some(t!("WasmFuncTypeRefIndexed" idx))
        }

        type WasmFuncRef = ParseTree;

        type WasmFuncRefNamed = ParseTree;
        fn wasm_func_ref_named(&mut self, name: &str) -> Option<Self::WasmFuncRefNamed> {
            Some(t!("WasmFuncRefNamed" name))
        }

        type WasmFuncRefIndexed = ParseTree;
        fn wasm_func_ref_indexed(&mut self, idx: u32) -> Option<Self::WasmFuncRefIndexed> {
            Some(t!("WasmFuncRefIndexed" idx))
        }

        type BindingRef = ParseTree;

        type BindingRefNamed = ParseTree;
        fn binding_ref_named(&mut self, name: &str) -> Option<Self::BindingRefNamed> {
            Some(t!("BindingRefNamed" name))
        }

        type BindingRefIndexed = ParseTree;
        fn binding_ref_indexed(&mut self, idx: u32) -> Option<Self::BindingRefIndexed> {
            Some(t!("BindingRefIndexed" idx))
        }
    }

    macro_rules! ok {
        ($name: ident, $parser: ident, $input: expr, $output: expr) => {
            #[test]
            fn $name() {
                let actions = &mut BuildParseTree;
                let lexer_builder = crate::lexer::LexerBuilder::new();
                let lexer = lexer_builder.lexer($input);

                let actual = $parser::new().parse($input, actions, lexer).unwrap();
                let expected = $output;
                println!("actual = {:#?}", actual);
                println!("expected = {:#?}", expected);
                assert_eq!(actual, expected);
            }
        };
    }

    macro_rules! err {
        ($name: ident, $parser: ident, $input: expr) => {
            #[test]
            fn $name() {
                let actions = &mut BuildParseTree;
                let lexer_builder = crate::lexer::LexerBuilder::new();
                let lexer = lexer_builder.lexer($input);

                assert!($parser::new().parse($input, actions, lexer).is_err());
            }
        };
    }

    ok!(
        explainer_example,
        WebidlBindingsSectionParser,
        // The Wasm type and func that are being bound are:
        //
        //     (type $EncodeIntoFuncWasm
        //       (param anyref anyref i32 i32)
        //       (result i64 i64))
        //
        //     (func $encodeInto
        //       (import "TextEncoder" "encodeInto")
        //       (type $EncodeIntoFuncWasm))
        r#"
        ;; Define the signature of `encodeInto`.
        type $TextEncoderEncodeIntoResult
          (; a dictionary with 2 fields: `read` and `written`. ; dict
            (field "read" unsigned long long)
            (field "written" unsigned long long))

        type $EncodeIntoFuncWebIDL
           (func (method any)
              (param USVString Uint8Array)
              (result $TextEncoderEncodeIntoResult))

        ;; Apply the binding.
        func-binding $encodeIntoBinding import $EncodeIntoFuncWasm $EncodeIntoFuncWebIDL
          (param
            (as any 0)
            (as any 1)
            (view Uint8Array 2 3))
          (result
            (as i64 (field 0 (get 0)))
            (as i64 (field 1 (get 0))))

        bind $encodeInto $encodeIntoBinding
        "#,
        t!("WebidlBindingsSection"
           t!("WebidlTypeSubsection"
              t!(t!("WebidlType"
                    t!("Some" "$TextEncoderEncodeIntoResult")
                    t!("WebidlDictionary"
                       t!(t!("WebidlDictionaryField"
                             t!("WebidlDictionaryFieldName" "read")
                             t!("WebidlScalarType" "unsigned long long"))
                          t!("WebidlDictionaryField"
                             t!("WebidlDictionaryFieldName" "written")
                             t!("WebidlScalarType" "unsigned long long")))))
                 t!("WebidlType"
                    t!("Some" "$EncodeIntoFuncWebIDL")
                    t!("WebidlFunction"
                       t!("Some" t!("WebidlFunctionKindMethod" t!("WebidlScalarType" "any")))
                       t!("Some" t!("WebidlFunctionParams"
                                    t!(t!("WebidlScalarType" "USVString")
                                       t!("WebidlScalarType" "Uint8Array"))))
                       t!("Some" t!("WebidlFunctionResult"
                                    t!("WebidlTypeRefNamed" "$TextEncoderEncodeIntoResult")))))))
           t!("WebidlFunctionBindingsSubsection"
              t!(t!("ImportBinding"
                    t!("Some" "$encodeIntoBinding")
                    t!("WasmFuncTypeRefNamed" "$EncodeIntoFuncWasm")
                    t!("WebidlTypeRefNamed" "$EncodeIntoFuncWebIDL")
                    t!("OutgoingBindingMap"
                       t!(t!("OutgoingBindingExpressionAs"
                             t!("WebidlScalarType" "any")
                             0)
                          t!("OutgoingBindingExpressionAs"
                             t!("WebidlScalarType" "any")
                             1)
                          t!("OutgoingBindingExpressionView"
                             t!("WebidlScalarType" "Uint8Array")
                             2
                             3)))
                    t!("IncomingBindingMap"
                       t!(t!("IncomingBindingExpressionAs"
                             t!("WasmValType" "i64")
                             t!("IncomingBindingExpressionField"
                                0
                                t!("IncomingBindingExpressionGet" 0)))
                          t!("IncomingBindingExpressionAs"
                             t!("WasmValType" "i64")
                             t!("IncomingBindingExpressionField"
                                1
                                t!("IncomingBindingExpressionGet" 0)))))))
                t!(t!("Bind"
                      t!("WasmFuncRefNamed" "$encodeInto")
                      t!("BindingRefNamed" "$encodeIntoBinding")))))
    );

    ok!(
        webidl_type_func_ok_1,
        WebidlTypeParser,
        "type $AddContactFuncWebIDL (func (method any) (param $Contact DOMString) (result boolean))",
        t!("WebidlType"
           t!("Some" "$AddContactFuncWebIDL")
           t!("WebidlFunction"
              t!("Some" t!("WebidlFunctionKindMethod"
                           t!("WebidlScalarType" "any")))
              t!("Some" t!("WebidlFunctionParams"
                           t!(t!("WebidlTypeRefNamed" "$Contact")
                              t!("WebidlScalarType" "DOMString"))))
              t!("Some" t!("WebidlFunctionResult" t!("WebidlScalarType" "boolean")))))
    );
    ok!(
        webidl_type_func_ok_2,
        WebidlTypeParser,
        "type $AddContactFuncWebIDL (func (method any) (param $Contact DOMString))",
        t!("WebidlType"
           t!("Some" "$AddContactFuncWebIDL")
           t!("WebidlFunction"
              t!("Some" t!("WebidlFunctionKindMethod"
                           t!("WebidlScalarType" "any")))
              t!("Some" t!("WebidlFunctionParams"
                           t!(t!("WebidlTypeRefNamed" "$Contact")
                              t!("WebidlScalarType" "DOMString"))))
              t!("None")))
    );
    ok!(
        webidl_type_func_ok_3,
        WebidlTypeParser,
        "type $AddContactFuncWebIDL (func (param DOMString))",
        t!("WebidlType"
           t!("Some" "$AddContactFuncWebIDL")
           t!("WebidlFunction"
              t!("None")
              t!("Some" t!("WebidlFunctionParams" t!(t!("WebidlScalarType" "DOMString"))))
              t!("None")))
    );
    ok!(
        webidl_type_func_ok_4,
        WebidlTypeParser,
        "type $AddContactFuncWebIDL (func (param))",
        t!("WebidlType"
           t!("Some" "$AddContactFuncWebIDL")
           t!("WebidlFunction"
              t!("None")
              t!("Some" t!("WebidlFunctionParams" t!()))
              t!("None")))
    );
    ok!(
        webidl_type_func_ok_5,
        WebidlTypeParser,
        "type $AddContactFuncWebIDL (func)",
        t!("WebidlType"
           t!("Some" "$AddContactFuncWebIDL")
           t!("WebidlFunction"
              t!("None")
              t!("None")
              t!("None")))
    );
    ok!(
        webidl_type_func_ok_6,
        WebidlTypeParser,
        "type (func)",
        t!("WebidlType"
           t!("None")
           t!("WebidlFunction"
              t!("None")
              t!("None")
              t!("None")))
    );
    ok!(
        webidl_type_func_ok_7,
        WebidlTypeParser,
        "type MyCtor (func (constructor default-new-target) (result any))",
        t!("WebidlType"
           t!("Some" "MyCtor")
           t!("WebidlFunction"
              t!("Some" t!("WebidlFunctionKindConstructor"))
              t!("None")
              t!("Some" t!("WebidlFunctionResult" t!("WebidlScalarType" "any")))))
    );
    err!(
        webidl_type_func_err_1,
        WebidlTypeParser,
        "type blahBlahBlah"
    );
    err!(
        webidl_type_func_err_2,
        WebidlTypeParser,
        "type blahBlahBlah (func (result any) (result any))"
    );
    err!(
        webidl_type_func_err_3,
        WebidlTypeParser,
        "type blahBlahBlah (func (method any) (method any))"
    );

    ok!(
        webidl_type_dict_ok_1,
        WebidlTypeParser,
        r#"type $Contact (dict (field "name" DOMString) (field "age" long))"#,
        t!("WebidlType"
           t!("Some" "$Contact")
           t!("WebidlDictionary"
              t!(t!("WebidlDictionaryField"
                    t!("WebidlDictionaryFieldName" "name")
                    t!("WebidlScalarType" "DOMString"))
                 t!("WebidlDictionaryField"
                    t!("WebidlDictionaryFieldName" "age")
                    t!("WebidlScalarType" "long")))))
    );
    ok!(
        webidl_type_dict_ok_2,
        WebidlTypeParser,
        r#"type $Contact (dict)"#,
        t!("WebidlType"
           t!("Some" "$Contact")
           t!("WebidlDictionary" t!()))
    );
    err!(
        webidl_type_dict_err_1,
        WebidlTypeParser,
        r#"type $Contact (dict (field "name"))"#
    );
    err!(
        webidl_type_dict_err_2,
        WebidlTypeParser,
        r#"type $Contact (dict (field DOMString))"#
    );

    ok!(
        webidl_type_enum_ok_1,
        WebidlTypeParser,
        r#"type Blah (enum "uno" "dos" "tres")"#,
        t!("WebidlType"
           t!("Some" "Blah")
           t!("WebidlEnumeration"
              t!(
                  t!("WebidlEnumerationValue" "uno")
                  t!("WebidlEnumerationValue" "dos")
                  t!("WebidlEnumerationValue" "tres")
              )
           )
        )
    );
    ok!(
        webidl_type_enum_ok_2,
        WebidlTypeParser,
        r#"type (enum)"#,
        t!("WebidlType"
           t!("None")
           t!("WebidlEnumeration" t!())
        )
    );
    err!(
        webidl_type_enum_err_1,
        WebidlTypeParser,
        r#"type (enum 1 2 3)"#
    );

    ok!(
        webidl_type_union_ok_1,
        WebidlTypeParser,
        "type MyUnion (union long boolean)",
        t!("WebidlType"
           t!("Some" "MyUnion")
            t!("WebidlUnion"
               t!(
                   t!("WebidlScalarType" "long")
                   t!("WebidlScalarType" "boolean")
               )
            )
        )
    );
    ok!(
        webidl_type_union_ok_2,
        WebidlTypeParser,
        "type (union)",
        t!("WebidlType"
           t!("None")
           t!("WebidlUnion" t!())
        )
    );
    err!(
        webidl_type_union_err_1,
        WebidlTypeParser,
        r#"type (union "hello")"#
    );

    ok!(
        import_binding_ok_1,
        ImportBindingParser,
        "func-binding Yoyo import MyWasmFunc MyWebidlFunc (param (as any 0)) (result (as i32 (get 0)))",
        t!("ImportBinding"
           t!("Some" "Yoyo")
           t!("WasmFuncTypeRefNamed" "MyWasmFunc")
           t!("WebidlTypeRefNamed" "MyWebidlFunc")
           t!("OutgoingBindingMap"
              t!(
                  t!("OutgoingBindingExpressionAs"
                     t!("WebidlScalarType" "any")
                     0
                  )
              )
           )
           t!("IncomingBindingMap"
              t!(
                  t!("IncomingBindingExpressionAs"
                     t!("WasmValType" "i32")
                     t!("IncomingBindingExpressionGet" 0)
                  )
              )
           )
        )
    );
    ok!(
        import_binding_ok_2,
        ImportBindingParser,
        "func-binding import MyWasmFunc MyWebidlFunc (param) (result)",
        t!("ImportBinding"
           t!("None")
           t!("WasmFuncTypeRefNamed" "MyWasmFunc")
           t!("WebidlTypeRefNamed" "MyWebidlFunc")
           t!("OutgoingBindingMap" t!())
           t!("IncomingBindingMap" t!())
        )
    );
    err!(
        import_binding_err_1,
        ImportBindingParser,
        "func-binding import MyWasmFunc MyWebidlFunc (param)"
    );
    err!(
        import_binding_err_2,
        ImportBindingParser,
        "func-binding import MyWasmFunc MyWebidlFunc (result)"
    );
    err!(
        import_binding_err_3,
        ImportBindingParser,
        "func-binding import MyWasmFunc (param) (result)"
    );
    err!(
        import_binding_err_4,
        ImportBindingParser,
        "func-binding import MyWebidlFunc (param) (result)"
    );
    err!(
        import_binding_err_5,
        ImportBindingParser,
        "func-binding MyWasmFunc MyWebidlFunc (param) (result)"
    );
    err!(
        import_binding_err_6,
        ImportBindingParser,
        "import MyWasmFunc MyWebidlFunc (param) (result)"
    );

    ok!(
        export_binding_ok_1,
        ExportBindingParser,
        "func-binding $Yoyo export MyWasmFunc MyWebidlFunc (param (as i32 (get 0))) (result (as any 0))",
        t!("ExportBinding"
           t!("Some" "$Yoyo")
           t!("WasmFuncTypeRefNamed" "MyWasmFunc")
           t!("WebidlTypeRefNamed" "MyWebidlFunc")
           t!("IncomingBindingMap"
              t!(
                  t!("IncomingBindingExpressionAs"
                     t!("WasmValType" "i32")
                     t!("IncomingBindingExpressionGet" 0)
                  )
              )
           )
           t!("OutgoingBindingMap"
              t!(
                  t!("OutgoingBindingExpressionAs"
                     t!("WebidlScalarType" "any")
                     0
                  )
              )
           )
        )
    );
    ok!(
        export_binding_ok_2,
        ExportBindingParser,
        "func-binding export $MyWasmFunc $MyWebidlFunc (param) (result)",
        t!("ExportBinding"
           t!("None")
           t!("WasmFuncTypeRefNamed" "$MyWasmFunc")
           t!("WebidlTypeRefNamed" "$MyWebidlFunc")
           t!("IncomingBindingMap" t!())
           t!("OutgoingBindingMap" t!())
        )
    );
    err!(
        export_binding_err_1,
        ExportBindingParser,
        "func-binding export MyWasmFunc MyWebidlFunc (param)"
    );
    err!(
        export_binding_err_2,
        ExportBindingParser,
        "func-binding export MyWasmFunc MyWebidlFunc (result)"
    );
    err!(
        export_binding_err_3,
        ExportBindingParser,
        "func-binding export MyWasmFunc (param) (result)"
    );
    err!(
        export_binding_err_4,
        ExportBindingParser,
        "func-binding export MyWebidlFunc (param) (result)"
    );
    err!(
        export_binding_err_5,
        ExportBindingParser,
        "func-binding MyWasmFunc MyWebidlFunc (param) (result)"
    );
    err!(
        export_binding_err_6,
        ExportBindingParser,
        "export MyWasmFunc MyWebidlFunc (param) (result)"
    );

    ok!(
        webidl_type_ref_ok_1,
        WebidlTypeRefParser,
        "$Contact",
        t!("WebidlTypeRefNamed" "$Contact")
    );
    ok!(
        webidl_type_ref_ok_2,
        WebidlTypeRefParser,
        "42",
        t!("WebidlTypeRefIndexed" 42)
    );
    err!(webidl_type_ref_err, WebidlTypeRefParser, "1abc");

    ok!(
        wasm_type_ref_ok_1,
        WasmValTypeParser,
        "i32",
        t!("WasmValType" "i32")
    );
    ok!(
        wasm_type_ref_ok_2,
        WasmValTypeParser,
        "i64",
        t!("WasmValType" "i64")
    );
    ok!(
        wasm_type_ref_ok_3,
        WasmValTypeParser,
        "f32",
        t!("WasmValType" "f32")
    );
    ok!(
        wasm_type_ref_ok_4,
        WasmValTypeParser,
        "f64",
        t!("WasmValType" "f64")
    );
    ok!(
        wasm_type_ref_ok_5,
        WasmValTypeParser,
        "v128",
        t!("WasmValType" "v128")
    );
    ok!(
        wasm_type_ref_ok_6,
        WasmValTypeParser,
        "anyref",
        t!("WasmValType" "anyref")
    );
    err!(wasm_type_ref_err, WasmValTypeParser, "a32");

    ok!(
        export_binding_ref_ok_1,
        BindingRefParser,
        "$Contact",
        t!("BindingRefNamed" "$Contact")
    );
    ok!(
        export_binding_ref_ok_2,
        BindingRefParser,
        "42",
        t!("BindingRefIndexed" 42)
    );
    err!(export_binding_ref_err, BindingRefParser, "1abc");

    ok!(
        wasm_func_ref_ok_1,
        WasmFuncRefParser,
        "$my_func",
        t!("WasmFuncRefNamed" "$my_func")
    );
    ok!(
        wasm_func_ref_ok_2,
        WasmFuncRefParser,
        "42",
        t!("WasmFuncRefIndexed" 42)
    );
    err!(wasm_func_ref_err, WasmFuncRefParser, "1abc");

    ok!(
        outgoing_binding_expression_as_ok_1,
        OutgoingBindingExpressionParser,
        "(as long 2)",
        t!("OutgoingBindingExpressionAs"
           t!("WebidlScalarType" "long")
           2
        )
    );
    ok!(
        outgoing_binding_expression_as_ok_2,
        OutgoingBindingExpressionParser,
        "(as 1 2)",
        t!("OutgoingBindingExpressionAs"
           t!("WebidlTypeRefIndexed" 1)
           2
        )
    );
    err!(
        outgoing_binding_expression_as_err_1,
        OutgoingBindingExpressionParser,
        "(as long)"
    );
    err!(
        outgoing_binding_expression_as_err_2,
        OutgoingBindingExpressionParser,
        "(as 2)"
    );

    ok!(
        outgoing_binding_expression_utf8_str_ok,
        OutgoingBindingExpressionParser,
        "(utf8-str DOMString 123 456)",
        t!("OutgoingBindingExpressionUtf8Str"
           t!("WebidlScalarType" "DOMString")
           123
           456
        )
    );
    err!(
        outgoing_binding_expression_utf8_str_err_1,
        OutgoingBindingExpressionParser,
        "(utf8-str DOMString 123)"
    );
    err!(
        outgoing_binding_expression_utf8_str_err_2,
        OutgoingBindingExpressionParser,
        "(utf8-str 123 456)"
    );

    ok!(
        outgoing_binding_expression_utf8_c_str_ok,
        OutgoingBindingExpressionParser,
        "(utf8-cstr DOMString 123)",
        t!("OutgoingBindingExpressionUtf8CStr"
           t!("WebidlScalarType" "DOMString")
           123
        )
    );
    err!(
        outgoing_binding_expression_utf8_c_str_err_1,
        OutgoingBindingExpressionParser,
        "(utf8-cstr DOMString)"
    );
    err!(
        outgoing_binding_expression_utf8_c_str_err_2,
        OutgoingBindingExpressionParser,
        "(utf8-cstr 123)"
    );

    ok!(
        outgoing_binding_expression_i32_to_enum_ok,
        OutgoingBindingExpressionParser,
        "(i32-to-enum Blah 22)",
        t!("OutgoingBindingExpressionI32ToEnum"
           t!("WebidlTypeRefNamed" "Blah")
           22
        )
    );
    err!(
        outgoing_binding_expression_i32_to_enum_err_1,
        OutgoingBindingExpressionParser,
        "(i32-to-enum Blah)"
    );
    err!(
        outgoing_binding_expression_i32_to_enum_err_2,
        OutgoingBindingExpressionParser,
        "(i32-to-enum 22)"
    );

    ok!(
        outgoing_binding_expression_view_ok,
        OutgoingBindingExpressionParser,
        "(view Uint8Array 123 456)",
        t!("OutgoingBindingExpressionView"
           t!("WebidlScalarType" "Uint8Array")
           123
           456
        )
    );
    err!(
        outgoing_binding_expression_view_err_1,
        OutgoingBindingExpressionParser,
        "(view Uint8Array 123)"
    );
    err!(
        outgoing_binding_expression_view_err_2,
        OutgoingBindingExpressionParser,
        "(view 123 456)"
    );

    ok!(
        outgoing_binding_expression_copy_ok,
        OutgoingBindingExpressionParser,
        "(copy Uint8Array 123 456)",
        t!("OutgoingBindingExpressionCopy"
           t!("WebidlScalarType" "Uint8Array")
           123
           456
        )
    );
    err!(
        outgoing_binding_expression_copy_err_1,
        OutgoingBindingExpressionParser,
        "(copy Uint8Array 123)"
    );
    err!(
        outgoing_binding_expression_copy_err_2,
        OutgoingBindingExpressionParser,
        "(copy 123 456)"
    );

    ok!(
        outgoing_binding_expression_dict_ok_1,
        OutgoingBindingExpressionParser,
        "(dict $Contact (utf8-str DOMString 0 1) (as long 2))",
        t!("OutgoingBindingExpressionDict"
           t!("WebidlTypeRefNamed" "$Contact")
           t!(
               t!("OutgoingBindingExpressionUtf8Str"
                  t!("WebidlScalarType" "DOMString")
                  0
                  1)
               t!("OutgoingBindingExpressionAs"
                  t!("WebidlScalarType" "long")
                  2)
           )
        )
    );
    ok!(
        outgoing_binding_expression_dict_ok_2,
        OutgoingBindingExpressionParser,
        "(dict $Contact)",
        t!("OutgoingBindingExpressionDict"
           t!("WebidlTypeRefNamed" "$Contact")
           t!()
        )
    );
    err!(
        outgoing_binding_expression_dict_err_1,
        OutgoingBindingExpressionParser,
        "(dict (as long 1))"
    );

    ok!(
        outgoing_binding_expression_bind_export_ok_1,
        OutgoingBindingExpressionParser,
        "(bind-export $SomeCallback $SomeBinding 2)",
        t!("OutgoingBindingExpressionBindExport"
           t!("WebidlTypeRefNamed" "$SomeCallback")
           t!("BindingRefNamed" "$SomeBinding")
           2
        )
    );
    err!(
        outgoing_binding_expression_bind_export_err_1,
        OutgoingBindingExpressionParser,
        "(bind-export SomeBinding 2)"
    );
    err!(
        outgoing_binding_expression_bind_export_err_2,
        OutgoingBindingExpressionParser,
        "(bind-export SomeCallback 2)"
    );
    err!(
        outgoing_binding_expression_bind_export_err_3,
        OutgoingBindingExpressionParser,
        "(bind-export SomeCallback SomeBinding)"
    );

    ok!(
        incoming_binding_expression_get_ok_1,
        IncomingBindingExpressionParser,
        "(get 9)",
        t!("IncomingBindingExpressionGet" 9)
    );
    err!(
        incoming_binding_expression_get_err_1,
        IncomingBindingExpressionParser,
        "(get)"
    );
    err!(
        incoming_binding_expression_get_err_2,
        IncomingBindingExpressionParser,
        "(get 1 2)"
    );

    ok!(
        incoming_binding_expression_as_ok_1,
        IncomingBindingExpressionParser,
        "(as i32 (get 0))",
        t!("IncomingBindingExpressionAs"
           t!("WasmValType" "i32")
           t!("IncomingBindingExpressionGet" 0)
        )
    );
    err!(
        incoming_binding_expression_as_err_1,
        IncomingBindingExpressionParser,
        "(as i32)"
    );
    err!(
        incoming_binding_expression_as_err_2,
        IncomingBindingExpressionParser,
        "(as (get 1))"
    );

    ok!(
        incoming_binding_expression_alloc_utf8_str_ok_1,
        IncomingBindingExpressionParser,
        "(alloc-utf8-str malloc (get 0))",
        t!("IncomingBindingExpressionAllocUtf8Str"
           "malloc"
           t!("IncomingBindingExpressionGet" 0)
        )
    );
    err!(
        incoming_binding_expression_alloc_utf8_str_err_1,
        IncomingBindingExpressionParser,
        "(alloc-utf8-str (get 0))"
    );
    err!(
        incoming_binding_expression_alloc_utf8_str_err_2,
        IncomingBindingExpressionParser,
        "(alloc-utf8-str malloc)"
    );

    ok!(
        incoming_binding_expression_alloc_copy_ok_1,
        IncomingBindingExpressionParser,
        "(alloc-copy malloc (get 0))",
        t!("IncomingBindingExpressionAllocCopy"
           "malloc"
           t!("IncomingBindingExpressionGet" 0)
        )
    );
    err!(
        incoming_binding_expression_alloc_copy_err_1,
        IncomingBindingExpressionParser,
        "(alloc-copy (get 0))"
    );
    err!(
        incoming_binding_expression_alloc_copy_err_2,
        IncomingBindingExpressionParser,
        "(alloc-copy malloc)"
    );

    ok!(
        incoming_binding_expression_enum_to_i32_ok_1,
        IncomingBindingExpressionParser,
        "(enum-to-i32 Blah (get 0))",
        t!("IncomingBindingExpressionEnumToI32"
           t!("WebidlTypeRefNamed" "Blah")
           t!("IncomingBindingExpressionGet" 0)
        )
    );
    err!(
        incoming_binding_expression_enum_to_i32_err_1,
        IncomingBindingExpressionParser,
        "(enum-to-i32 (get 0))"
    );
    err!(
        incoming_binding_expression_enum_to_i32_err_2,
        IncomingBindingExpressionParser,
        "(enum-to-i32 Blah)"
    );

    ok!(
        incoming_binding_expression_field_ok_1,
        IncomingBindingExpressionParser,
        "(field 0 (get 1))",
        t!("IncomingBindingExpressionField"
           0
           t!("IncomingBindingExpressionGet" 1)
        )
    );
    err!(
        incoming_binding_expression_field_err_1,
        IncomingBindingExpressionParser,
        "(field (get 1))"
    );
    err!(
        incoming_binding_expression_field_err_2,
        IncomingBindingExpressionParser,
        "(field 0)"
    );

    ok!(
        incoming_binding_expression_bind_import_ok_1,
        IncomingBindingExpressionParser,
        "(bind-import hi hello (get 1))",
        t!("IncomingBindingExpressionBindImport"
           t!("WasmFuncTypeRefNamed" "hi")
           t!("BindingRefNamed" "hello")
           t!("IncomingBindingExpressionGet" 1)
        )
    );
    err!(
        incoming_binding_expression_bind_import_err_1,
        IncomingBindingExpressionParser,
        "(bind-import hi hello)"
    );
    err!(
        incoming_binding_expression_bind_import_err_2,
        IncomingBindingExpressionParser,
        "(bind-import hi (get 1))"
    );
    err!(
        incoming_binding_expression_bind_import_err_3,
        IncomingBindingExpressionParser,
        "(bind-import hello (get 1))"
    );
}
