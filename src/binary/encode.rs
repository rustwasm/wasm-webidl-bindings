use crate::ast;
use std::io;

pub(crate) trait Encode<W: ?Sized + io::Write> {
    fn encode(&self, w: &mut W) -> io::Result<()>;
}

trait WriteExt: io::Write {
    fn byte(&mut self, b: u8) -> io::Result<()> {
        self.write_all(&[b])?;
        Ok(())
    }

    fn uleb(&mut self, val: u32) -> io::Result<()> {
        leb128::write::unsigned(self, val as u64)?;
        Ok(())
    }

    fn ileb(&mut self, val: i32) -> io::Result<()> {
        leb128::write::signed(self, val as i64)?;
        Ok(())
    }

    fn vec<E>(&mut self, items: &[E]) -> io::Result<()>
    where
        E: Encode<Self>,
    {
        self.uleb(items.len() as u32)?;
        for x in items {
            x.encode(self)?;
        }
        Ok(())
    }
}

impl<W> WriteExt for W where W: ?Sized + io::Write {}

impl<W: ?Sized + io::Write> Encode<W> for String {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        w.uleb(self.len() as u32)?;
        w.write_all(self.as_bytes())
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlBindingsSection {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        self.types.encode(w)?;
        self.bindings.encode(w)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlTypeSubsection {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        w.byte(0)?;
        w.vec(&self.types)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlType {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        self.ty.encode(w)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlCompoundType {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        match self {
            ast::WebidlCompoundType::Function(f) => {
                w.byte(0)?;
                f.encode(w)
            }
            ast::WebidlCompoundType::Dictionary(d) => {
                w.byte(1)?;
                d.encode(w)
            }
            ast::WebidlCompoundType::Enumeration(e) => {
                w.byte(2)?;
                e.encode(w)
            }
            ast::WebidlCompoundType::Union(u) => {
                w.byte(3)?;
                u.encode(w)
            }
        }
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlFunction {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        self.kind.encode(w)?;
        w.vec(&self.params)?;
        if let Some(result) = self.result.as_ref() {
            w.byte(1)?;
            result.encode(w)
        } else {
            w.byte(0)
        }
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlFunctionKind {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        match self {
            ast::WebidlFunctionKind::Static => w.byte(0),
            ast::WebidlFunctionKind::Method(m) => {
                w.byte(1)?;
                m.ty.encode(w)
            }
            ast::WebidlFunctionKind::Constructor => w.byte(2),
        }
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlTypeRef {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        match self {
            ast::WebidlTypeRef::Indexed(i) => w.ileb(i.idx as i32),
            ast::WebidlTypeRef::Scalar(s) => s.encode(w),
            ast::WebidlTypeRef::Named(_) => panic!("can only encode canonicalized ASTs"),
        }
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlScalarType {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        match self {
            ast::WebidlScalarType::Any => w.ileb(-1),
            ast::WebidlScalarType::Boolean => w.ileb(-2),
            ast::WebidlScalarType::Byte => w.ileb(-3),
            ast::WebidlScalarType::Octet => w.ileb(-4),
            ast::WebidlScalarType::Long => w.ileb(-5),
            ast::WebidlScalarType::UnsignedLong => w.ileb(-6),
            ast::WebidlScalarType::Short => w.ileb(-7),
            ast::WebidlScalarType::UnsignedShort => w.ileb(-8),
            ast::WebidlScalarType::LongLong => w.ileb(-9),
            ast::WebidlScalarType::UnsignedLongLong => w.ileb(-10),
            ast::WebidlScalarType::Float => w.ileb(-11),
            ast::WebidlScalarType::UnrestrictedFloat => w.ileb(-12),
            ast::WebidlScalarType::Double => w.ileb(-13),
            ast::WebidlScalarType::UnrestrictedDouble => w.ileb(-14),
            ast::WebidlScalarType::DomString => w.ileb(-15),
            ast::WebidlScalarType::ByteString => w.ileb(-16),
            ast::WebidlScalarType::UsvString => w.ileb(-17),
            ast::WebidlScalarType::Object => w.ileb(-18),
            ast::WebidlScalarType::Symbol => w.ileb(-19),
            ast::WebidlScalarType::ArrayBuffer => w.ileb(-20),
            ast::WebidlScalarType::DataView => w.ileb(-21),
            ast::WebidlScalarType::Int8Array => w.ileb(-22),
            ast::WebidlScalarType::Int16Array => w.ileb(-23),
            ast::WebidlScalarType::Int32Array => w.ileb(-24),
            ast::WebidlScalarType::Uint8Array => w.ileb(-25),
            ast::WebidlScalarType::Uint16Array => w.ileb(-26),
            ast::WebidlScalarType::Uint32Array => w.ileb(-27),
            ast::WebidlScalarType::Uint8ClampedArray => w.ileb(-28),
            ast::WebidlScalarType::Float32Array => w.ileb(-29),
            ast::WebidlScalarType::Float64Array => w.ileb(-30),
        }
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlDictionary {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        w.vec(&self.fields)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlDictionaryField {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        self.name.encode(w)?;
        self.ty.encode(w)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlEnumeration {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        w.vec(&self.values)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlUnion {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        w.vec(&self.members)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WebidlFunctionBindingsSubsection {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        w.byte(1)?;
        w.vec(&self.bindings)?;
        w.vec(&self.binds)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::FunctionBinding {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        match self {
            ast::FunctionBinding::Import(i) => {
                w.byte(0)?;
                i.encode(w)
            }
            ast::FunctionBinding::Export(e) => {
                w.byte(1)?;
                e.encode(w)
            }
        }
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::ImportBinding {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        self.wasm_ty.encode(w)?;
        self.webidl_ty.encode(w)?;
        self.params.encode(w)?;
        self.result.encode(w)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::ExportBinding {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        self.wasm_ty.encode(w)?;
        self.webidl_ty.encode(w)?;
        self.params.encode(w)?;
        self.result.encode(w)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for walrus::ValType {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        match self {
            walrus::ValType::I32 => w.byte(0x7f),
            walrus::ValType::I64 => w.byte(0x7e),
            walrus::ValType::F32 => w.byte(0x7d),
            walrus::ValType::F64 => w.byte(0x7c),
            walrus::ValType::V128 => w.byte(0x7b),
            walrus::ValType::Anyref => w.byte(0x6f),
        }
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WasmFuncTypeRef {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        match self {
            ast::WasmFuncTypeRef::Indexed(i) => w.uleb(i.idx as u32),
            ast::WasmFuncTypeRef::Named(_) => panic!("can only encode canonicalized ASTs"),
        }
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::OutgoingBindingMap {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        w.vec(&self.bindings)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::OutgoingBindingExpression {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        match self {
            ast::OutgoingBindingExpression::As(e) => {
                w.byte(0)?;
                e.ty.encode(w)?;
                w.uleb(e.idx)
            }
            ast::OutgoingBindingExpression::Utf8Str(e) => {
                w.byte(1)?;
                e.ty.encode(w)?;
                w.uleb(e.offset)?;
                w.uleb(e.length)
            }
            ast::OutgoingBindingExpression::Utf8CStr(e) => {
                w.byte(2)?;
                e.ty.encode(w)?;
                w.uleb(e.offset)
            }
            ast::OutgoingBindingExpression::I32ToEnum(e) => {
                w.byte(3)?;
                e.ty.encode(w)?;
                w.uleb(e.idx)
            }
            ast::OutgoingBindingExpression::View(e) => {
                w.byte(4)?;
                e.ty.encode(w)?;
                w.uleb(e.offset)?;
                w.uleb(e.length)
            }
            ast::OutgoingBindingExpression::Copy(e) => {
                w.byte(5)?;
                e.ty.encode(w)?;
                w.uleb(e.offset)?;
                w.uleb(e.length)
            }
            ast::OutgoingBindingExpression::Dict(e) => {
                w.byte(6)?;
                e.ty.encode(w)?;
                w.vec(&e.fields)
            }
            ast::OutgoingBindingExpression::BindExport(e) => {
                w.byte(7)?;
                e.ty.encode(w)?;
                e.binding.encode(w)?;
                w.uleb(e.idx)
            }
        }
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::BindingRef {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        match self {
            ast::BindingRef::Indexed(i) => w.uleb(i.idx),
            ast::BindingRef::Named(_) => panic!("can only encode canonicalized ASTs"),
        }
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::IncomingBindingMap {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        w.vec(&self.bindings)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::IncomingBindingExpression {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        match self {
            ast::IncomingBindingExpression::Get(e) => {
                w.byte(0)?;
                w.uleb(e.idx)
            }
            ast::IncomingBindingExpression::As(e) => {
                w.byte(1)?;
                e.ty.encode(w)?;
                e.expr.encode(w)
            }
            ast::IncomingBindingExpression::AllocUtf8Str(e) => {
                w.byte(2)?;
                e.alloc_func_name.encode(w)?;
                e.expr.encode(w)
            }
            ast::IncomingBindingExpression::AllocCopy(e) => {
                w.byte(3)?;
                e.alloc_func_name.encode(w)?;
                e.expr.encode(w)
            }
            ast::IncomingBindingExpression::EnumToI32(e) => {
                w.byte(4)?;
                e.ty.encode(w)?;
                e.expr.encode(w)
            }
            ast::IncomingBindingExpression::Field(e) => {
                w.byte(5)?;
                w.uleb(e.idx)?;
                e.expr.encode(w)
            }
            ast::IncomingBindingExpression::BindImport(e) => {
                w.byte(6)?;
                e.ty.encode(w)?;
                e.binding.encode(w)?;
                e.expr.encode(w)
            }
        }
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::Bind {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        self.func.encode(w)?;
        self.binding.encode(w)
    }
}

impl<W: ?Sized + io::Write> Encode<W> for ast::WasmFuncRef {
    fn encode(&self, w: &mut W) -> io::Result<()> {
        match self {
            ast::WasmFuncRef::Indexed(i) => w.uleb(i.idx),
            ast::WasmFuncRef::Named(_) => panic!("can only encode canonicalized ASTs"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    macro_rules! assert_encoding {
        (
            $(
                $name:ident($ast:expr, $expected:expr $(,)* );
            )*
        ) => {
            $(
                #[test]
                fn $name() {
                    let mut buf = vec![];
                    $ast.encode(&mut buf).expect("writing into a vec can't fail");
                    assert_eq!(buf, &$expected[..]);
                }
            )*
        }
    }

    // SLEB128 = [185, 10]
    const WEBIDL_TYPE_REF_A: WebidlTypeRef =
        WebidlTypeRef::Indexed(WebidlTypeRefIndexed { idx: 1337 });
    // SLEB128 = [169, 198, 0]
    const WEBIDL_TYPE_REF_B: WebidlTypeRef =
        WebidlTypeRef::Indexed(WebidlTypeRefIndexed { idx: 9001 });

    assert_encoding! {
        webidl_bindings_sec(
            WebidlBindingsSection {
                types: WebidlTypeSubsection {
                    types: vec![
                        WebidlType {
                            name: Some("$TextEncoderEncodeIntoResult".into()),
                            ty: WebidlCompoundType::Dictionary(WebidlDictionary {
                                fields: vec![
                                    WebidlDictionaryField {
                                        name: "read".into(),
                                        ty: WebidlTypeRef::Indexed(WebidlTypeRefIndexed {
                                            idx: 0,
                                        }),
                                    },
                                    WebidlDictionaryField {
                                        name: "written".into(),
                                        ty: WebidlTypeRef::Indexed(WebidlTypeRefIndexed {
                                            idx: 1,
                                        }),
                                    },
                                ],
                            }),
                        },
                        WebidlType {
                            name: Some("$EncodeIntoFuncWebIDL".into()),
                            ty: WebidlCompoundType::Function(WebidlFunction {
                                kind: WebidlFunctionKind::Method(WebidlFunctionKindMethod {
                                    ty: WebidlTypeRef::Indexed(WebidlTypeRefIndexed {
                                        idx: 2,
                                    }),
                                }),
                                params: vec![
                                    WebidlTypeRef::Indexed(WebidlTypeRefIndexed {
                                        idx: 3,
                                    }),
                                    WebidlTypeRef::Indexed(WebidlTypeRefIndexed {
                                        idx: 4,
                                    }),
                                ],
                                result: Some(WebidlTypeRef::Indexed(WebidlTypeRefIndexed {
                                    idx: 5,
                                })),
                            })
                        },
                    ]
                },
                bindings: WebidlFunctionBindingsSubsection {
                    bindings: vec![FunctionBinding::Import(ImportBinding {
                        name: Some("$encodeIntoBinding".into()),
                        wasm_ty: WasmFuncTypeRef::Indexed(WasmFuncTypeRefIndexed {
                            idx: 6,
                        }),
                        webidl_ty: WebidlTypeRef::Indexed(WebidlTypeRefIndexed {
                            idx: 7,
                        }),
                        params: OutgoingBindingMap {
                            bindings: vec![
                                OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                                    ty: WebidlTypeRef::Indexed(WebidlTypeRefIndexed {
                                        idx: 8,
                                    }),
                                    idx: 0
                                }),
                                OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                                    ty: WebidlTypeRef::Indexed(WebidlTypeRefIndexed {
                                        idx: 9,
                                    }),
                                    idx: 1
                                }),
                                OutgoingBindingExpression::View(OutgoingBindingExpressionView {
                                    ty: WebidlTypeRef::Indexed(WebidlTypeRefIndexed {
                                        idx: 10,
                                    }),
                                    offset: 2,
                                    length: 3
                                })
                            ]
                        },
                        result: IncomingBindingMap {
                            bindings: vec![
                                IncomingBindingExpression::As(IncomingBindingExpressionAs {
                                    ty: walrus::ValType::I32,
                                    expr: Box::new(IncomingBindingExpression::Field(
                                        IncomingBindingExpressionField {
                                            idx: 0,
                                            expr: Box::new(IncomingBindingExpression::Get(
                                                IncomingBindingExpressionGet { idx: 0 }
                                            ))
                                        }
                                    ))
                                }),
                                IncomingBindingExpression::As(IncomingBindingExpressionAs {
                                    ty: walrus::ValType::I32,
                                    expr: Box::new(IncomingBindingExpression::Field(
                                        IncomingBindingExpressionField {
                                            idx: 1,
                                            expr: Box::new(IncomingBindingExpression::Get(
                                                IncomingBindingExpressionGet { idx: 0 }
                                            )),
                                        }
                                    ))
                                })
                            ]
                        }
                    })],
                    binds: vec![Bind {
                        func: WasmFuncRef::Indexed(WasmFuncRefIndexed {
                            idx: 13,
                        }),
                        binding: BindingRef::Indexed(BindingRefIndexed {
                            idx: 14,
                        })
                    }]
                },
            },
            vec![
                // types subsection
                0,
                // number of types
                2,
                // dictionary type
                1,
                // number of fields
                2,
                // "read"
                4, 114, 101, 97, 100,
                0,
                // "written"
                7, 119, 114, 105, 116, 116, 101, 110,
                1,
                // function type
                0,
                // method
                1, 2,
                // params
                2, 3, 4,
                // result
                1, 5,
                // bindings subsection
                1,
                // number of bindings
                1,
                // import
                0,
                6,
                7,
                // params
                3,
                // as
                0, 8, 0,
                // as
                0, 9, 1,
                // view
                4, 10, 2, 3,
                // results
                2,
                // as
                1, 0x7f,
                // field
                5, 0,
                // get
                0, 0,
                // as
                1, 0x7f,
                // field
                5, 1,
                // get
                0, 0,
                // number of binds
                1,
                // bind
                13, 14,
            ],
        );

        webidl_type_function(
            WebidlType {
                name: None,
                ty: WebidlCompoundType::Function(WebidlFunction {
                    kind: WebidlFunctionKind::Static,
                    params: vec![],
                    result: None
                }),
            },
            [
                // Function kind
                0,
                // Static kind
                0,
                // Number of params
                0,
                // Has result?
                0,
            ]
        );
        webidl_type_dictionary(
            WebidlType {
                name: None,
                ty: WebidlCompoundType::Dictionary(WebidlDictionary {
                    fields: vec![
                        WebidlDictionaryField { name: "first".into(), ty: WEBIDL_TYPE_REF_A, },
                        WebidlDictionaryField { name: "second".into(), ty: WEBIDL_TYPE_REF_B, },
                    ],
                }),
            },
            [
                // Dictionary type
                1,
                // Number of fields
                2,
                // "first"
                5, 102, 105, 114, 115, 116,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // "second"
                6, 115, 101, 99, 111, 110, 100,
                // WEBIDL_TYPE_REF_B
                169, 198, 0,
            ],
        );
        webidl_type_enumeration(
            WebidlType {
                name: None,
                ty: WebidlCompoundType::Enumeration(WebidlEnumeration {
                    values: vec!["hi".into(), "bye".into()],
                }),
            },
            [
                // Enumeration type
                2,
                // Number of values
                2,
                // "hi"
                2, 104, 105,
                // "bye"
                3, 98, 121, 101,
            ]
        );
        webidl_type_union(
            WebidlType {
                name: None,
                ty: WebidlCompoundType::Union(WebidlUnion {
                    members: vec![WEBIDL_TYPE_REF_A, WEBIDL_TYPE_REF_B]
                }),
            },
            [
                // Union type
                3,
                // Number of members
                2,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // WEBIDL_TYPE_REF_B
                169, 198, 0,
            ]
        );

        webidl_function_static(
            WebidlFunction {
                kind: WebidlFunctionKind::Static,
                params: vec![],
                result: None
            },
            [
                // Static kind
                0,
                // Number of params
                0,
                // Has result?
                0,
            ]
        );
        webidl_function_method(
            WebidlFunction {
                kind: WebidlFunctionKind::Method(WebidlFunctionKindMethod {
                    ty: WEBIDL_TYPE_REF_A,
                }),
                params: vec![],
                result: None
            },
            [
                // Method kind
                1,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // Number of params
                0,
                // Has result?
                0,
            ]
        );
        webidl_function_constructor(
            WebidlFunction {
                kind: WebidlFunctionKind::Constructor,
                params: vec![],
                result: None
            },
            [
                // Constructor kind
                2,
                // Number of params
                0,
                // Has result?
                0,
            ]
        );
        webidl_function_params(
            WebidlFunction {
                kind: WebidlFunctionKind::Static,
                params: vec![WEBIDL_TYPE_REF_A, WEBIDL_TYPE_REF_B],
                result: None
            },
            [
                // Static kind
                0,
                // Number of params
                2,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // WEBIDL_TYPE_REF_B
                169, 198, 0,
                // Has result?
                0,
            ]
        );
        webidl_function_result(
            WebidlFunction {
                kind: WebidlFunctionKind::Static,
                params: vec![],
                result: Some(WEBIDL_TYPE_REF_A),
            },
            [
                // Static kind
                0,
                // Number of params
                0,
                // Has result?
                1,
                // WEBIDL_TYPE_REF_A
                185, 10,
            ]
        );

        webidl_dictionary(
            WebidlDictionary {
                fields: vec![
                    WebidlDictionaryField { name: "first".into(), ty: WEBIDL_TYPE_REF_A, },
                    WebidlDictionaryField { name: "second".into(), ty: WEBIDL_TYPE_REF_B, },
                ],
            },
            [
                // Number of fields
                2,
                // "first"
                5, 102, 105, 114, 115, 116,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // "second"
                6, 115, 101, 99, 111, 110, 100,
                // WEBIDL_TYPE_REF_B
                169, 198, 0,
            ],
        );

        webidl_enumeration(
            WebidlEnumeration {
                values: vec!["a".into(), "b".into(), "c".into()],
            },
            [
                // Number of values
                3,
                // "a"
                1, 97,
                // "b"
                1, 98,
                // "c"
                1, 99,
            ],
        );

        webidl_union(
            WebidlUnion {
                members: vec![WEBIDL_TYPE_REF_A, WEBIDL_TYPE_REF_B],
            },
            [
                // Number of members
                2,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // WEBIDL_TYPE_REF_B
                169, 198, 0,
            ],
        );

        bindings_subsec(
            WebidlFunctionBindingsSubsection {
                bindings: vec![],
                binds: vec![],
            },
            [
                // function bindings subsection
                1,
                // 0 bindings
                0,
                // 0 binds
                0,
            ],
        );

        function_binding_export(
            FunctionBinding::Export(ExportBinding {
                name: None,
                wasm_ty: WasmFuncTypeRef::Indexed(WasmFuncTypeRefIndexed { idx: 0 }),
                webidl_ty: WEBIDL_TYPE_REF_A,
                params: IncomingBindingMap { bindings: vec![] },
                result: OutgoingBindingMap { bindings: vec![] },
            }),
            [
                // export binding
                1,
                // wasm_ty
                0,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // 0 params
                0,
                // 0 results
                0,
            ],
        );

        function_binding_import(
            FunctionBinding::Import(ImportBinding {
                name: None,
                wasm_ty: WasmFuncTypeRef::Indexed(WasmFuncTypeRefIndexed { idx: 0 }),
                webidl_ty: WEBIDL_TYPE_REF_A,
                params: OutgoingBindingMap { bindings: vec![] },
                result: IncomingBindingMap { bindings: vec![] },
            }),
            [
                // import binding
                0,
                // wasm_ty
                0,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // 0 params
                0,
                // 0 results
                0,
            ],
        );

        outgoing_binding_map(
            OutgoingBindingMap {
                bindings: vec![
                    OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                        ty: WEBIDL_TYPE_REF_A,
                        idx: 1,
                    }),
                    OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                        ty: WEBIDL_TYPE_REF_B,
                        idx: 2,
                    }),
                ]
            },
            [
                // Number of expresssions
                2,
                // as
                0,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // idx
                1,
                // as
                0,
                // WEBIDL_TYPE_REF_B
                169, 198, 0,
                // idx
                2,
            ],
        );

        outgoing_binding_expression_as(
            OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                ty: WEBIDL_TYPE_REF_A,
                idx: 2,
            }),
            [
                // as
                0,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // idx
                2,
            ],
        );
        outgoing_binding_expression_utf8_str(
            OutgoingBindingExpression::Utf8Str(OutgoingBindingExpressionUtf8Str {
                ty: WEBIDL_TYPE_REF_A,
                offset: 22,
                length: 33,
            }),
            [
                // utf8-str
                1,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // offset
                22,
                // length
                33,
            ],
        );
        outgoing_binding_expression_utf8_cstr(
            OutgoingBindingExpression::Utf8CStr(OutgoingBindingExpressionUtf8CStr {
                ty: WEBIDL_TYPE_REF_A,
                offset: 22,
            }),
            [
                // utf8-cstr
                2,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // offset
                22,
            ],
        );
        outgoing_binding_expression_i32_to_enum(
            OutgoingBindingExpression::I32ToEnum(OutgoingBindingExpressionI32ToEnum {
                ty: WEBIDL_TYPE_REF_A,
                idx: 4,
            }),
            [
                // i32-to-enum
                3,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // idx
                4,
            ],
        );
        outgoing_binding_expression_view(
            OutgoingBindingExpression::View(OutgoingBindingExpressionView {
                ty: WEBIDL_TYPE_REF_A,
                offset: 1,
                length: 2,
            }),
            [
                // view
                4,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // offset
                1,
                // length
                2,
            ],
        );
        outgoing_binding_expression_copy(
            OutgoingBindingExpression::Copy(OutgoingBindingExpressionCopy {
                ty: WEBIDL_TYPE_REF_A,
                offset: 1,
                length: 2,
            }),
            [
                // copy
                5,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // offset
                1,
                // length
                2,
            ],
        );
        outgoing_binding_expression_dict(
            OutgoingBindingExpression::Dict(OutgoingBindingExpressionDict {
                ty: WEBIDL_TYPE_REF_A,
                fields: vec![
                    OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                        ty: WEBIDL_TYPE_REF_B,
                        idx: 2,
                    }),
                ],
            }),
            [
                // dict
                6,
                // WEBIDL_TYPE_REF_A
                185, 10,
                // Number of fields
                1,
                // as
                0,
                // WEBIDL_TYPE_REF_B
                169, 198, 0,
                // idx
                2
            ],
        );
        outgoing_binding_expression_bind_export(
            OutgoingBindingExpression::BindExport(OutgoingBindingExpressionBindExport {
                ty: WebidlTypeRef::Indexed(WebidlTypeRefIndexed { idx: 6 }),
                binding: BindingRef::Indexed(BindingRefIndexed { idx: 1 }),
                idx: 3,
            }),
            [
                // bind-export
                7,
                6,
                1,
                3,
            ],
        );

        incoming_binding_map(
            IncomingBindingMap {
                bindings: vec![
                    IncomingBindingExpression::Get(IncomingBindingExpressionGet { idx: 1 }),
                    IncomingBindingExpression::Get(IncomingBindingExpressionGet { idx: 2 }),
                    IncomingBindingExpression::Get(IncomingBindingExpressionGet { idx: 3 }),
                ],
            },
            [
                // Number of expressions
                3,
                // get
                0, 1,
                // get
                0, 2,
                // get
                0, 3
            ],
        );

        incoming_binding_expression_get(
            IncomingBindingExpression::Get(IncomingBindingExpressionGet { idx: 1 }),
            [
                // get
                0,
                1,
            ]
        );
        incoming_binding_expression_as(
            IncomingBindingExpression::As(IncomingBindingExpressionAs {
                ty: walrus::ValType::I32,
                expr: Box::new(IncomingBindingExpression::Get(IncomingBindingExpressionGet {
                    idx: 2,
                })),
            }),
            [
                // as
                1, 0x7f,
                // get
                0, 2,
            ],
        );
        incoming_binding_expression_alloc_utf8_str(
            IncomingBindingExpression::AllocUtf8Str(IncomingBindingExpressionAllocUtf8Str {
                alloc_func_name: "malloc".into(),
                expr: Box::new(IncomingBindingExpression::Get(IncomingBindingExpressionGet {
                    idx: 1,
                })),
            }),
            [
                // alloc-utf8-str
                2,
                // "malloc"
                6, 109, 97, 108, 108, 111, 99,
                // get
                0,
                1
            ],
        );
        incoming_binding_expression_alloc_copy(
            IncomingBindingExpression::AllocCopy(IncomingBindingExpressionAllocCopy {
                alloc_func_name: "malloc".into(),
                expr: Box::new(IncomingBindingExpression::Get(IncomingBindingExpressionGet {
                    idx: 1,
                })),
            }),
            [
                // alloc-copy
                3,
                // "malloc"
                6, 109, 97, 108, 108, 111, 99,
                // get
                0,
                1
            ],
        );
        incoming_binding_expression_enum_to_i32(
            IncomingBindingExpression::EnumToI32(IncomingBindingExpressionEnumToI32 {
                ty: WebidlTypeRef::Indexed(WebidlTypeRefIndexed { idx: 1 }),
                expr: Box::new(IncomingBindingExpression::Get(IncomingBindingExpressionGet {
                    idx: 2,
                })),
            }),
            [
                // enum-to-i32
                4,
                1,
                // get
                0,
                2,
            ],
        );
        incoming_binding_expression_field(
            IncomingBindingExpression::Field(IncomingBindingExpressionField {
                idx: 1,
                expr: Box::new(IncomingBindingExpression::Get(IncomingBindingExpressionGet {
                    idx: 2,
                })),
            }),
            [
                // field
                5,
                1,
                // get
                0,
                2,
            ],
        );
        incoming_binding_expression_bind_import(
            IncomingBindingExpression::BindImport(IncomingBindingExpressionBindImport {
                ty: WasmFuncTypeRef::Indexed(WasmFuncTypeRefIndexed { idx: 1 }),
                binding: BindingRef::Indexed(BindingRefIndexed { idx: 2 }),
                expr: Box::new(IncomingBindingExpression::Get(IncomingBindingExpressionGet {
                    idx: 3,
                })),
            }),
            [
                // bind-import
                6,
                1,
                2,
                // get
                0,
                3
            ],
        );

        bind(
            Bind {
                func: WasmFuncRef::Indexed(WasmFuncRefIndexed { idx: 1 }),
                binding: BindingRef::Indexed(BindingRefIndexed { idx: 2 }),
            },
            [1, 2],
        );

        webidl_type_ref(WEBIDL_TYPE_REF_A, [185, 10]);
    }
}
