use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Network(String),
    Unexpected(String),
    Invalid(String),
    Other(String),
}
