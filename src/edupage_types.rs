use std::fmt::Display;

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
    Other(String),
}

impl Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Student(id) => write!(f, "Student {}", id),
            AccountType::Parent(id) => write!(f, "Parent {}", id),
            AccountType::Teacher(id) => write!(f, "Teacher {}", id),
            AccountType::Admin => write!(f, "Admin"),
            AccountType::Other(s) => write!(f, "{}", s),
        }
    }
}


#[derive(Debug, Clone)]
pub enum Error {
    LoginFailed,
    NotLoggedIn,
    NoResponse,
    ParseError,
    NotFound,
}
