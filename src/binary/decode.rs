use crate::ast::*;
use id_arena::Id;
use std::io::Read;

/// A trait implemented by every Web IDL bindings thing that can be decoded from
/// an input stream.
pub(crate) trait Decode {
    /// The output of decoding `Self` from the input stream.
    ///
    /// Since the custom section's arenas and AST nodes typically live inside
    /// the `cx`, this is usually some sort of typed ID pointing into an arena.
    type Output;

    /// Decode an instance of `Self` into the given `cx`.
    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read;
}

// Factored out to make testing easier.
pub(crate) trait Ids {
    fn webidl_bindings(&mut self) -> &mut WebidlBindings;
    fn webidl_type_id(&self, index: u32) -> Result<Id<WebidlCompoundType>, failure::Error>;
    fn binding_id(&self, index: u32) -> Result<Id<FunctionBinding>, failure::Error>;
    fn wasm_func_id(&self, index: u32) -> Result<walrus::FunctionId, failure::Error>;
    fn wasm_func_type_id(&self, index: u32) -> Result<walrus::TypeId, failure::Error>;
}

pub(crate) struct DecodeContext<'a> {
    ids: &'a walrus::IndicesToIds,
    pub(crate) webidl_bindings: WebidlBindings,
}

impl<'a> DecodeContext<'a> {
    pub(crate) fn new(ids: &'a walrus::IndicesToIds) -> Self {
        let webidl_bindings = WebidlBindings::default();
        DecodeContext {
            ids,
            webidl_bindings,
        }
    }
}

impl Ids for DecodeContext<'_> {
    fn webidl_bindings(&mut self) -> &mut WebidlBindings {
        &mut self.webidl_bindings
    }

    fn webidl_type_id(&self, index: u32) -> Result<Id<WebidlCompoundType>, failure::Error> {
        self.webidl_bindings
            .types
            .by_index(index)
            .ok_or_else(|| failure::format_err!("no Web IDL type for index {}", index))
    }

    fn binding_id(&self, index: u32) -> Result<Id<FunctionBinding>, failure::Error> {
        self.webidl_bindings
            .bindings
            .by_index(index)
            .ok_or_else(|| failure::format_err!("no function binding for index {}", index))
    }

    fn wasm_func_id(&self, index: u32) -> Result<walrus::FunctionId, failure::Error> {
        self.ids.get_func(index)
    }

    fn wasm_func_type_id(&self, index: u32) -> Result<walrus::TypeId, failure::Error> {
        self.ids.get_type(index)
    }
}

trait ReadExt: Read {
    fn read_byte(&mut self) -> Result<u8, failure::Error>;
    fn expect_byte(&mut self, expected: u8) -> Result<(), failure::Error>;
    fn uleb(&mut self) -> Result<u32, failure::Error>;
    fn ileb(&mut self) -> Result<i32, failure::Error>;
    fn vec<T, Cx, E>(&mut self, cx: &mut Cx, e: &mut E) -> Result<(), failure::Error>
    where
        T: Decode,
        Cx: Ids,
        E: Extend<<T as Decode>::Output>;
    fn option<T, Cx>(
        &mut self,
        cx: &mut Cx,
    ) -> Result<Option<<T as Decode>::Output>, failure::Error>
    where
        T: Decode,
        Cx: Ids;
    fn string(&mut self) -> Result<String, failure::Error>;
}

impl<R: Read> ReadExt for R {
    fn read_byte(&mut self) -> Result<u8, failure::Error> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn expect_byte(&mut self, expected: u8) -> Result<(), failure::Error> {
        let actual = self.read_byte()?;
        if actual == expected {
            Ok(())
        } else {
            failure::bail!("expected byte 0x{:02X}, found 0x{:02X}", expected, actual)
        }
    }

    fn uleb(&mut self) -> Result<u32, failure::Error> {
        let n = leb128::read::unsigned(self)?;
        if n <= (std::u32::MAX as u64) {
            Ok(n as u32)
        } else {
            failure::bail!("{} does not fit in a u32", n)
        }
    }

    fn ileb(&mut self) -> Result<i32, failure::Error> {
        let n = leb128::read::signed(self)?;
        if (std::i32::MIN as i64) <= n && n <= (std::i32::MAX as i64) {
            Ok(n as i32)
        } else {
            failure::bail!("{} does not fit in an i32", n)
        }
    }

    fn vec<T, Cx, E>(&mut self, cx: &mut Cx, e: &mut E) -> Result<(), failure::Error>
    where
        T: Decode,
        Cx: Ids,
        E: Extend<<T as Decode>::Output>,
    {
        // TODO: instead of repeatedly extending with `std::iter::once`, create
        // an iterable of all the parsed values, implement the size hint for it
        // with `n`, and only make a single `extend` call. This should allow the
        // thing being extended to reserve space for everything up front, which
        // should be more efficient.
        let n = self.uleb()?;
        for _ in 0..n {
            e.extend(std::iter::once(T::decode(cx, self)?));
        }
        Ok(())
    }

    fn option<T, Cx>(
        &mut self,
        cx: &mut Cx,
    ) -> Result<Option<<T as Decode>::Output>, failure::Error>
    where
        T: Decode,
        Cx: Ids,
    {
        match self.read_byte()? {
            0 => Ok(None),
            1 => Ok(Some(T::decode(cx, self)?)),
            n => failure::bail!(
                "expected 0x0 or 0x1, found bad option discriminant: 0x{:02X}",
                n
            ),
        }
    }

    fn string(&mut self) -> Result<String, failure::Error> {
        let n = self.uleb()?;
        let mut v = vec![0; n as usize];
        self.read_exact(&mut v)?;
        let s = String::from_utf8(v)?;
        Ok(s)
    }
}

struct Ignore;
impl<T> Extend<T> for Ignore {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        iter.into_iter().for_each(drop);
    }
}

impl Decode for String {
    type Output = Self;

    fn decode<Cx, R>(_cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        r.string()
    }
}

impl<D> Decode for Box<D>
where
    D: Decode,
{
    type Output = Box<D::Output>;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let d = D::decode(cx, r)?;
        Ok(Box::new(d))
    }
}

impl Decode for WebidlBindings {
    type Output = ();

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<(), failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        // Web IDL Type Subsection.
        WebidlTypes::decode(cx, r)?;

        // Web IDL Function Binding Subsection.
        r.expect_byte(1)?;

        // Function bindings.
        r.vec::<FunctionBinding, _, _>(cx, &mut Ignore)?;

        // Bind statements.
        r.vec::<Bind, _, _>(cx, &mut Ignore)
    }
}

impl Decode for WebidlTypes {
    type Output = ();

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<(), failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        r.expect_byte(0)?;
        r.vec::<WebidlType, _, _>(cx, &mut Ignore)
    }
}

impl Decode for WebidlType {
    type Output = Id<WebidlCompoundType>;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let id = WebidlCompoundType::decode(cx, r)?;
        Ok(id.into())
    }
}

impl Decode for WebidlCompoundType {
    type Output = Id<WebidlCompoundType>;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        match r.read_byte()? {
            0 => WebidlFunction::decode(cx, r).map(Into::into),
            1 => WebidlDictionary::decode(cx, r).map(Into::into),
            2 => WebidlEnumeration::decode(cx, r).map(Into::into),
            3 => WebidlUnion::decode(cx, r).map(Into::into),
            n => failure::bail!("unexpected Web IDL compound type discriminant: {}", n),
        }
    }
}

