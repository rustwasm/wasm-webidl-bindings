#![allow(unused_imports, dead_code, missing_debug_implementations)]

include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    macro_rules! ok {
        ($name: ident, $parser: ident, $input: expr, $output: expr) => {
            #[test]
            fn $name() {
                let actions = &mut BuildAstActions::default();
                let actual = $parser::new().parse(actions, $input).unwrap();
                let expected = $output;
                assert_eq!(actual, expected);
            }
        };
    }

    macro_rules! err {
        ($name: ident, $parser: ident, $input: expr) => {
            #[test]
            fn $name() {
                let actions = &mut BuildAstActions::default();
                assert!($parser::new().parse(actions, $input).is_err());
            }
        };
    }

    ok!(
        webidl_type_ref_ok_1,
        WebidlTypeRefParser,
        "$Contact",
        WebidlTypeRef::Named(WebidlTypeRefNamed {
            name: "$Contact".into(),
        })
    );
    ok!(
        webidl_type_ref_ok_2,
        WebidlTypeRefParser,
        "42",
        WebidlTypeRef::Indexed(WebidlTypeRefIndexed { idx: 42 })
    );
    err!(webidl_type_ref_err, WebidlTypeRefParser, "1abc");

    ok!(
        wasm_type_ref_ok_1,
        WasmTypeRefParser,
        "$Contact",
        WasmTypeRef::Named(WasmTypeRefNamed {
            name: "$Contact".into(),
        })
    );
    ok!(
        wasm_type_ref_ok_2,
        WasmTypeRefParser,
        "42",
        WasmTypeRef::Indexed(WasmTypeRefIndexed { idx: 42 })
    );
    err!(wasm_type_ref_err, WasmTypeRefParser, "1abc");

    ok!(
        export_binding_ref_ok_1,
        ExportBindingRefParser,
        "$Contact",
        ExportBindingRef::Named(ExportBindingRefNamed {
            name: "$Contact".into(),
        })
    );
    ok!(
        export_binding_ref_ok_2,
        ExportBindingRefParser,
        "42",
        ExportBindingRef::Indexed(ExportBindingRefIndexed { idx: 42 })
    );
    err!(export_binding_ref_err, ExportBindingRefParser, "1abc");

    ok!(
        import_binding_ref_ok_1,
        ImportBindingRefParser,
        "$Contact",
        ImportBindingRef::Named(ImportBindingRefNamed {
            name: "$Contact".into(),
        })
    );
    ok!(
        import_binding_ref_ok_2,
        ImportBindingRefParser,
        "42",
        ImportBindingRef::Indexed(ImportBindingRefIndexed { idx: 42 })
    );
    err!(import_binding_ref_err, ImportBindingRefParser, "1abc");

    ok!(
        outgoing_binding_expression_as_ok_1,
        OutgoingBindingExpressionParser,
        "(as long 2)",
        OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
            ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                name: "long".into()
            }),
            idx: 2
        })
    );
    ok!(
        outgoing_binding_expression_as_ok_2,
        OutgoingBindingExpressionParser,
        "(as 1 2)",
        OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
            ty: WebidlTypeRef::Indexed(WebidlTypeRefIndexed { idx: 1 }),
            idx: 2
        })
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
        OutgoingBindingExpression::Utf8Str(OutgoingBindingExpressionUtf8Str {
            ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                name: "DOMString".into(),
            }),
            offset: 123,
            length: 456,
        })
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
        OutgoingBindingExpression::Utf8CStr(OutgoingBindingExpressionUtf8CStr {
            ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                name: "DOMString".into(),
            }),
            offset: 123,
        })
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
        OutgoingBindingExpression::I32ToEnum(OutgoingBindingExpressionI32ToEnum {
            ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                name: "Blah".into(),
            }),
            idx: 22,
        })
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
        OutgoingBindingExpression::View(OutgoingBindingExpressionView {
            ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                name: "Uint8Array".into(),
            }),
            offset: 123,
            length: 456,
        })
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
        OutgoingBindingExpression::Copy(OutgoingBindingExpressionCopy {
            ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                name: "Uint8Array".into(),
            }),
            offset: 123,
            length: 456,
        })
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
        OutgoingBindingExpression::Dict(OutgoingBindingExpressionDict {
            ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                name: "$Contact".into()
            }),
            fields: vec![
                OutgoingBindingExpression::Utf8Str(OutgoingBindingExpressionUtf8Str {
                    ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                        name: "DOMString".into()
                    }),
                    offset: 0,
                    length: 1,
                }),
                OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                    ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                        name: "long".into()
                    }),
                    idx: 2,
                })
            ]
        })
    );
    ok!(
        outgoing_binding_expression_dict_ok_2,
        OutgoingBindingExpressionParser,
        "(dict $Contact)",
        OutgoingBindingExpression::Dict(OutgoingBindingExpressionDict {
            ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                name: "$Contact".into()
            }),
            fields: vec![]
        })
    );
    err!(
        outgoing_binding_expression_dict_err_1,
        OutgoingBindingExpressionParser,
        "(dict (as long 1))"
    );

    ok!(
        outgoing_binding_expression_bind_export_ok_1,
        OutgoingBindingExpressionParser,
        "(bind-export SomeCallback SomeBinding 2)",
        OutgoingBindingExpression::BindExport(OutgoingBindingExpressionBindExport {
            ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                name: "SomeCallback".into()
            }),
            binding: ExportBindingRef::Named(ExportBindingRefNamed {
                name: "SomeBinding".into()
            }),
            idx: 2,
        })
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
        IncomingBindingExpression::Get(IncomingBindingExpressionGet { idx: 9 })
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
        IncomingBindingExpression::As(IncomingBindingExpressionAs {
            ty: WasmTypeRef::Named(WasmTypeRefNamed { name: "i32".into() }),
            expr: Box::new(IncomingBindingExpression::Get(
                IncomingBindingExpressionGet { idx: 0 }
            )),
        })
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
        IncomingBindingExpression::AllocUtf8Str(IncomingBindingExpressionAllocUtf8Str {
            alloc_func_name: "malloc".into(),
            expr: Box::new(IncomingBindingExpression::Get(
                IncomingBindingExpressionGet { idx: 0 }
            )),
        })
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
        IncomingBindingExpression::AllocCopy(IncomingBindingExpressionAllocCopy {
            alloc_func_name: "malloc".into(),
            expr: Box::new(IncomingBindingExpression::Get(
                IncomingBindingExpressionGet { idx: 0 }
            )),
        })
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
        IncomingBindingExpression::EnumToI32(IncomingBindingExpressionEnumToI32 {
            ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                name: "Blah".into(),
            }),
            expr: Box::new(IncomingBindingExpression::Get(
                IncomingBindingExpressionGet { idx: 0 }
            )),
        })
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
        IncomingBindingExpression::Field(IncomingBindingExpressionField {
            idx: 0,
            expr: Box::new(IncomingBindingExpression::Get(
                IncomingBindingExpressionGet { idx: 1 }
            )),
        })
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
        IncomingBindingExpression::BindImport(IncomingBindingExpressionBindImport {
            ty: WasmTypeRef::Named(WasmTypeRefNamed { name: "hi".into() }),
            binding: ImportBindingRef::Named(ImportBindingRefNamed { name: "hello".into() }),
            expr: Box::new(IncomingBindingExpression::Get(
                IncomingBindingExpressionGet { idx: 1 }
            )),
        })
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
