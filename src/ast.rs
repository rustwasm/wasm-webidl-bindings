use crate::actions::Actions;

#[derive(Debug, Default)]
pub struct BuildAstActions;

impl Actions for BuildAstActions {
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
}

#[derive(Debug, PartialEq, Eq)]
pub enum OutgoingBindingExpression {
    As(OutgoingBindingExpressionAs),
    Utf8Str(OutgoingBindingExpressionUtf8Str),
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

#[derive(Debug, PartialEq, Eq)]
pub struct OutgoingBindingExpressionAs {
    pub ty: WebidlTypeRef,
    pub idx: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct OutgoingBindingExpressionUtf8Str {
    pub ty: WebidlTypeRef,
    pub offset: u32,
    pub length: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WebidlTypeRef {
    Named(WebidlTypeRefNamed),
    Indexed(WebidlTypeRefIndexed),
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

#[derive(Debug, PartialEq, Eq)]
pub struct WebidlTypeRefNamed {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WebidlTypeRefIndexed {
    pub idx: u32,
}