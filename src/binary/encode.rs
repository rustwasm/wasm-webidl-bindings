use crate::ast::*;
use id_arena::Id;
use std::collections::HashMap;
use std::io;

pub(crate) struct EncodeContext<'a> {
    indices: &'a walrus::IdsToIndices,
    webidl_type_id_to_idx: HashMap<Id<WebidlCompoundType>, u32>,
    binding_id_to_idx: HashMap<Id<FunctionBinding>, u32>,
}

// Factor this out into a trait to make testing easier.
pub(crate) trait Indices {
    fn assign_webidl_type_index(&mut self, id: Id<WebidlCompoundType>);
    fn webidl_type_index(&self, id: Id<WebidlCompoundType>) -> u32;
    fn assign_binding_index(&mut self, id: Id<FunctionBinding>);
    fn binding_index(&self, id: Id<FunctionBinding>) -> u32;
    fn wasm_func_index(&self, id: walrus::FunctionId) -> u32;
    fn wasm_func_type_index(&self, id: walrus::TypeId) -> u32;
}

impl Indices for EncodeContext<'_> {
    fn assign_webidl_type_index(&mut self, id: Id<WebidlCompoundType>) {
        let idx = self.webidl_type_id_to_idx.len() as u32;
        let old_idx = self.webidl_type_id_to_idx.insert(id, idx);
        assert!(old_idx.is_none());
    }

    fn webidl_type_index(&self, id: Id<WebidlCompoundType>) -> u32 {
        self.webidl_type_id_to_idx[&id]
    }

    fn assign_binding_index(&mut self, id: Id<FunctionBinding>) {
        let idx = self.binding_id_to_idx.len() as u32;
        let old_idx = self.binding_id_to_idx.insert(id, idx);
        assert!(old_idx.is_none());
    }

    fn binding_index(&self, id: Id<FunctionBinding>) -> u32 {
        self.binding_id_to_idx[&id]
    }

    fn wasm_func_index(&self, id: walrus::FunctionId) -> u32 {
        self.indices.get_func_index(id)
    }

    fn wasm_func_type_index(&self, id: walrus::TypeId) -> u32 {
        self.indices.get_type_index(id)
    }
}

impl EncodeContext<'_> {
    pub fn new(indices: &walrus::IdsToIndices) -> EncodeContext {
        EncodeContext {
            indices,
            webidl_type_id_to_idx: Default::default(),
            binding_id_to_idx: Default::default(),
        }
    }
}

pub(crate) trait Encode {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write;
}

impl<'a, T> Encode for &'a T
where
    T: Encode,
{
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        (**self).encode(cx, w)
    }
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

    fn vec<Cx, I, E>(&mut self, cx: &mut Cx, items: I) -> io::Result<()>
    where
        Cx: Indices,
        I: IntoIterator<Item = E>,
        I::IntoIter: ExactSizeIterator,
        E: Encode,
    {
        let items = items.into_iter();
        self.uleb(items.len() as u32)?;
        for x in items {
            x.encode(cx, self)?;
        }
        Ok(())
    }
}

impl<W> WriteExt for W where W: ?Sized + io::Write {}

impl Encode for String {
    fn encode<Cx, W>(&self, _cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        w.uleb(self.len() as u32)?;
        w.write_all(self.as_bytes())
    }
}

impl Encode for WebidlBindings {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        // Web IDL Type Subsection.
        self.types.encode(cx, w)?;

        // Web IDL Function Binding Subsection.
        w.byte(1)?;

        // Bindings.
        //
        // First assign them all indices.
        for (id, _) in self.bindings.arena.iter() {
            cx.assign_binding_index(id);
        }
        // Then actually encode them.
        w.vec(cx, self.bindings.arena.iter().map(|(_, binding)| binding))?;

        // Binds.
        w.vec(cx, self.binds.arena.iter().map(|(_id, b)| b))
    }
}

