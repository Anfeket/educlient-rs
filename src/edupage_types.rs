#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone)]
pub enum AccountType {
    Student,
    Parent,
    Teacher,
}

#[derive(Debug, Clone)]
pub enum Error {
    LoginFailed,
    NotLoggedIn,
    NoResponse,
    ParseError,
    NotFound,
}
