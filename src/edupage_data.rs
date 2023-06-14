use std::collections::HashMap;
use crate::edupage_types::*;

#[derive(Debug, Clone)]
pub struct Data {
    pub ringing: Vec<Ringing>,
    pub user: User,
    pub dbi: DBI,
    pub nameday_today: String,
    pub nameday_tomorrow: String,
    pub day_plans: HashMap<String, Vec<Lesson>>,
    pub year: i32,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub class_id: Option<i32>,
    pub user_type: AccountType,
    pub first_name: String,
    pub last_name: String,
    pub mail: String,
    pub gender: Gender,
    pub login: String,
}

#[derive(Debug, Clone)]
pub struct DBI {
    pub classes: Vec<Class>,
    pub classrooms: Vec<Classroom>,
    pub parents: Vec<Parent>,
    pub plans: Vec<Plan>,
    pub students: Vec<Student>,
    pub subjects: HashMap<i32, Subject>,
    pub teachers: Vec<Teacher>,
    pub homeworks_enabled: bool,
    pub art_school: bool,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub classroom_id: Option<i32>,
    pub grade: i32,
    pub id: i32,
    pub name: String,
    pub name_short: String,
    pub teacher_id: Option<i32>,
    pub teacher2_id: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct Classroom {
    pub id: i32,
    pub name: String,
    pub name_short: String,
}

#[derive(Debug, Clone)]
pub struct Parent {
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
    pub id: i32,
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub plan_id: i32,
    pub name: String,
    pub subject_id: i32,
    pub teachers: Vec<i32>,
    pub class_ids: Vec<i32>,
    pub students: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct Student {
    pub class_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub id: i32,
    pub parents: Vec<i32>,
    pub gender: Gender,
    pub since: String,
    pub class_position: i32,
}

#[derive(Debug, Clone)]
pub struct Subject {
    pub name: String,
    pub name_short: String,
}

#[derive(Debug, Clone)]
pub struct Teacher {
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
    pub id: i32,
    pub short_name: String,
    pub since: String,
    pub classroom_id: Option<i32>,
}


#[derive(Debug, Clone)]
pub struct Ringing {
    pub id: i32,
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone)]
pub struct Lesson {
    pub subject_id: Option<i32>,
    pub plan_id: Option<i32>,
    pub period: i32,
}