impl Encode for WebidlTypes {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        w.byte(0)?;
        for (id, _) in self.arena.iter() {
            cx.assign_webidl_type_index(id);
        }
        w.vec(cx, self.arena.iter().map(|(_, ty)| ty))
    }
}

impl Encode for WebidlType {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        self.ty.encode(cx, w)
    }
}

impl Encode for WebidlCompoundType {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        match self {
            WebidlCompoundType::Function(f) => {
                w.byte(0)?;
                f.encode(cx, w)
            }
            WebidlCompoundType::Dictionary(d) => {
                w.byte(1)?;
                d.encode(cx, w)
            }
            WebidlCompoundType::Enumeration(e) => {
                w.byte(2)?;
                e.encode(cx, w)
            }
            WebidlCompoundType::Union(u) => {
                w.byte(3)?;
                u.encode(cx, w)
            }
        }
    }
}

impl Encode for WebidlFunction {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        self.kind.encode(cx, w)?;
        w.vec(cx, &self.params)?;
        if let Some(result) = self.result.as_ref() {
            w.byte(1)?;
            result.encode(cx, w)
        } else {
            w.byte(0)
        }
    }
}

impl Encode for WebidlFunctionKind {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        match self {
            WebidlFunctionKind::Static => w.byte(0),
            WebidlFunctionKind::Method(m) => {
                w.byte(1)?;
                m.ty.encode(cx, w)
            }
            WebidlFunctionKind::Constructor => w.byte(2),
        }
    }
}

impl Encode for WebidlTypeRef {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        match self {
            WebidlTypeRef::Id(id) => w.ileb(cx.webidl_type_index(*id) as i32),
            WebidlTypeRef::Scalar(s) => s.encode(cx, w),
        }
    }
}

impl Encode for WebidlScalarType {
    fn encode<Cx, W>(&self, _cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        match self {
            WebidlScalarType::Any => w.ileb(-1),
            WebidlScalarType::Boolean => w.ileb(-2),
            WebidlScalarType::Byte => w.ileb(-3),
            WebidlScalarType::Octet => w.ileb(-4),
            WebidlScalarType::Long => w.ileb(-5),
            WebidlScalarType::UnsignedLong => w.ileb(-6),
            WebidlScalarType::Short => w.ileb(-7),
            WebidlScalarType::UnsignedShort => w.ileb(-8),
            WebidlScalarType::LongLong => w.ileb(-9),
            WebidlScalarType::UnsignedLongLong => w.ileb(-10),
            WebidlScalarType::Float => w.ileb(-11),
            WebidlScalarType::UnrestrictedFloat => w.ileb(-12),
            WebidlScalarType::Double => w.ileb(-13),
            WebidlScalarType::UnrestrictedDouble => w.ileb(-14),
            WebidlScalarType::DomString => w.ileb(-15),
            WebidlScalarType::ByteString => w.ileb(-16),
            WebidlScalarType::UsvString => w.ileb(-17),
            WebidlScalarType::Object => w.ileb(-18),
            WebidlScalarType::Symbol => w.ileb(-19),
            WebidlScalarType::ArrayBuffer => w.ileb(-20),
            WebidlScalarType::DataView => w.ileb(-21),
            WebidlScalarType::Int8Array => w.ileb(-22),
            WebidlScalarType::Int16Array => w.ileb(-23),
            WebidlScalarType::Int32Array => w.ileb(-24),
            WebidlScalarType::Uint8Array => w.ileb(-25),
            WebidlScalarType::Uint16Array => w.ileb(-26),
            WebidlScalarType::Uint32Array => w.ileb(-27),
            WebidlScalarType::Uint8ClampedArray => w.ileb(-28),
            WebidlScalarType::Float32Array => w.ileb(-29),
            WebidlScalarType::Float64Array => w.ileb(-30),
        }
    }
}

impl Encode for WebidlDictionary {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        w.vec(cx, &self.fields)
    }
}

impl Encode for WebidlDictionaryField {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        self.name.encode(cx, w)?;
        self.ty.encode(cx, w)
    }
}

