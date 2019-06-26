use crate::ast::*;
use quickcheck::{Arbitrary, Gen};
use rand::{seq::IteratorRandom, Rng};

impl Arbitrary for WebidlBindings {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let mut wb = WebidlBindings::default();
        wb.arbitrary_types(g);
        wb.arbitrary_function_bindings(g);
        wb.arbitrary_binds(g);
        wb
    }
}

impl WebidlBindings {
    fn arbitrary_types(&mut self, g: &mut impl Gen) {
        for _ in 0..g.size() {
            match g.gen_range(0, 4) {
                0 => {
                    // Function.
                    self.arbitrary_webidl_function(g);
                }
                1 => {
                    // Dictionary.
                    self.arbitrary_webidl_dictionary(g);
                }
                2 => {
                    // Enumeration.
                    self.arbitrary_webidl_enumeration(g);
                }
                3 => {
                    // Union.
                    self.arbitrary_webidl_union(g);
                }
                _ => unreachable!(),
            }
        }
    }

    fn arbitrary_webidl_function(&mut self, g: &mut impl Gen) {
        let kind = match g.gen_range(0, 3) {
            0 => WebidlFunctionKind::Static,
            1 => WebidlFunctionKind::Method(WebidlFunctionKindMethod {
                ty: WebidlScalarType::Any.into(),
            }),
            2 => WebidlFunctionKind::Constructor,
            _ => unreachable!(),
        };

        let params: Vec<_> = (0..g.size())
            .map(|_| self.arbitrary_webidl_type_ref(g))
            .collect();

        let result = if g.gen() {
            Some(self.arbitrary_webidl_type_ref(g))
        } else {
            None
        };

        self.types.insert(WebidlFunction {
            kind,
            params,
            result,
        });
    }

    fn arbitrary_webidl_type_ref(&mut self, g: &mut impl Gen) -> WebidlTypeRef {
        if self.types.arena.len() == 0 || g.gen() {
            // Scalar type.
            match g.gen_range(0, 30) {
                0 => WebidlScalarType::Any.into(),
                1 => WebidlScalarType::Boolean.into(),
                2 => WebidlScalarType::Byte.into(),
                3 => WebidlScalarType::Octet.into(),
                4 => WebidlScalarType::Long.into(),
                5 => WebidlScalarType::UnsignedLong.into(),
                6 => WebidlScalarType::Short.into(),
                7 => WebidlScalarType::UnsignedShort.into(),
                8 => WebidlScalarType::LongLong.into(),
                9 => WebidlScalarType::UnsignedLongLong.into(),
                10 => WebidlScalarType::Float.into(),
                11 => WebidlScalarType::UnrestrictedFloat.into(),
                12 => WebidlScalarType::Double.into(),
                13 => WebidlScalarType::UnrestrictedDouble.into(),
                14 => WebidlScalarType::DomString.into(),
                15 => WebidlScalarType::ByteString.into(),
                16 => WebidlScalarType::UsvString.into(),
                17 => WebidlScalarType::Object.into(),
                18 => WebidlScalarType::Symbol.into(),
                19 => WebidlScalarType::ArrayBuffer.into(),
                20 => WebidlScalarType::DataView.into(),
                21 => WebidlScalarType::Int8Array.into(),
                22 => WebidlScalarType::Int16Array.into(),
                23 => WebidlScalarType::Int32Array.into(),
                24 => WebidlScalarType::Uint8Array.into(),
                25 => WebidlScalarType::Uint16Array.into(),
                26 => WebidlScalarType::Uint32Array.into(),
                27 => WebidlScalarType::Uint8ClampedArray.into(),
                28 => WebidlScalarType::Float32Array.into(),
                29 => WebidlScalarType::Float64Array.into(),
                _ => unreachable!(),
            }
        } else {
            // Reference to an existing compound type.
            self.types
                .arena
                .iter()
                .map(|(id, _)| id)
                .choose(g)
                .unwrap()
                .into()
        }
    }

    fn arbitrary_webidl_dictionary(&mut self, g: &mut impl Gen) {
        let fields: Vec<_> = (0..g.size())
            .map(|_| {
                let name = String::arbitrary(g);
                let ty = self.arbitrary_webidl_type_ref(g);
                WebidlDictionaryField { name, ty }
            })
            .collect();

        self.types.insert(WebidlDictionary { fields });
    }

    fn arbitrary_webidl_enumeration(&mut self, g: &mut impl Gen) {
        let values: Vec<_> = (0..g.size()).map(|_| String::arbitrary(g)).collect();
        self.types.insert(WebidlEnumeration { values });
    }

    fn arbitrary_webidl_union(&mut self, g: &mut impl Gen) {
        let members: Vec<_> = (0..g.size())
            .map(|_| self.arbitrary_webidl_type_ref(g))
            .collect();
        self.types.insert(WebidlUnion { members });
    }

    fn arbitrary_function_bindings(&mut self, _g: &mut impl Gen) {
        // TODO: we don't actually generate any of these because we need to get
        // `walrus::TypeId`s which means we need access to the `walrus::Module`.
    }

    fn arbitrary_binds(&mut self, _g: &mut impl Gen) {
        // TODO: same story! We don't actually generate any bind statements here
        // because we need access to a wasm module in order to get ahold of ids
        // pointing to its functions.
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::WebidlBindings;

    quickcheck::quickcheck! {
        fn can_encode_and_decode_arbitrary_webidl_bindings(section: WebidlBindings) -> () {
            let mut module = walrus::Module::default();
            module.customs.add(section);
            let buf = module.emit_wasm().expect("should emit wasm OK");

            let mut config = walrus::ModuleConfig::default();
            config.on_parse(|module, ids| {
                let raw = module.customs.remove_raw("webidl-bindings")
                    .expect("the webidl-bindings custom section should have been emitted");
                crate::binary::decode(ids, &raw.data)
                    .expect("should decode webidl-bindings section OK");
                Ok(())
            });
            config.parse(&buf).expect("should parse the wasm OK");
        }
    }
}
