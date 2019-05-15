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
        webidl_type_func_ok_1,
        WebidlTypeParser,
        "type $AddContactFuncWebIDL (func (method any) (param $Contact DOMString) (result bool))",
        WebidlType {
            name: Some("$AddContactFuncWebIDL".into()),
            ty: WebidlCompoundType::Function(WebidlFunction {
                kind: WebidlFunctionKind::Method(WebidlFunctionKindMethod {
                    ty: WebidlTypeRef::Named(WebidlTypeRefNamed { name: "any".into() })
                }),
                params: vec![
                    WebidlTypeRef::Named(WebidlTypeRefNamed {
                        name: "$Contact".into()
                    }),
                    WebidlTypeRef::Named(WebidlTypeRefNamed {
                        name: "DOMString".into()
                    }),
                ],
                result: Some(WebidlTypeRef::Named(WebidlTypeRefNamed {
                    name: "bool".into()
                })),
            })
        }
    );
    ok!(
        webidl_type_func_ok_2,
        WebidlTypeParser,
        "type $AddContactFuncWebIDL (func (method any) (param $Contact DOMString))",
        WebidlType {
            name: Some("$AddContactFuncWebIDL".into()),
            ty: WebidlCompoundType::Function(WebidlFunction {
                kind: WebidlFunctionKind::Method(WebidlFunctionKindMethod {
                    ty: WebidlTypeRef::Named(WebidlTypeRefNamed { name: "any".into() })
                }),
                params: vec![
                    WebidlTypeRef::Named(WebidlTypeRefNamed {
                        name: "$Contact".into()
                    }),
                    WebidlTypeRef::Named(WebidlTypeRefNamed {
                        name: "DOMString".into()
                    }),
                ],
                result: None,
            })
        }
    );
    ok!(
        webidl_type_func_ok_3,
        WebidlTypeParser,
        "type $AddContactFuncWebIDL (func (param DOMString))",
        WebidlType {
            name: Some("$AddContactFuncWebIDL".into()),
            ty: WebidlCompoundType::Function(WebidlFunction {
                kind: WebidlFunctionKind::Static,
                params: vec![WebidlTypeRef::Named(WebidlTypeRefNamed {
                    name: "DOMString".into()
                })],
                result: None,
            })
        }
    );
    ok!(
        webidl_type_func_ok_4,
        WebidlTypeParser,
        "type $AddContactFuncWebIDL (func (param))",
        WebidlType {
            name: Some("$AddContactFuncWebIDL".into()),
            ty: WebidlCompoundType::Function(WebidlFunction {
                kind: WebidlFunctionKind::Static,
                params: vec![],
                result: None,
            })
        }
    );
    ok!(
        webidl_type_func_ok_5,
        WebidlTypeParser,
        "type $AddContactFuncWebIDL (func)",
        WebidlType {
            name: Some("$AddContactFuncWebIDL".into()),
            ty: WebidlCompoundType::Function(WebidlFunction {
                kind: WebidlFunctionKind::Static,
                params: vec![],
                result: None,
            })
        }
    );
    ok!(
        webidl_type_func_ok_6,
        WebidlTypeParser,
        "type (func)",
        WebidlType {
            name: None,
            ty: WebidlCompoundType::Function(WebidlFunction {
                kind: WebidlFunctionKind::Static,
                params: vec![],
                result: None,
            })
        }
    );
    ok!(
        webidl_type_func_ok_7,
        WebidlTypeParser,
        "type MyCtor (func (constructor default-new-target) (result any))",
        WebidlType {
            name: Some("MyCtor".into()),
            ty: WebidlCompoundType::Function(WebidlFunction {
                kind: WebidlFunctionKind::Constructor,
                params: vec![],
                result: Some(WebidlTypeRef::Named(WebidlTypeRefNamed {
                    name: "any".into()
                })),
            })
        }
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
        WebidlType {
            name: Some("$Contact".into()),
            ty: WebidlCompoundType::Dictionary(WebidlDictionary {
                fields: vec![
                    WebidlDictionaryField {
                        name: "name".into(),
                        ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                            name: "DOMString".into()
                        }),
                    },
                    WebidlDictionaryField {
                        name: "age".into(),
                        ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                            name: "long".into()
                        }),
                    },
                ],
            }),
        }
    );
    ok!(
        webidl_type_dict_ok_2,
        WebidlTypeParser,
        r#"type $Contact (dict)"#,
        WebidlType {
            name: Some("$Contact".into()),
            ty: WebidlCompoundType::Dictionary(WebidlDictionary { fields: vec![] }),
        }
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
        WebidlType {
            name: Some("Blah".into()),
            ty: WebidlCompoundType::Enumeration(WebidlEnumeration {
                values: vec!["uno".into(), "dos".into(), "tres".into()],
            }),
        }
    );
    ok!(
        webidl_type_enum_ok_2,
        WebidlTypeParser,
        r#"type (enum)"#,
        WebidlType {
            name: None,
            ty: WebidlCompoundType::Enumeration(WebidlEnumeration { values: vec![] }),
        }
    );
    err!(
        webidl_type_enum_err_1,
        WebidlTypeParser,
        r#"type (enum 1 2 3)"#
    );

    ok!(
        webidl_type_union_ok_1,
        WebidlTypeParser,
        "type MyUnion (union long bool)",
        WebidlType {
            name: Some("MyUnion".into()),
            ty: WebidlCompoundType::Union(WebidlUnion {
                members: vec![
                    WebidlTypeRef::Named(WebidlTypeRefNamed {
                        name: "long".into(),
                    }),
                    WebidlTypeRef::Named(WebidlTypeRefNamed {
                        name: "bool".into(),
                    }),
                ],
            })
        }
    );
    ok!(
        webidl_type_union_ok_2,
        WebidlTypeParser,
        "type (union)",
        WebidlType {
            name: None,
            ty: WebidlCompoundType::Union(WebidlUnion { members: vec![] })
        }
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
        ImportBinding {
            name: Some("Yoyo".into()),
            wasm_ty: WasmTypeRef::Named(WasmTypeRefNamed { name: "MyWasmFunc".into() }),
            webidl_ty: WebidlTypeRef::Named(WebidlTypeRefNamed { name: "MyWebidlFunc".into() }),
            params: OutgoingBindingMap {
                bindings: vec![OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                    ty: WebidlTypeRef::Named(WebidlTypeRefNamed { name: "any".into() }),
                    idx: 0,
                })],
            },
            result: IncomingBindingMap {
                bindings: vec![IncomingBindingExpression::As(IncomingBindingExpressionAs {
                    ty: WasmTypeRef::Named(WasmTypeRefNamed { name: "i32".into() }),
                    expr: Box::new(IncomingBindingExpression::Get(IncomingBindingExpressionGet { idx: 0 })),
                })],
            },
        }
    );
    ok!(
        import_binding_ok_2,
        ImportBindingParser,
        "func-binding import MyWasmFunc MyWebidlFunc (param) (result)",
        ImportBinding {
            name: None,
            wasm_ty: WasmTypeRef::Named(WasmTypeRefNamed {
                name: "MyWasmFunc".into()
            }),
            webidl_ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                name: "MyWebidlFunc".into()
            }),
            params: OutgoingBindingMap { bindings: vec![] },
            result: IncomingBindingMap { bindings: vec![] },
        }
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
        "func-binding Yoyo export MyWasmFunc MyWebidlFunc (param (as i32 (get 0))) (result (as any 0))",
        ExportBinding {
            name: Some("Yoyo".into()),
            wasm_ty: WasmTypeRef::Named(WasmTypeRefNamed { name: "MyWasmFunc".into() }),
            webidl_ty: WebidlTypeRef::Named(WebidlTypeRefNamed { name: "MyWebidlFunc".into() }),
            params: IncomingBindingMap {
                bindings: vec![IncomingBindingExpression::As(IncomingBindingExpressionAs {
                    ty: WasmTypeRef::Named(WasmTypeRefNamed { name: "i32".into() }),
                    expr: Box::new(IncomingBindingExpression::Get(IncomingBindingExpressionGet { idx: 0 })),
                })],
            },
            result: OutgoingBindingMap {
                bindings: vec![OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                    ty: WebidlTypeRef::Named(WebidlTypeRefNamed { name: "any".into() }),
                    idx: 0,
                })],
            },
        }
    );
    ok!(
        export_binding_ok_2,
        ExportBindingParser,
        "func-binding export MyWasmFunc MyWebidlFunc (param) (result)",
        ExportBinding {
            name: None,
            wasm_ty: WasmTypeRef::Named(WasmTypeRefNamed {
                name: "MyWasmFunc".into()
            }),
            webidl_ty: WebidlTypeRef::Named(WebidlTypeRefNamed {
                name: "MyWebidlFunc".into()
            }),
            params: IncomingBindingMap { bindings: vec![] },
            result: OutgoingBindingMap { bindings: vec![] },
        }
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
            binding: ImportBindingRef::Named(ImportBindingRefNamed {
                name: "hello".into()
            }),
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