impl Decode for WebidlFunction {
    type Output = WebidlFunctionId;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let kind = WebidlFunctionKind::decode(cx, r)?;

        let mut params = vec![];
        r.vec::<WebidlTypeRef, _, _>(cx, &mut params)?;

        let result = r.option::<WebidlTypeRef, _>(cx)?;

        Ok(cx.webidl_bindings().types.insert(WebidlFunction {
            kind,
            params,
            result,
        }))
    }
}

impl Decode for WebidlFunctionKind {
    type Output = Self;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        match r.read_byte()? {
            0 => Ok(WebidlFunctionKind::Static),
            1 => {
                let ty = WebidlTypeRef::decode(cx, r)?;
                Ok(WebidlFunctionKind::Method(WebidlFunctionKindMethod { ty }))
            }
            2 => Ok(WebidlFunctionKind::Constructor),
            n => failure::bail!(
                "expected 0x0, 0x1, or 0x2, found bad Web IDL function kind discriminant: 0x{:02X}",
                n
            ),
        }
    }
}

impl Decode for WebidlTypeRef {
    type Output = Self;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        use WebidlScalarType::*;

        fn scalar(s: WebidlScalarType) -> Result<WebidlTypeRef, failure::Error> {
            Ok(s.into())
        }

        match r.ileb()? {
            // Scalar Web IDL types.
            -1 => scalar(Any),
            -2 => scalar(Boolean),
            -3 => scalar(Byte),
            -4 => scalar(Octet),
            -5 => scalar(Long),
            -6 => scalar(UnsignedLong),
            -7 => scalar(Short),
            -8 => scalar(UnsignedShort),
            -9 => scalar(LongLong),
            -10 => scalar(UnsignedLongLong),
            -11 => scalar(Float),
            -12 => scalar(UnrestrictedFloat),
            -13 => scalar(Double),
            -14 => scalar(UnrestrictedDouble),
            -15 => scalar(DomString),
            -16 => scalar(ByteString),
            -17 => scalar(UsvString),
            -18 => scalar(Object),
            -19 => scalar(Symbol),
            -20 => scalar(ArrayBuffer),
            -21 => scalar(DataView),
            -22 => scalar(Int8Array),
            -23 => scalar(Int16Array),
            -24 => scalar(Int32Array),
            -25 => scalar(Uint8Array),
            -26 => scalar(Uint16Array),
            -27 => scalar(Uint32Array),
            -28 => scalar(Uint8ClampedArray),
            -29 => scalar(Float32Array),
            -30 => scalar(Float64Array),

            // Indices of compound Web IDL types.
            n if n >= 0 => {
                let id = cx
                    .webidl_bindings()
                    .types
                    .by_index(n as u32)
                    .ok_or_else(|| {
                        failure::format_err!("{} is an invalid Web IDL compound type reference", n)
                    })?;
                Ok(WebidlTypeRef::Id(id))
            }

            // Bad Web IDL scalar type references.
            n => failure::bail!("reference to an unknown Web IDL scalar type: {}", n),
        }
    }
}

impl Decode for WebidlDictionary {
    type Output = WebidlDictionaryId;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let mut fields = vec![];
        r.vec::<WebidlDictionaryField, _, _>(cx, &mut fields)?;
        Ok(cx
            .webidl_bindings()
            .types
            .insert(WebidlDictionary { fields }))
    }
}

impl Decode for WebidlDictionaryField {
    type Output = Self;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let name = r.string()?;
        let ty = WebidlTypeRef::decode(cx, r)?;
        Ok(WebidlDictionaryField { name, ty })
    }
}

impl Decode for WebidlEnumeration {
    type Output = WebidlEnumerationId;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let mut values = vec![];
        r.vec::<String, _, _>(cx, &mut values)?;
        Ok(cx
            .webidl_bindings()
            .types
            .insert(WebidlEnumeration { values }))
    }
}

impl Decode for WebidlUnion {
    type Output = WebidlUnionId;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let mut members = vec![];
        r.vec::<WebidlTypeRef, _, _>(cx, &mut members)?;
        Ok(cx.webidl_bindings().types.insert(WebidlUnion { members }))
    }
}

impl Decode for FunctionBinding {
    type Output = Id<FunctionBinding>;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        match r.read_byte()? {
            0 => ImportBinding::decode(cx, r).map(Into::into),
            1 => ExportBinding::decode(cx, r).map(Into::into),
            n => failure::bail!(
                "expected 0x1 or 0x2, found unknown function binding discriminant 0x{:02X}",
                n
            ),
        }
    }
}

impl Decode for ImportBinding {
    type Output = ImportBindingId;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let wasm_ty = walrus::TypeId::decode(cx, r)?;
        let webidl_ty = WebidlTypeRef::decode(cx, r)?;
        let params = OutgoingBindingMap::decode(cx, r)?;
        let result = IncomingBindingMap::decode(cx, r)?;

        Ok(cx.webidl_bindings().bindings.insert(ImportBinding {
            wasm_ty,
            webidl_ty,
            params,
            result,
        }))
    }
}

impl Decode for ExportBinding {
    type Output = ExportBindingId;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let wasm_ty = walrus::TypeId::decode(cx, r)?;
        let webidl_ty = WebidlTypeRef::decode(cx, r)?;
        let params = IncomingBindingMap::decode(cx, r)?;
        let result = OutgoingBindingMap::decode(cx, r)?;

        Ok(cx.webidl_bindings().bindings.insert(ExportBinding {
            wasm_ty,
            webidl_ty,
            params,
            result,
        }))
    }
}

impl Decode for walrus::ValType {
    type Output = Self;

    fn decode<Cx, R>(_cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        match r.read_byte()? {
            0x7f => Ok(walrus::ValType::I32),
            0x7e => Ok(walrus::ValType::I64),
            0x7d => Ok(walrus::ValType::F32),
            0x7c => Ok(walrus::ValType::F64),
            0x7b => Ok(walrus::ValType::V128),
            0x6f => Ok(walrus::ValType::Anyref),
            n => failure::bail!("invalid valtype encoding: 0x{:02X}", n),
        }
    }
}

impl Decode for walrus::TypeId {
    type Output = Self;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let index = r.uleb()?;
        cx.wasm_func_type_id(index)
    }
}

impl Decode for OutgoingBindingMap {
    type Output = Self;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let mut bindings = vec![];
        r.vec::<OutgoingBindingExpression, _, _>(cx, &mut bindings)?;
        Ok(OutgoingBindingMap { bindings })
    }
}

impl Decode for OutgoingBindingExpression {
    type Output = Self;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        fn e<T: Into<OutgoingBindingExpression>>(
            e: T,
        ) -> Result<OutgoingBindingExpression, failure::Error> {
            Ok(e.into())
        }

