#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone)]
pub enum AccountType {
    Student(i32),
    Parent(i32),
    Teacher(i32),
    Admin,
}

#[derive(Debug, Clone)]
pub enum Error {
    LoginFailed,
    NotLoggedIn,
    NoResponse,
    ParseError,
    NotFound,
}
