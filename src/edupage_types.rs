use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    Student,
    Parent,
    Teacher,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    LoginFailed,
    NotLoggedIn,
    NoResponse,
    ParseError,
    NotFound,
}