        match r.read_byte()? {
            0 => {
                let ty = WebidlTypeRef::decode(cx, r)?;
                let idx = r.uleb()?;
                e(OutgoingBindingExpressionAs { ty, idx })
            }
            1 => {
                let ty = WebidlTypeRef::decode(cx, r)?;
                let offset = r.uleb()?;
                let length = r.uleb()?;
                e(OutgoingBindingExpressionUtf8Str { ty, offset, length })
            }
            2 => {
                let ty = WebidlTypeRef::decode(cx, r)?;
                let offset = r.uleb()?;
                e(OutgoingBindingExpressionUtf8CStr { ty, offset })
            }
            3 => {
                let ty = WebidlTypeRef::decode(cx, r)?;
                let idx = r.uleb()?;
                e(OutgoingBindingExpressionI32ToEnum { ty, idx })
            }
            4 => {
                let ty = WebidlTypeRef::decode(cx, r)?;
                let offset = r.uleb()?;
                let length = r.uleb()?;
                e(OutgoingBindingExpressionView { ty, offset, length })
            }
            5 => {
                let ty = WebidlTypeRef::decode(cx, r)?;
                let offset = r.uleb()?;
                let length = r.uleb()?;
                e(OutgoingBindingExpressionCopy { ty, offset, length })
            }
            6 => {
                let ty = WebidlTypeRef::decode(cx, r)?;
                let mut fields = vec![];
                r.vec::<OutgoingBindingExpression, _, _>(cx, &mut fields)?;
                e(OutgoingBindingExpressionDict { ty, fields })
            }
            7 => {
                let ty = WebidlTypeRef::decode(cx, r)?;
                let binding = <Id<FunctionBinding>>::decode(cx, r)?;
                let idx = r.uleb()?;
                e(OutgoingBindingExpressionBindExport { ty, binding, idx })
            }
            n => failure::bail!(
                "unknown outgoing binding expression discriminant: 0x{:02X}",
                n
            ),
        }
    }
}

impl Decode for Id<FunctionBinding> {
    type Output = Self;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let index = r.uleb()?;
        cx.binding_id(index)
    }
}

impl Decode for IncomingBindingMap {
    type Output = Self;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let mut bindings = vec![];
        r.vec::<IncomingBindingExpression, _, _>(cx, &mut bindings)?;
        Ok(IncomingBindingMap { bindings })
    }
}

impl Decode for IncomingBindingExpression {
    type Output = Self;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        fn e<T: Into<IncomingBindingExpression>>(
            e: T,
        ) -> Result<IncomingBindingExpression, failure::Error> {
            Ok(e.into())
        }

        match r.read_byte()? {
            0 => {
                let idx = r.uleb()?;
                e(IncomingBindingExpressionGet { idx })
            }
            1 => {
                let ty = walrus::ValType::decode(cx, r)?;
                let expr = <Box<IncomingBindingExpression>>::decode(cx, r)?;
                e(IncomingBindingExpressionAs { ty, expr })
            }
            2 => {
                let alloc_func_name = String::decode(cx, r)?;
                let expr = <Box<IncomingBindingExpression>>::decode(cx, r)?;
                e(IncomingBindingExpressionAllocUtf8Str {
                    alloc_func_name,
                    expr,
                })
            }
            3 => {
                let alloc_func_name = String::decode(cx, r)?;
                let expr = <Box<IncomingBindingExpression>>::decode(cx, r)?;
                e(IncomingBindingExpressionAllocCopy {
                    alloc_func_name,
                    expr,
                })
            }
            4 => {
                let ty = WebidlTypeRef::decode(cx, r)?;
                let expr = <Box<IncomingBindingExpression>>::decode(cx, r)?;
                e(IncomingBindingExpressionEnumToI32 { ty, expr })
            }
            5 => {
                let idx = r.uleb()?;
                let expr = <Box<IncomingBindingExpression>>::decode(cx, r)?;
                e(IncomingBindingExpressionField { idx, expr })
            }
            6 => {
                let ty = walrus::TypeId::decode(cx, r)?;
                let binding = <Id<FunctionBinding>>::decode(cx, r)?;
                let expr = <Box<IncomingBindingExpression>>::decode(cx, r)?;
                e(IncomingBindingExpressionBindImport { ty, binding, expr })
            }
            n => failure::bail!(
                "unknown incoming binding expression discriminant: 0x{:02X}",
                n
            ),
        }
    }
}

impl Decode for Bind {
    type Output = Id<Bind>;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let func = walrus::FunctionId::decode(cx, r)?;
        let binding = <Id<FunctionBinding>>::decode(cx, r)?;
        Ok(cx.webidl_bindings().binds.insert(Bind { func, binding }))
    }
}

impl Decode for walrus::FunctionId {
    type Output = Self;