impl Encode for WebidlEnumeration {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        w.vec(cx, &self.values)
    }
}

impl Encode for WebidlUnion {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        w.vec(cx, &self.members)
    }
}

impl Encode for FunctionBinding {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        match self {
            FunctionBinding::Import(i) => {
                w.byte(0)?;
                i.encode(cx, w)
            }
            FunctionBinding::Export(e) => {
                w.byte(1)?;
                e.encode(cx, w)
            }
        }
    }
}

impl Encode for ImportBinding {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        self.wasm_ty.encode(cx, w)?;
        self.webidl_ty.encode(cx, w)?;
        self.params.encode(cx, w)?;
        self.result.encode(cx, w)
    }
}

impl Encode for ExportBinding {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        self.wasm_ty.encode(cx, w)?;
        self.webidl_ty.encode(cx, w)?;
        self.params.encode(cx, w)?;
        self.result.encode(cx, w)
    }
}

impl Encode for walrus::ValType {
    fn encode<Cx, W>(&self, _cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
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

impl Encode for walrus::TypeId {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        w.uleb(cx.wasm_func_type_index(*self))
    }
}

impl Encode for OutgoingBindingMap {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        w.vec(cx, &self.bindings)
    }
}

impl Encode for OutgoingBindingExpression {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        match self {
            OutgoingBindingExpression::As(e) => {
                w.byte(0)?;
                e.ty.encode(cx, w)?;
                w.uleb(e.idx)
            }
            OutgoingBindingExpression::Utf8Str(e) => {
                w.byte(1)?;
                e.ty.encode(cx, w)?;
                w.uleb(e.offset)?;
                w.uleb(e.length)
            }
            OutgoingBindingExpression::Utf8CStr(e) => {
                w.byte(2)?;
                e.ty.encode(cx, w)?;
                w.uleb(e.offset)
            }
            OutgoingBindingExpression::I32ToEnum(e) => {
                w.byte(3)?;
                e.ty.encode(cx, w)?;
                w.uleb(e.idx)
            }
            OutgoingBindingExpression::View(e) => {
                w.byte(4)?;
                e.ty.encode(cx, w)?;
                w.uleb(e.offset)?;
                w.uleb(e.length)
            }
            OutgoingBindingExpression::Copy(e) => {
                w.byte(5)?;
                e.ty.encode(cx, w)?;
                w.uleb(e.offset)?;
                w.uleb(e.length)
            }
            OutgoingBindingExpression::Dict(e) => {
                w.byte(6)?;
                e.ty.encode(cx, w)?;
                w.vec(cx, &e.fields)
            }
            OutgoingBindingExpression::BindExport(e) => {
                w.byte(7)?;
                e.ty.encode(cx, w)?;
                e.binding.encode(cx, w)?;
                w.uleb(e.idx)
            }
        }
    }
}

impl Encode for Id<FunctionBinding> {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        w.uleb(cx.binding_index(*self))
    }
}

impl Encode for IncomingBindingMap {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        w.vec(cx, &self.bindings)
    }
}

impl Encode for IncomingBindingExpression {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        match self {
            IncomingBindingExpression::Get(e) => {
                w.byte(0)?;
                w.uleb(e.idx)
            }
            IncomingBindingExpression::As(e) => {
                w.byte(1)?;
                e.ty.encode(cx, w)?;
                e.expr.encode(cx, w)
            }
            IncomingBindingExpression::AllocUtf8Str(e) => {
                w.byte(2)?;
                e.alloc_func_name.encode(cx, w)?;
                e.expr.encode(cx, w)
            }
            IncomingBindingExpression::AllocCopy(e) => {
                w.byte(3)?;
                e.alloc_func_name.encode(cx, w)?;
                e.expr.encode(cx, w)
            }
            IncomingBindingExpression::EnumToI32(e) => {
                w.byte(4)?;
                e.ty.encode(cx, w)?;
                e.expr.encode(cx, w)
            }
            IncomingBindingExpression::Field(e) => {
                w.byte(5)?;
                w.uleb(e.idx)?;
                e.expr.encode(cx, w)
            }
            IncomingBindingExpression::BindImport(e) => {
                w.byte(6)?;
                e.ty.encode(cx, w)?;
                e.binding.encode(cx, w)?;
                e.expr.encode(cx, w)
            }
        }
    }
}

