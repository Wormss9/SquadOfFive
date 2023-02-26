use http::{StatusCode,HeaderMap};

#[derive(Debug)]
pub struct Error {
    code: StatusCode,
    message: Option<String>,
}

impl Error {
    pub fn code(code: StatusCode) -> Self {
        Self {
            code,
            message: None,
        }
    }
    pub fn code_fn<T: std::fmt::Debug>(code: StatusCode) -> impl Fn(T) -> Self {
        move |x| -> Self {
            Self::code(code)
        }
    }
}
