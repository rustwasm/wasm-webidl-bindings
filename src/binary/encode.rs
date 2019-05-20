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
        unimplemented!()
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
            // TODO: primitive types:
            // https://github.com/rustwasm/wasm-webidl-bindings/issues/8
            ast::WebidlTypeRef::Named(_) => panic!("can only encode canonicalized ASTs"),
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
                    assert_eq!(buf, &$expected);
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

        webidl_type_ref(WEBIDL_TYPE_REF_A, [185, 10]);
    }
}