impl Encode for Bind {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        self.func.encode(cx, w)?;
        self.binding.encode(cx, w)
    }
}

impl Encode for walrus::FunctionId {
    fn encode<Cx, W>(&self, cx: &mut Cx, w: &mut W) -> io::Result<()>
    where
        Cx: Indices,
        W: ?Sized + io::Write,
    {
        w.uleb(cx.wasm_func_index(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestIndices;

    impl Indices for TestIndices {
        fn assign_webidl_type_index(&mut self, _: Id<WebidlCompoundType>) {}

        fn webidl_type_index(&self, _: Id<WebidlCompoundType>) -> u32 {
            11
        }

        fn assign_binding_index(&mut self, _: Id<FunctionBinding>) {}

        fn binding_index(&self, _: Id<FunctionBinding>) -> u32 {
            22
        }

        fn wasm_func_index(&self, _: walrus::FunctionId) -> u32 {
            33
        }

        fn wasm_func_type_index(&self, _: walrus::TypeId) -> u32 {
            44
        }
    }

    // fn get_func_index(&self, id: walrus::FunctionId) -> u32;
    // fn get_func_type_index(&self, id: walrus::TypeId) -> u32;
    fn do_assert_encoding<E>(ast: E, expected: &[u8])
    where
        E: Encode,
    {
        let mut actual = vec![];
        ast.encode(&mut TestIndices, &mut actual)
            .expect("writing to a vec can't fail");
        assert_eq!(expected, &actual[..]);
    }

    fn get_webidl_type_ref(b: &mut WebidlBindings) -> WebidlTypeRef {
        let id: WebidlUnionId = b.types.insert(WebidlUnion { members: vec![] });
        let id: Id<WebidlCompoundType> = id.into();
        id.into()
    }

    fn get_wasm_func_ref(m: &mut walrus::Module) -> walrus::FunctionId {
        for f in m.funcs.iter() {
            return f.id().into();
        }

        let ty = m.types.add(&[], &[]);
        let id = walrus::FunctionBuilder::new().finish(ty, vec![], vec![], m);
        id.into()
    }

    fn get_wasm_func_type_ref(m: &mut walrus::Module) -> walrus::TypeId {
        m.types.add(&[], &[])
    }

    fn get_binding_ref(b: &mut WebidlBindings, m: &mut walrus::Module) -> Id<FunctionBinding> {
        let wasm_ty = get_wasm_func_type_ref(m);
        let webidl_ty = get_webidl_type_ref(b);
        let id: ImportBindingId = b.bindings.insert(ImportBinding {
            wasm_ty,
            webidl_ty,
            params: OutgoingBindingMap { bindings: vec![] },
            result: IncomingBindingMap { bindings: vec![] },
        });
        let id: Id<FunctionBinding> = id.into();
        id.into()
    }

    macro_rules! assert_encoding {
        (
            $(
                $name:ident(
                    |$bindings:ident, $module:ident| $ast:expr,
                    $expected:expr $(,)*
                );
            )*
        ) => {
            $(
                #[test]
                #[allow(unused_variables)]
                fn $name() {
                    let $bindings = &mut WebidlBindings::default();
                    let $module = &mut walrus::Module::default();
                    let ast = $ast;
                    do_assert_encoding(ast, &$expected);
                }
            )*
        }
    }

    assert_encoding! {
        webidl_bindings_sec(
            |b, m| {
                let encode_into_result = b.types.insert(WebidlDictionary {
                    fields: vec![
                        WebidlDictionaryField {
                            name: "read".into(),
                            ty: WebidlScalarType::UnsignedLongLong.into(),
                        },
                        WebidlDictionaryField {
                            name: "written".into(),
                            ty: WebidlScalarType::UnsignedLongLong.into(),
                        },
                    ],
                });

                let encode_into = b.types.insert(WebidlFunction {
                    kind: WebidlFunctionKind::Method(WebidlFunctionKindMethod {
                        ty: WebidlScalarType::Any.into(),
                    }),
                    params: vec![
                        WebidlScalarType::UsvString.into(),
                        WebidlScalarType::Uint8Array.into(),
                    ],
                    result: Some(encode_into_result.into()),
                });

                let binding = b.bindings.insert(ImportBinding {
                    wasm_ty: get_wasm_func_type_ref(m),
                    webidl_ty: encode_into.into(),
                    params: OutgoingBindingMap {
                        bindings: vec![
                            OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                                ty: WebidlScalarType::Any.into(),
                                idx: 0
                            }),
                            OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                                ty: WebidlScalarType::Any.into(),
                                idx: 1
                            }),
                            OutgoingBindingExpression::View(OutgoingBindingExpressionView {
                                ty: WebidlScalarType::Uint8Array.into(),
                                offset: 2,
                                length: 3
                            }),
                        ],
                    },
                    result: IncomingBindingMap {
                        bindings: vec![
                            IncomingBindingExpression::As(IncomingBindingExpressionAs {
                                ty: walrus::ValType::I64,
                                expr: Box::new(IncomingBindingExpression::Field(
                                    IncomingBindingExpressionField {
                                        idx: 0,
                                        expr: Box::new(IncomingBindingExpression::Get(
                                            IncomingBindingExpressionGet { idx: 0 },
                                        ))
                                    }
                                ))
                            }),
                            IncomingBindingExpression::As(IncomingBindingExpressionAs {
                                ty: walrus::ValType::I64,
                                expr: Box::new(IncomingBindingExpression::Field(
                                    IncomingBindingExpressionField {
                                        idx: 1,
                                        expr: Box::new(IncomingBindingExpression::Get(
                                            IncomingBindingExpressionGet { idx: 0 },
                                        )),
                                    }
                                ))
                            }),
                        ],
                    }
                });

                let func = get_wasm_func_ref(m);
                let binding: Id<FunctionBinding> = binding.into();
                b.binds.insert(Bind {
                    func,
                    binding,
                });

                &*b
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
                118,
                // "written"
                7, 119, 114, 105, 116, 116, 101, 110,
                118,
                // function type
                0,
                // method
                1, 127,
                // params
                2, 111, 103,
                // result
                1, 11,
                // bindings subsection
                1,
                // number of bindings
                1,
                // import
                0,
                44,
                11,
                // params
                3,
                // as
                0, 127, 0,
                // as
                0, 127, 1,
                // view
                4, 103, 2, 3,
                // results
                2,
                // as
                1, 126,
                // field
                5, 0,
                // get
                0, 0,
                // as
                1, 126,
                // field
                5, 1,
                // get
                0, 0,
                // number of binds
                1,
                // bind
                33, 22,
            ],
        );