    fn decode<Cx, R>(cx: &mut Cx, r: &mut R) -> Result<Self::Output, failure::Error>
    where
        Cx: Ids,
        R: Read,
    {
        let index = r.uleb()?;
        cx.wasm_func_id(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    fn make_test_module() -> walrus::Module {
        let mut config = walrus::ModuleConfig::new();
        config.generate_producers_section(false);

        let mut module = walrus::Module::with_config(config);

        let ty = module
            .types
            .add(&[walrus::ValType::I32], &[walrus::ValType::I32]);

        let mut builder = walrus::FunctionBuilder::new();
        let arg = module.locals.add(walrus::ValType::I32);
        let lhs = builder.local_get(arg);
        let rhs = builder.i32_const(2);
        let mul = builder.binop(walrus::ir::BinaryOp::I32Mul, lhs, rhs);
        builder.finish(ty, vec![arg], vec![mul], &mut module);

        module
    }

    lazy_static::lazy_static! {
        static ref WASM_BUF: Vec<u8> = make_test_module()
            .emit_wasm()
            .expect("should emit test module's wasm");
    }

    fn with_test_module<F>(f: F)
    where
        F: Fn(&mut walrus::Module, &walrus::IndicesToIds) + Send + Sync + 'static,
    {
        let mut config = walrus::ModuleConfig::new();
        config.generate_producers_section(false);
        config.on_parse(move |module, ids| {
            f(module, ids);
            Ok(())
        });
        let result = config.parse(&*WASM_BUF);
        assert!(result.is_ok());
    }

    fn get_type_id(m: &walrus::Module) -> walrus::TypeId {
        m.types.iter().nth(0).unwrap().id()
    }

    fn get_my_dict_id(b: &WebidlBindings) -> WebidlTypeRef {
        b.types.arena.iter().nth(0).unwrap().0.into()
    }

    fn get_my_enum_id(b: &WebidlBindings) -> WebidlTypeRef {
        b.types.arena.iter().nth(1).unwrap().0.into()
    }

    fn get_my_func_id(b: &WebidlBindings) -> WebidlTypeRef {
        b.types.arena.iter().nth(2).unwrap().0.into()
    }

    fn get_my_import_binding(b: &WebidlBindings) -> Id<FunctionBinding> {
        b.bindings.arena.iter().nth(0).unwrap().0
    }

    fn get_my_export_binding(b: &WebidlBindings) -> Id<FunctionBinding> {
        b.bindings.arena.iter().nth(1).unwrap().0
    }

    fn with_decode_context<F>(f: F)
    where
        F: Fn(&mut walrus::Module, &mut DecodeContext) + Send + Sync + 'static,
    {
        #![allow(unused_variables)]

        with_test_module(move |module, ids| {
            let mut cx = DecodeContext::new(ids);

            // Now insert a bunch of stuff into the custom section so that we
            // have things to test against.
            let wb = cx.webidl_bindings();

            // Index 0
            let my_dict = wb.types.insert(WebidlDictionary {
                fields: vec![WebidlDictionaryField {
                    name: "foo".into(),
                    ty: WebidlTypeRef::Scalar(WebidlScalarType::Any),
                }],
            });

            // Index 1
            let my_enum = wb.types.insert(WebidlEnumeration {
                values: vec!["Foo".into(), "Bar".into()],
            });

            // Index 2
            let my_func = wb.types.insert(WebidlFunction {
                kind: WebidlFunctionKind::Static,
                params: vec![WebidlScalarType::UnsignedLong.into()],
                result: Some(WebidlScalarType::UnsignedLong.into()),
            });

            // Index 0
            let my_import_binding = wb.bindings.insert(ImportBinding {
                wasm_ty: get_type_id(module),
                webidl_ty: my_func.into(),
                params: OutgoingBindingMap {
                    bindings: vec![OutgoingBindingExpressionAs {
                        ty: WebidlScalarType::UnsignedLong.into(),
                        idx: 0,
                    }
                    .into()],
                },
                result: IncomingBindingMap {
                    bindings: vec![IncomingBindingExpressionAs {
                        ty: walrus::ValType::I32,
                        expr: Box::new(IncomingBindingExpressionGet { idx: 0 }.into()),
                    }
                    .into()],
                },
            });

            // Index 1
            let my_export_binding = wb.bindings.insert(ExportBinding {
                wasm_ty: get_type_id(module),
                webidl_ty: my_func.into(),
                params: IncomingBindingMap {
                    bindings: vec![IncomingBindingExpressionAs {
                        ty: walrus::ValType::I32,
                        expr: Box::new(IncomingBindingExpressionGet { idx: 0 }.into()),
                    }
                    .into()],
                },
                result: OutgoingBindingMap {
                    bindings: vec![OutgoingBindingExpressionAs {
                        ty: WebidlScalarType::UnsignedLong.into(),
                        idx: 0,
                    }
                    .into()],
                },
            });

            f(module, &mut cx);
        });
    }

    fn do_assert_decode_ok<F, T>(
        module: &walrus::Module,
        cx: &mut DecodeContext,
        expected: F,
        mut encoded: &[u8],
    ) where
        F: FnOnce(&walrus::Module, &walrus::IndicesToIds, &WebidlBindings) -> T::Output,
        T: Decode,
        T::Output: PartialEq + Debug,
    {
        let actual = T::decode(cx, &mut encoded).expect("should decode OK");
        let expected = expected(module, cx.ids, &cx.webidl_bindings);
        assert_eq!(expected, actual);
    }

    fn do_assert_decode_err<T>(cx: &mut DecodeContext, mut encoded: &[u8])
    where
        T: Decode,
    {
        let result = T::decode(cx, &mut encoded);
        assert!(result.is_err());
    }

    macro_rules! assert_decode_ok {
        (
            $ty:ty,
            $(
                $name:ident(
                    |$module:ident, $ids:ident, $bindings:ident| $expected:expr,
                    $encoded:expr $(,)*
                ) $(,)*
            )*
        ) => {
            $(
                #[test]
                #[allow(unused_variables)]
                fn $name() {
                    with_decode_context(|module, cx| {
                        let encoded: &[u8] = &$encoded;
                        do_assert_decode_ok::<_, $ty>(
                            module,
                            cx,
                            |$module, $ids, $bindings| $expected,
                            encoded,
                        );
                    });
                }
            )*
        };
    }

    macro_rules! assert_decode_err {
        (
            $ty:ty,
            $(
                $name:ident($encoded:expr $(,)* ) $(,)*
            )*
        ) => {
            $(
                #[test]
                fn $name() {
                    with_decode_context(|_module, cx| {
                        let encoded: &[u8] = &$encoded;
                        do_assert_decode_err::<$ty>(cx, encoded);
                    });
                }
            )*
        };
    }

    // String
    assert_decode_ok!(
        String,
        string_ok_0(
            |m, i, b| "foo".to_string(),
            [
                3, // length
                b'f', b'o', b'o',
            ],
        ),
        string_ok_1(
            |m, i, b| "".to_string(),
            [
                0, // length
            ],
        ),
    );
    assert_decode_err!(
        String,
        string_err_0([]),
        string_err_1([
            1, // length
               // no chars
        ]),
    );

    // WebidlBindings
    assert_decode_ok!(
        WebidlBindings,
        webidl_bindings_ok_0(
            |m, i, b| {},
            [
                0, // types subsection
                0, // number of types
                1, // bindings subsection
                0, // number of bindings
                0, // number of bind statements
            ],
        ),
    );
    assert_decode_err!(
        WebidlBindings,
        // Empty input stream.
        webidl_bindings_err_0([]),
        // Bad sub-section discriminant.
        webidl_bindings_err_1([2]),
        webidl_bindings_err_2([
            0, // types subsection
            0, // number of types
            1, // bindings subsection
            0, // number of bindings
               // no number of bind statements
        ]),
        webidl_bindings_err_3([
            0, // types subsection
            0, // number of types
            1, // bindings subsection
               // no number of bindings
        ]),
        webidl_bindings_err_4([
            0, // types subsection
            0, // number of types
               // no bindings subsection
        ]),
    );

    // WebidlTypes
    assert_decode_ok!(
        WebidlTypes,
        webidl_types_ok_0(
            |m, i, b| {},
            [
                0, // subsection number
                0, // number of types
            ]
        ),
    );
    assert_decode_err!(
        WebidlTypes,
        // Empty.
        webidl_types_err_0([]),
        webidl_types_err_1([
            0, // subsection number
            1, // number of types
               // no types
        ]),
        webidl_types_err_2([
            0, // subsection number
               // no number-of-types
        ]),
    );

    // WebidlCompoundType
    assert_decode_ok!(
        WebidlCompoundType,
        webidl_compound_type_ok_0(
            |m, i, b| {
                let (id, ty) = b.types.arena.iter().last().unwrap();
                assert!(match ty {
                    WebidlCompoundType::Function(_) => true,
                    _ => false,
                });
                id
            },
            [
                0,    // function discriminant
                0,    // static kind
                1,    // number of params
                0x7f, // any
                0,    // no result
            ]
        ),
        webidl_compound_type_ok_1(
            |m, i, b| {
                let (id, ty) = b.types.arena.iter().last().unwrap();
                assert!(match ty {
                    WebidlCompoundType::Dictionary(_) => true,
                    _ => false,
                });
                id
            },
            [
                1, // dictionary discriminant
                0, // number of fields
            ]
        ),
        webidl_compound_type_ok_2(
            |m, i, b| {
                let (id, ty) = b.types.arena.iter().last().unwrap();
                assert!(match ty {
                    WebidlCompoundType::Enumeration(_) => true,
                    _ => false,
                });
                id
            },
            [
                2, // enum discriminant
                0, // number of members
            ]
        ),
        webidl_compound_type_ok_3(
            |m, i, b| {
                let (id, ty) = b.types.arena.iter().last().unwrap();
                assert!(match ty {
                    WebidlCompoundType::Union(_) => true,
                    _ => false,
                });
                id
            },
            [
                3, // union discriminant
                0, // number of variants
            ]
        ),
    );
    assert_decode_err!(
        WebidlCompoundType,
        // Empty.
        webidl_compound_type_err_0([]),
        // Bad discriminant.
        webidl_compound_type_err_1([4]),
    );

    // WebidlFunction
    assert_decode_ok!(
        WebidlFunction,
        webidl_function_ok_0(
            |m, i, b| {
                let (id, ty) = b.types.arena.iter().last().unwrap();
                let ty = match ty {
                    WebidlCompoundType::Function(f) => f,
                    _ => panic!(),
                };
                assert_eq!(ty.params, [WebidlScalarType::Any.into()]);
                assert_eq!(ty.result, Some(WebidlScalarType::Any.into()));
                <WebidlFunction as WebidlTypeId>::wrap(id)
            },
            [
                0,    // static kind
                1,    // number of params
                0x7f, // any
                1,    // has a result
                0x7f, // any
            ],
        ),
        webidl_function_ok_1(
            |m, i, b| {
                let (id, ty) = b.types.arena.iter().last().unwrap();
                let ty = match ty {
                    WebidlCompoundType::Function(f) => f,
                    _ => panic!(),
                };
                assert_eq!(ty.params, [WebidlScalarType::Any.into()]);
                assert_eq!(ty.result, None);
                <WebidlFunction as WebidlTypeId>::wrap(id)
            },
            [
                0,    // static kind
                1,    // number of params
                0x7f, // any
                0,    // no result
            ],
        ),
    );
    assert_decode_err!(
        WebidlFunction,
        // Empty.
        webidl_function_err_0([]),
        webidl_function_err_1([
            0,    // static kind
            1,    // number of params
            0x7f, // any
            1,    // has a result
                  // no result
        ]),
        webidl_function_err_2([
            0, // static kind
            1, // number of params
            0x7f, // any
               // no has-a-result
        ]),
        webidl_function_err_3([
            0, // static kind
            1, // number of params
               // no param
        ]),
        webidl_function_err_4([
            0, // static kind
               // no number-of-params
        ]),
    );

    // WebidlFunctionKind
    assert_decode_ok!(
        WebidlFunctionKind,
        webidl_function_kind_ok_0(|m, i, b| WebidlFunctionKind::Static, [0]),
        webidl_function_kind_ok_1(
            |m, i, b| WebidlFunctionKindMethod {
                ty: WebidlScalarType::Any.into()
            }
            .into(),
            [1, 0x7f]
        ),
        webidl_function_kind_ok_2(|m, i, b| WebidlFunctionKind::Constructor, [2]),
    );
    assert_decode_err!(
        WebidlFunctionKind,
        // Empty.
        webidl_function_kind_err_0([]),
        // Bad discriminant.
        webidl_function_kind_err_1([3]),
    );

    // WebidlTypeRef
    assert_decode_ok!(
        WebidlTypeRef,
        // Indices/identifiers.
        webidl_type_ref_ok_0(|m, i, b| get_my_dict_id(b), [0]),
        webidl_type_ref_ok_1(|m, i, b| get_my_enum_id(b), [1]),
        webidl_type_ref_ok_2(|m, i, b| get_my_func_id(b), [2]),
        // Scalars.
        webidl_type_ref_ok_3(|m, i, b| WebidlScalarType::Any.into(), [0x7f]),
        webidl_type_ref_ok_4(|m, i, b| WebidlScalarType::Boolean.into(), [0x7e]),
        webidl_type_ref_ok_5(|m, i, b| WebidlScalarType::Byte.into(), [0x7d]),
        webidl_type_ref_ok_6(|m, i, b| WebidlScalarType::Octet.into(), [0x7c]),
        webidl_type_ref_ok_7(|m, i, b| WebidlScalarType::Long.into(), [0x7b]),
        webidl_type_ref_ok_8(|m, i, b| WebidlScalarType::UnsignedLong.into(), [0x7a]),
        webidl_type_ref_ok_9(|m, i, b| WebidlScalarType::Short.into(), [0x79]),
        webidl_type_ref_ok_10(|m, i, b| WebidlScalarType::UnsignedShort.into(), [0x78]),
        webidl_type_ref_ok_11(|m, i, b| WebidlScalarType::LongLong.into(), [0x77]),
        webidl_type_ref_ok_12(|m, i, b| WebidlScalarType::UnsignedLongLong.into(), [0x76]),
        webidl_type_ref_ok_13(|m, i, b| WebidlScalarType::Float.into(), [0x75]),
        webidl_type_ref_ok_14(|m, i, b| WebidlScalarType::UnrestrictedFloat.into(), [0x74]),
        webidl_type_ref_ok_15(|m, i, b| WebidlScalarType::Double.into(), [0x73]),
        webidl_type_ref_ok_16(
            |m, i, b| WebidlScalarType::UnrestrictedDouble.into(),
            [0x72]
        ),
        webidl_type_ref_ok_17(|m, i, b| WebidlScalarType::DomString.into(), [0x71]),
        webidl_type_ref_ok_18(|m, i, b| WebidlScalarType::ByteString.into(), [0x70]),
        webidl_type_ref_ok_19(|m, i, b| WebidlScalarType::UsvString.into(), [0x6f]),
        webidl_type_ref_ok_20(|m, i, b| WebidlScalarType::Object.into(), [0x6e]),
        webidl_type_ref_ok_21(|m, i, b| WebidlScalarType::Symbol.into(), [0x6d]),
        webidl_type_ref_ok_22(|m, i, b| WebidlScalarType::ArrayBuffer.into(), [0x6c]),
        webidl_type_ref_ok_23(|m, i, b| WebidlScalarType::DataView.into(), [0x6b]),
        webidl_type_ref_ok_24(|m, i, b| WebidlScalarType::Int8Array.into(), [0x6a]),
        webidl_type_ref_ok_25(|m, i, b| WebidlScalarType::Int16Array.into(), [0x69]),
        webidl_type_ref_ok_26(|m, i, b| WebidlScalarType::Int32Array.into(), [0x68]),
        webidl_type_ref_ok_27(|m, i, b| WebidlScalarType::Uint8Array.into(), [0x67]),
        webidl_type_ref_ok_28(|m, i, b| WebidlScalarType::Uint16Array.into(), [0x66]),
        webidl_type_ref_ok_29(|m, i, b| WebidlScalarType::Uint32Array.into(), [0x65]),
        webidl_type_ref_ok_30(|m, i, b| WebidlScalarType::Uint8ClampedArray.into(), [0x64]),
        webidl_type_ref_ok_31(|m, i, b| WebidlScalarType::Float32Array.into(), [0x63]),
        webidl_type_ref_ok_32(|m, i, b| WebidlScalarType::Float64Array.into(), [0x62]),
    );
    assert_decode_err!(
        WebidlTypeRef,
        // Empty.
        webidl_type_ref_err_0([]),
        // Bad positive ileb index.
        webidl_type_ref_err_1([0x3f]),
        // Bad negative ileb scalar.
        webidl_type_ref_err_2([0x40]),
    );

    // WebidlDictionary
    assert_decode_ok!(
        WebidlDictionary,
        webidl_dictionary_ok_0(
            |m, i, b| {
                let (id, ty) = b.types.arena.iter().last().unwrap();
                let ty = match ty {
                    WebidlCompoundType::Dictionary(d) => d,
                    _ => panic!(),
                };
                assert_eq!(
                    ty.fields,
                    [
                        WebidlDictionaryField {
                            name: "a".into(),
                            ty: WebidlScalarType::Any.into()
                        },
                        WebidlDictionaryField {
                            name: "b".into(),
                            ty: WebidlScalarType::Any.into()
                        },
                    ]
                );
                <WebidlDictionary as WebidlTypeId>::wrap(id)
            },
            [
                2, // number of fields
                1, b'a', // name
                0x7f, // any
                1, b'b', // name
                0x7f, // any
            ]
        ),
    );
    assert_decode_err!(
        WebidlDictionary,
        webidl_dictionary_err_0([]),
        webidl_dictionary_err_1([
            2, // number of fields
               // no fields
        ]),
    );

    // WebidlDictionaryField
    assert_decode_ok!(
        WebidlDictionaryField,
        webidl_dictionary_field_ok_0(
            |m, i, b| WebidlDictionaryField {
                name: "a".into(),
                ty: WebidlScalarType::Any.into()
            },
            [
                1, b'a', // name
                0x7f, // any
            ]
        ),
    );
    assert_decode_err!(
        WebidlDictionaryField,
        webidl_dictionary_field_err_0([
            // Empty.
        ]),
        webidl_dictionary_field_err_1([
            1, b'a', // name
                  // no ty
        ]),
    );

    // WebidlEnumeration
    assert_decode_ok!(
        WebidlEnumeration,
        webidl_enumeration_ok_0(
            |m, i, b| {
                let (id, ty) = b.types.arena.iter().last().unwrap();
                let ty = match ty {
                    WebidlCompoundType::Enumeration(e) => e,
                    _ => panic!(),
                };
                assert!(ty.values.is_empty());
                <WebidlEnumeration as WebidlTypeId>::wrap(id)
            },
            [
                0, // number of values
            ],
        ),
        webidl_enumeration_ok_1(
            |m, i, b| {
                let (id, ty) = b.types.arena.iter().last().unwrap();
                let ty = match ty {
                    WebidlCompoundType::Enumeration(e) => e,
                    _ => panic!(),
                };
                assert_eq!(ty.values, ["a".to_string(), "b".to_string()]);
                <WebidlEnumeration as WebidlTypeId>::wrap(id)
            },
            [
                2, // number of values
                1, b'a', // "a"
                1, b'b', // "b"
            ],
        ),
    );
    assert_decode_err!(
        WebidlEnumeration,
        webidl_enumeration_err_0([
            // Empty input stream.
        ]),
        webidl_enumeration_err_1([
            2, // number of values
               // no values
        ]),
    );

    // WebidlUnion
    assert_decode_ok!(
        WebidlUnion,
        webidl_union_ok_0(
            |m, i, b| {
                let (id, ty) = b.types.arena.iter().last().unwrap();
                let ty = match ty {
                    WebidlCompoundType::Union(u) => u,
                    _ => panic!(),
                };
                assert_eq!(ty.members, []);
                <WebidlUnion as WebidlTypeId>::wrap(id)
            },
            [
                0, // number of members
            ]
        ),
        webidl_union_ok_1(
            |m, i, b| {
                let (id, ty) = b.types.arena.iter().last().unwrap();
                let ty = match ty {
                    WebidlCompoundType::Union(u) => u,
                    _ => panic!(),
                };
                assert_eq!(
                    ty.members,
                    [
                        WebidlScalarType::Boolean.into(),
                        WebidlScalarType::Long.into()
                    ]
                );
                <WebidlUnion as WebidlTypeId>::wrap(id)
            },
            [
                2,    // number of members
                0x7e, // boolean
                0x7b, // long
            ]
        ),
    );
    assert_decode_err!(
        WebidlUnion,
        webidl_union_err_0([
            // Empty input stream.
        ]),
        webidl_union_err_1([
            2, // number of members
               // no members
        ]),
    );

    // FunctionBinding
    assert_decode_ok!(
        FunctionBinding,
        function_binding_ok_0(
            |m, i, b| {
                let (id, binding) = b.bindings.arena.iter().last().unwrap();
                assert!(match binding {
                    FunctionBinding::Import(_) => true,
                    FunctionBinding::Export(_) => false,
                });
                id
            },
            [
                0,    // import binding discriminant
                0,    // type id
                2,    // my_func
                1,    // number of params
                0,    // discriminant
                0x7a, // unsigned long
                0,    // idx
                1,    // number of results
                1,    // discriminant
                0x7f, // i32
                0,    // discriminant
                0,    // idx
            ]
        ),
        function_binding_ok_1(
            |m, i, b| {
                let (id, binding) = b.bindings.arena.iter().last().unwrap();
                assert!(match binding {
                    FunctionBinding::Import(_) => false,
                    FunctionBinding::Export(_) => true,
                });
                id
            },
            [
                1,    // export binding discriminant
                0,    // type id
                2,    // my_func
                1,    // number of params
                1,    // discriminant
                0x7f, // i32
                0,    // discriminant
                0,    // idx
                1,    // number of results
                0,    // discriminant
                0x7a, // unsigned long
                0,    // idx
            ]
        ),
    );
    assert_decode_err!(
        FunctionBinding,
        function_binding_err_0([
            // Empty input stream.
        ]),
        function_binding_err_1([
            2, // Bad discriminant.
        ]),
    );

    // ImportBinding
    assert_decode_ok!(
        ImportBinding,
        import_binding_ok_0(
            |m, i, b| {
                let (id, binding) = b.bindings.arena.iter().last().unwrap();
                let binding = match binding {
                    FunctionBinding::Import(i) => i,
                    _ => panic!(),
                };
                assert_eq!(binding.wasm_ty, get_type_id(m));
                assert_eq!(binding.webidl_ty, get_my_func_id(b));
                assert_eq!(
                    binding.params.bindings,
                    vec![OutgoingBindingExpressionAs {
                        ty: WebidlScalarType::UnsignedLong.into(),
                        idx: 0,
                    }
                    .into()]
                );
                assert_eq!(
                    binding.result.bindings,
                    vec![IncomingBindingExpressionAs {
                        ty: walrus::ValType::I32,
                        expr: Box::new(IncomingBindingExpressionGet { idx: 0 }.into()),
                    }
                    .into()]
                );
                <ImportBinding as FunctionBindingId>::wrap(id)
            },
            [
                0,    // type id
                2,    // my_func
                1,    // number of params
                0,    // discriminant
                0x7a, // unsigned long
                0,    // idx
                1,    // number of results
                1,    // discriminant
                0x7f, // i32
                0,    // discriminant
                0,    // idx
            ],
        ),
    );
    assert_decode_err!(
        ImportBinding,
        import_binding_err_0([
            0,    // type id
            2,    // my_func
            1,    // number of params
            0,    // discriminant
            0x7a, // unsigned long
            0,    // idx
            1,    // number of results
                  // no results
        ]),
        import_binding_err_1([
            0,    // type id
            2,    // my_func
            1,    // number of params
            0,    // discriminant
            0x7a, // unsigned long
            0,    // idx
                  // no number of results
        ]),
        import_binding_err_2([
            0, // type id
            2, // my_func
            1, // number of params
               // no params
        ]),
        import_binding_err_3([
            0, // type id
            2, // my_func
               // no number of params
        ]),
        import_binding_err_4([
            0, // type id
               // no func ty
        ]),
        import_binding_err_5([
            // Empty input stream.
        ]),
    );

    // ExportBinding
    assert_decode_ok!(
        ExportBinding,
        export_binding_ok_0(
            |m, i, b| {
                let (id, binding) = b.bindings.arena.iter().last().unwrap();
                let binding = match binding {
                    FunctionBinding::Export(e) => e,
                    _ => panic!(),
                };
                assert_eq!(binding.wasm_ty, get_type_id(m));
                assert_eq!(binding.webidl_ty, get_my_func_id(b));
                assert_eq!(
                    binding.params.bindings,
                    vec![IncomingBindingExpressionAs {
                        ty: walrus::ValType::I32,
                        expr: Box::new(IncomingBindingExpressionGet { idx: 0 }.into()),
                    }
                    .into()]
                );
                assert_eq!(
                    binding.result.bindings,
                    vec![OutgoingBindingExpressionAs {
                        ty: WebidlScalarType::UnsignedLong.into(),
                        idx: 0,
                    }
                    .into()]
                );
                <ExportBinding as FunctionBindingId>::wrap(id)
            },
            [
                0,    // type id
                2,    // my_func
                1,    // number of params
                1,    // discriminant
                0x7f, // i32
                0,    // discriminant
                0,    // idx
                1,    // number of results
                0,    // discriminant
                0x7a, // unsigned long
                0,    // idx
            ],
        ),
    );
    assert_decode_err!(
        ExportBinding,
        export_binding_err_0([
            0,    // type id
            2,    // my_func
            1,    // number of params
            1,    // discriminant
            0x7f, // i32
            0,    // discriminant
            0,    // idx
            1,    // number of results
                  // no results
        ]),
        export_binding_err_1([
            0,    // type id
            2,    // my_func
            1,    // number of params
            1,    // discriminant
            0x7f, // i32
            0,    // discriminant
            0,    // idx
                  // no number of results
        ]),
        export_binding_err_2([
            0, // type id
            2, // my_func
            1, // number of params
               // no params
        ]),
        export_binding_err_3([
            0, // type id
            2, // my_func
               // no number of params
        ]),
        export_binding_err_4([
            0, // type id
               // no func ty
        ]),
        export_binding_err_5([
            // Empty input stream.
        ]),
    );

    // walrus::ValType
    assert_decode_ok!(
        walrus::ValType,
        valtype_ok_0(|m, i, b| walrus::ValType::I32, [0x7f]),
        valtype_ok_1(|m, i, b| walrus::ValType::I64, [0x7e]),
        valtype_ok_2(|m, i, b| walrus::ValType::F32, [0x7d]),
        valtype_ok_3(|m, i, b| walrus::ValType::F64, [0x7c]),
        valtype_ok_4(|m, i, b| walrus::ValType::V128, [0x7b]),
        valtype_ok_5(|m, i, b| walrus::ValType::Anyref, [0x6f]),
    );
    assert_decode_err!(
        walrus::ValType,
        // Empty input stream.
        valtype_err_0([]),
        // Unknown negative ileb.
        valtype_err_1([0x66]),
        // Positive ileb.
        valtype_err_2([4]),
    );

    // walrus::TypeId
    assert_decode_ok!(
        walrus::TypeId,
        type_id_ok_0(
            |m, i, b| get_type_id(m),
            [
                0, // type id
            ],
        ),
    );
    assert_decode_err!(
        walrus::TypeId,
        // Empty input stream.
        type_id_err_0([]),
        // Index that isn't associated with a type.
        type_id_err_1([99]),
    );

    // OutgoingBindingMap
    assert_decode_ok!(
        OutgoingBindingMap,
        outgoing_binding_map_ok_0(
            |m, i, b| OutgoingBindingMap { bindings: vec![] },
            [
                0, // number of bindings
            ],
        ),
        outgoing_binding_map_ok_1(
            |m, i, b| OutgoingBindingMap {
                bindings: vec![OutgoingBindingExpressionAs {
                    ty: WebidlScalarType::Any.into(),
                    idx: 3
                }
                .into()]
            },
            [
                1,    // number of bindings
                0,    // discriminant
                0x7f, // any
                3,    // idx
            ],
        ),
    );
    assert_decode_err!(
        OutgoingBindingMap,
        outgoing_binding_map_err_0([
            1, // number of bindings
               // no bindings
        ]),
        // Empty input stream.
        outgoing_binding_map_err_1([]),
    );

    // OutgoingBindingExpression
    fn obe<T: Into<OutgoingBindingExpression>>(t: T) -> OutgoingBindingExpression {
        t.into()
    }
    assert_decode_ok!(
        OutgoingBindingExpression,
        outgoing_binding_expression_ok_0(
            |m, i, b| obe(OutgoingBindingExpressionAs {
                ty: WebidlScalarType::Any.into(),
                idx: 3,
            }),
            [
                0,    // discriminant
                0x7f, // any
                3,    // idx
            ],
        ),
        outgoing_binding_expression_ok_1(
            |m, i, b| obe(OutgoingBindingExpressionUtf8Str {
                ty: WebidlScalarType::DomString.into(),
                offset: 0,
                length: 1,
            }),
            [
                1,    // discriminant
                0x71, // DOMString
                0,    // offset
                1,    // length
            ],
        ),
        outgoing_binding_expression_ok_2(
            |m, i, b| obe(OutgoingBindingExpressionUtf8CStr {
                ty: WebidlScalarType::DomString.into(),
                offset: 0,
            }),
            [
                2,    // discriminant
                0x71, // DOMString
                0,    // offset
            ],
        ),
        outgoing_binding_expression_ok_3(
            |m, i, b| obe(OutgoingBindingExpressionI32ToEnum {
                ty: get_my_enum_id(b),
                idx: 4,
            }),
            [
                3, // discriminant
                1, // my_enum
                4, // idx
            ],
        ),
        outgoing_binding_expression_ok_4(
            |m, i, b| obe(OutgoingBindingExpressionView {
                ty: WebidlScalarType::Uint8Array.into(),
                offset: 4,
                length: 5,
            }),
            [
                4,    // discriminant
                0x67, // Uint8Array
                4,    // offset
                5,    // length
            ],
        ),
        outgoing_binding_expression_ok_5(
            |m, i, b| obe(OutgoingBindingExpressionCopy {
                ty: WebidlScalarType::Uint8Array.into(),
                offset: 8,
                length: 9,
            }),
            [
                5,    // discriminant
                0x67, // Uint8Array
                8,    // offset
                9,    // length
            ],
        ),
        outgoing_binding_expression_ok_6(
            |m, i, b| obe(OutgoingBindingExpressionDict {
                ty: get_my_dict_id(b),
                fields: vec![obe(OutgoingBindingExpressionAs {
                    ty: WebidlScalarType::Any.into(),
                    idx: 8,
                })],
            }),
            [
                6,    // discriminant
                0,    // my_dict
                1,    // number of fields
                0,    // discriminant
                0x7f, // any
                8,    // idx
            ],
        ),
        outgoing_binding_expression_ok_7(
            |m, i, b| obe(OutgoingBindingExpressionBindExport {
                ty: get_my_func_id(b),
                binding: get_my_export_binding(b),
                idx: 7,
            }),
            [
                7, // discriminant
                2, // my_func
                1, // my_export_binding
                7, // idx
            ],
        ),
    );
    assert_decode_err!(
        OutgoingBindingExpression,
        // With the discriminant, but missing various parts of the expression.
        outgoing_binding_expression_err_0([
            0, // discriminant
            0x7f, // any
               // no idx
        ]),
        outgoing_binding_expression_err_1([
            0, // discriminant
               // no ty
               // no idx
        ]),
        outgoing_binding_expression_err_2([
            1,    // discriminant
            0x71, // DOMString
            0,    // offset
                  // no length
        ]),
        outgoing_binding_expression_err_3([
            1, // discriminant
            0x71, // DOMString
               // no offset
               // no length
        ]),
        outgoing_binding_expression_err_4([
            1, // discriminant
               // no ty
               // no offset
               // no length
        ]),
        outgoing_binding_expression_err_5([
            2, // discriminant
            0x71, // DOMString
               // no offset
        ]),
        outgoing_binding_expression_err_6([
            2, // discriminant
               // no ty
               // no offset
        ]),
        outgoing_binding_expression_err_7([
            3, // discriminant
            1, // my_enum
               // no idx
        ]),
        outgoing_binding_expression_err_8([
            3, // discriminant
               // no ty
               // no idx
        ]),
        outgoing_binding_expression_err_9([
            4,    // discriminant
            0x67, // Uint8Array
            4,    // offset
                  // no length
        ]),
        outgoing_binding_expression_err_10([
            4, // discriminant
            0x67, // Uint8Array
               // no offset
               // no length
        ]),
        outgoing_binding_expression_err_11([
            4, // discriminant
               // no ty
               // no offset
               // no length
        ]),
        outgoing_binding_expression_err_12([
            5,    // discriminant
            0x67, // Uint8Array
            8,    // offset
                  // no length
        ]),
        outgoing_binding_expression_err_13([
            5, // discriminant
            0x67, // Uint8Array
               // no offset
               // no length
        ]),
        outgoing_binding_expression_err_14([
            5, // discriminant
               // no ty
               // no offset
               // no length
        ]),
        outgoing_binding_expression_err_15([
            6, // discriminant
            0, // my_dict
            1, // number of fields
            0, // discriminant
            0x7f, // any
               // no idx
        ]),
        outgoing_binding_expression_err_16([
            6, // discriminant
            0, // my_dict
            1, // number of fields
               // no fields
        ]),
        outgoing_binding_expression_err_17([
            6, // discriminant
            0, // my_dict
               // no number of fields
        ]),
        outgoing_binding_expression_err_18([
            6, // discriminant
               // no dict type
               // no number of fields
        ]),
        outgoing_binding_expression_err_19([
            7, // discriminant
            2, // my_func
            1, // my_export_binding
               // no idx
        ]),
        outgoing_binding_expression_err_20([
            7, // discriminant
            2, // my_func
               // no export binding
               // no idx
        ]),
        outgoing_binding_expression_err_21([
            7, // discriminant
               // no func ty
               // no export binding
               // no idx
        ]),
        // Empty input stream.
        outgoing_binding_expression_err_22([]),
    );

    // Id<FunctionBinding>
    assert_decode_ok!(
        Id<FunctionBinding>,
        function_binding_id_ok_0(
            |m, i, b| get_my_import_binding(b),
            [
                0, // my_import_binding
            ],
        ),
    );
    assert_decode_err!(
        Id<FunctionBinding>,
        // Empty input stream.
        function_binding_id_err_0([]),
        // Index that doesn't correspond to a function binding.
        function_binding_id_err_1([9]),
    );

    // IncomingBindingMap
    assert_decode_ok!(
        IncomingBindingMap,
        incoming_binding_map_ok_0(
            |m, i, b| IncomingBindingMap { bindings: vec![] },
            [
                0, // number of bindings
            ],
        ),
        incoming_binding_map_ok_1(
            |m, i, b| IncomingBindingMap {
                bindings: vec![
                    Get(IncomingBindingExpressionGet { idx: 0 }),
                    Get(IncomingBindingExpressionGet { idx: 1 }),
                    Get(IncomingBindingExpressionGet { idx: 2 }),
                ]
            },
            [
                3, // number of bindings
                0, // discriminant
                0, // idx
                0, // discriminant
                1, // idx
                0, // discriminant
                2, // idx
            ],
        ),
    );
    assert_decode_err!(
        IncomingBindingMap,
        incoming_binding_map_err_0([]),
        incoming_binding_map_err_1([
            3, // number of bindings
            0, // discriminant
            0, // idx
            0, // discriminant
            1, // idx
               // no third binding expression
        ]),
    );

    // IncomingBindingExpression
    use IncomingBindingExpression::*;
    assert_decode_ok!(
        IncomingBindingExpression,
        incoming_bind_expression_ok_0(
            |m, i, b| Get(IncomingBindingExpressionGet { idx: 9 }),
            [
                0, // discriminant
                9, // idx
            ],
        ),
        incoming_bind_expression_ok_1(
            |m, i, b| As(IncomingBindingExpressionAs {
                ty: walrus::ValType::Anyref,
                expr: Box::new(Get(IncomingBindingExpressionGet { idx: 3 })),
            }),
            [
                1,    // discriminant
                0x6f, // anyref
                0,    // discriminant
                3,    // idx
            ],
        ),
        incoming_bind_expression_ok_2(
            |m, i, b| AllocUtf8Str(IncomingBindingExpressionAllocUtf8Str {
                alloc_func_name: "malloc".into(),
                expr: Box::new(Get(IncomingBindingExpressionGet { idx: 0 })),
            }),
            [
                2, // discriminant
                6, // length
                b'm', b'a', b'l', b'l', b'o', b'c', // "malloc"
                0,    // discriminant
                0,    // idx
            ],
        ),
        incoming_bind_expression_ok_3(
            |m, i, b| AllocCopy(IncomingBindingExpressionAllocCopy {
                alloc_func_name: "malloc".into(),
                expr: Box::new(Get(IncomingBindingExpressionGet { idx: 0 })),
            }),
            [
                3, // discriminant
                6, // length
                b'm', b'a', b'l', b'l', b'o', b'c', // "malloc"
                0,    // discriminant
                0,    // idx
            ],
        ),
        incoming_bind_expression_ok_4(
            |m, i, b| EnumToI32(IncomingBindingExpressionEnumToI32 {
                ty: get_my_enum_id(b),
                expr: Box::new(Get(IncomingBindingExpressionGet { idx: 0 })),
            }),
            [
                4, // discriminant
                1, // my_enum
                0, // discriminant
                0, // idx
            ],
        ),
        incoming_bind_expression_ok_5(
            |m, i, b| Field(IncomingBindingExpressionField {
                idx: 7,
                expr: Box::new(Get(IncomingBindingExpressionGet { idx: 0 })),
            }),
            [
                5, // discriminant
                7, // idx
                0, // discriminant
                0, // idx
            ],
        ),
        incoming_bind_expression_ok_6(
            |m, i, b| BindImport(IncomingBindingExpressionBindImport {
                ty: get_type_id(m),
                binding: get_my_import_binding(b),
                expr: Box::new(Get(IncomingBindingExpressionGet { idx: 0 })),
            }),
            [
                6, // discriminant
                0, // my_type_id
                0, // my_import_binding
                0, // discriminant
                0, // idx
            ],
        ),
    );
    assert_decode_err!(
        IncomingBindingExpression,
        // Various discriminants without the rest of the encoded expression.
        incoming_bind_expression_err_0([0]),
        incoming_bind_expression_err_1([1]),
        incoming_bind_expression_err_2([2]),
        incoming_bind_expression_err_3([3]),
        incoming_bind_expression_err_4([4]),
        incoming_bind_expression_err_5([5]),
        incoming_bind_expression_err_6([
            6, // discriminant
            0, // my_type_id
               // no import binding
        ]),
        incoming_bind_expression_err_7([
            6, // discriminant
               // no my_type_id
               // no import binding
        ]),
        // Empty input stream.
        incoming_bind_expression_err_8([]),
    );

    // Bind
    assert_decode_ok!(
        Bind,
        bind_ok_0(
            |m, i, b| { b.binds.arena.iter().last().unwrap().0.clone() },
            [
                0, // FunctionId
                0, // Id<FunctionBinding>
            ],
        ),
    );
    assert_decode_err!(
        Bind,
        // Empty input stream.
        bind_err_0([]),
        // Not enough input.
        bind_err_1([0]),
    );

    // walrus::FunctionId
    assert_decode_ok!(
        walrus::FunctionId,
        function_id_ok_0(|m, i, b| { m.funcs.iter().nth(0).unwrap().id() }, [0]),
    );
    assert_decode_err!(
        walrus::FunctionId,
        // Empty input stream.
        function_id_err_0([]),
        // Function index that doesn't exist in the Wasm module.
        function_id_err_1([9]),
    );
}
