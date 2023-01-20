use serde::{Deserialize, Serialize};
use crate::edupage_types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    ringing: Vec<Ringing>,
    userdata: User,
    dbi: DBI,
    nameday_today: String,
    nameday_tomorrow: String,
    day_plans: Vec<DayPlan>,
    year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: i32,
    #[serde(rename = "TriedaID")]
    class_id: i32,
    user_type: AccountType,
    #[serde(rename = "p_meno")]
    first_name: String,
    #[serde(rename = "p_priezvisko")]
    last_name: String,
    #[serde(rename = "p_mail")]
    mail: String,
    gender: Gender,
    #[serde(rename = "p_www_login")]
    login: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBI {
    classes: Vec<Class>,
    classrooms: Vec<Classroom>,
    parents: Vec<Parent>,
    plans: Vec<Plan>,
    students: Vec<Student>,
    subjects: Vec<Subject>,
    teachers: Vec<Teacher>,
    #[serde(rename = "homeworksEnabled")]
    homeworks_enabled: bool,
    #[serde(rename = "jeZUS")]
    art_school: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    #[serde(rename = "classroomid")]
    classroom_id: i32,
    grade: i32,
    id: i32,
    name: String,
    #[serde(rename = "short")]
    name_short: String,
    #[serde(rename = "teacherid")]
    teacher_id: i32,
    #[serde(rename = "teacher2id")]
    teacher2_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classroom {
    id: i32,
    name: String,
    #[serde(rename = "short")]
    name_short: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parent {
    #[serde(rename = "firstname")]
    first_name: String,
    #[serde(rename = "lastname")]
    last_name: String,
    gender: Gender,
    id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    #[serde(rename = "planid")]
    plan_id: i32,
    #[serde(rename = "nazovPlanu")]
    name: String,
    #[serde(rename = "predmetid")]
    subject_id: i32,
    #[serde(rename = "ucitelids")]
    teachers: Vec<i32>,
    #[serde(rename = "triedy")]
    class_ids: Vec<i32>,
    students: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    #[serde(rename = "classid")]
    class_id: i32,
    #[serde(rename = "firstname")]
    first_name: String,
    #[serde(rename = "lastname")]
    last_name: String,
    id: i32,
    parents: Vec<i32>,
    gender: Gender,
    #[serde(rename = "datefrom")]
    since: String,
    #[serde(rename = "numberinclass")]
    class_position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    id: i32,
    name: String,
    #[serde(rename = "short")]
    name_short: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Teacher {
    #[serde(rename = "firstname")]
    first_name: String,
    #[serde(rename = "lastname")]
    last_name: String,
    gender: Gender,
    id: i32,
    #[serde(rename = "short")]
    short_name: String,
    #[serde(rename = "datefrom")]
    since: String,
    #[serde(rename = "classroomid")]
    classroom_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayPlan {
    date: String,
    //plans: Vec<Plan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ringing {
    id: i32,
    start: String,
    end: String,
}