        webidl_type_function(
            |b, m| WebidlType {
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
            |b, m| WebidlType {
                name: None,
                ty: WebidlCompoundType::Dictionary(WebidlDictionary {
                    fields: vec![
                        WebidlDictionaryField { name: "first".into(), ty: get_webidl_type_ref(b), },
                        WebidlDictionaryField { name: "second".into(), ty: get_webidl_type_ref(b), },
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
                11,
                // "second"
                6, 115, 101, 99, 111, 110, 100,
                11,
            ],
        );
        webidl_type_enumeration(
            |b, m| WebidlType {
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
            |b, m| WebidlType {
                name: None,
                ty: WebidlCompoundType::Union(WebidlUnion {
                    members: vec![get_webidl_type_ref(b), get_webidl_type_ref(b)]
                }),
            },
            [
                // Union type
                3,
                // Number of members
                2,
                11,
                11,
            ]
        );

        webidl_function_static(
            |b, m| WebidlFunction {
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
            |b, m| WebidlFunction {
                kind: WebidlFunctionKind::Method(WebidlFunctionKindMethod {
                    ty: get_webidl_type_ref(b),
                }),
                params: vec![],
                result: None
            },
            [
                // Method kind
                1,
                11,
                // Number of params
                0,
                // Has result?
                0,
            ]
        );
        webidl_function_constructor(
            |b, m| WebidlFunction {
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
            |b, m| WebidlFunction {
                kind: WebidlFunctionKind::Static,
                params: vec![get_webidl_type_ref(b), get_webidl_type_ref(b)],
                result: None
            },
            [
                // Static kind
                0,
                // Number of params
                2,
                11,
                11,
                // Has result?
                0,
            ]
        );
        webidl_function_result(
            |b, m| WebidlFunction {
                kind: WebidlFunctionKind::Static,
                params: vec![],
                result: Some(get_webidl_type_ref(b)),
            },
            [
                // Static kind
                0,
                // Number of params
                0,
                // Has result?
                1,
                11,
            ]
        );

        webidl_dictionary(
            |b, m| WebidlDictionary {
                fields: vec![
                    WebidlDictionaryField { name: "first".into(), ty: get_webidl_type_ref(b), },
                    WebidlDictionaryField { name: "second".into(), ty: get_webidl_type_ref(b), },
                ],
            },
            [
                // Number of fields
                2,
                // "first"
                5, 102, 105, 114, 115, 116,
                11,
                // "second"
                6, 115, 101, 99, 111, 110, 100,
                11,
            ],
        );

        webidl_enumeration(
            |b, m| WebidlEnumeration {
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
            |b, m| WebidlUnion {
                members: vec![get_webidl_type_ref(b), get_webidl_type_ref(b)],
            },
            [
                // Number of members
                2,
                11,
                11,
            ],
        );

        function_binding_export(
            |b, m| FunctionBinding::Export(ExportBinding {
                wasm_ty: get_wasm_func_type_ref(m),
                webidl_ty: get_webidl_type_ref(b),
                params: IncomingBindingMap { bindings: vec![] },
                result: OutgoingBindingMap { bindings: vec![] },
            }),
            [
                // export binding
                1,
                // wasm_ty
                44,
                // webidl_ty
                11,
                // 0 params
                0,
                // 0 results
                0,
            ],
        );

        function_binding_import(
            |b, m| FunctionBinding::Import(ImportBinding {
                wasm_ty: get_wasm_func_type_ref(m),
                webidl_ty: get_webidl_type_ref(b),
                params: OutgoingBindingMap { bindings: vec![] },
                result: IncomingBindingMap { bindings: vec![] },
            }),
            [
                // import binding
                0,
                // wasm_ty
                44,
                // webidl_ty
                11,
                // 0 params
                0,
                // 0 results
                0,
            ],
        );

        outgoing_binding_map(
            |b, m| OutgoingBindingMap {
                bindings: vec![
                    OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                        ty: get_webidl_type_ref(b),
                        idx: 1,
                    }),
                    OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                        ty: get_webidl_type_ref(b),
                        idx: 2,
                    }),
                ]
            },
            [
                // Number of expresssions
                2,
                // as
                0,
                11,
                // idx
                1,
                // as
                0,
                11,
                // idx
                2,
            ],
        );

        outgoing_binding_expression_as(
            |b, m| OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                ty: get_webidl_type_ref(b),
                idx: 2,
            }),
            [
                // as
                0,
                11,
                // idx
                2,
            ],
        );
        outgoing_binding_expression_utf8_str(
            |b, m| OutgoingBindingExpression::Utf8Str(OutgoingBindingExpressionUtf8Str {
                ty: get_webidl_type_ref(b),
                offset: 3,
                length: 4,
            }),
            [
                // utf8-str
                1,
                // get_webidl_type_ref(b)
                11,
                // offset
                3,
                // length
                4,
            ],
        );
        outgoing_binding_expression_utf8_cstr(
            |b, m| OutgoingBindingExpression::Utf8CStr(OutgoingBindingExpressionUtf8CStr {
                ty: get_webidl_type_ref(b),
                offset: 99,
            }),
            [
                // utf8-cstr
                2,
                11,
                // offset
                99,
            ],
        );
        outgoing_binding_expression_i32_to_enum(
            |b, m| OutgoingBindingExpression::I32ToEnum(OutgoingBindingExpressionI32ToEnum {
                ty: get_webidl_type_ref(b),
                idx: 4,
            }),
            [
                // i32-to-enum
                3,
                11,
                // idx
                4,
            ],
        );
        outgoing_binding_expression_view(
            |b, m| OutgoingBindingExpression::View(OutgoingBindingExpressionView {
                ty: get_webidl_type_ref(b),
                offset: 1,
                length: 2,
            }),
            [
                // view
                4,
                11,
                // offset
                1,
                // length
                2,
            ],
        );
        outgoing_binding_expression_copy(
            |b, m| OutgoingBindingExpression::Copy(OutgoingBindingExpressionCopy {
                ty: get_webidl_type_ref(b),
                offset: 1,
                length: 2,
            }),
            [
                // copy
                5,
                11,
                // offset
                1,
                // length
                2,
            ],
        );
        outgoing_binding_expression_dict(
            |b, m| OutgoingBindingExpression::Dict(OutgoingBindingExpressionDict {
                ty: get_webidl_type_ref(b),
                fields: vec![
                    OutgoingBindingExpression::As(OutgoingBindingExpressionAs {
                        ty: get_webidl_type_ref(b),
                        idx: 2,
                    }),
                ],
            }),
            [
                // dict
                6,
                11,
                // Number of fields
                1,
                // as
                0,
                11,
                2
            ],
        );
        outgoing_binding_expression_bind_export(
            |b, m| OutgoingBindingExpression::BindExport(OutgoingBindingExpressionBindExport {
                ty: get_webidl_type_ref(b),
                binding: get_binding_ref(b, m),
                idx: 3,
            }),
            [
                // bind-export
                7,
                11,
                22,
                3,
            ],
        );

        incoming_binding_map(
            |b, m| IncomingBindingMap {
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
            |b, m| IncomingBindingExpression::Get(IncomingBindingExpressionGet { idx: 1 }),
            [
                // get
                0,
                1,
            ]
        );
        incoming_binding_expression_as(
            |b, m| IncomingBindingExpression::As(IncomingBindingExpressionAs {
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
            |b, m| IncomingBindingExpression::AllocUtf8Str(IncomingBindingExpressionAllocUtf8Str {
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
            |b, m| IncomingBindingExpression::AllocCopy(IncomingBindingExpressionAllocCopy {
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
            |b, m| IncomingBindingExpression::EnumToI32(IncomingBindingExpressionEnumToI32 {
                ty: get_webidl_type_ref(b),
                expr: Box::new(IncomingBindingExpression::Get(IncomingBindingExpressionGet {
                    idx: 2,
                })),
            }),
            [
                // enum-to-i32
                4,
                11,
                // get
                0,
                2,
            ],
        );
        incoming_binding_expression_field(
            |b, m| IncomingBindingExpression::Field(IncomingBindingExpressionField {
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
            |b, m| IncomingBindingExpression::BindImport(IncomingBindingExpressionBindImport {
                ty: get_wasm_func_type_ref(m),
                binding: get_binding_ref(b, m),
                expr: Box::new(IncomingBindingExpression::Get(IncomingBindingExpressionGet {
                    idx: 3,
                })),
            }),
            [
                // bind-import
                6,
                44,
                22,
                // get
                0,
                3
            ],
        );

        bind(
            |b, m| Bind {
                func: get_wasm_func_ref(m),
                binding: get_binding_ref(b, m),
            },
            [33, 22],
        );

        webidl_type_ref(|b, m| get_webidl_type_ref(b), [11]);
    }
}
