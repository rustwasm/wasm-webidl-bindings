pub(crate) fn error<L, T, S>(error: S) -> lalrpop_util::ParseError<L, T, String>
where
    S: Into<String>,
{
    let error = error.into();
    lalrpop_util::ParseError::User { error }
}
