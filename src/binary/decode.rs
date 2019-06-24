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
