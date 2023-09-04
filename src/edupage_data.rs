use crate::edupage_types::*;

#[derive(Debug, Clone)]
pub struct Data {
    pub ringing: Vec<Ringing>,
    pub user: User,
    pub dbi: DBI,
    pub nameday_today: String,
    pub nameday_tomorrow: String,
    pub day_plans: Vec<DayPlan>,
    pub year: i32,
    pub timeline: Vec<TimelineEvent>,
}

impl Data {
    pub fn day_plan_from_date(&self, date: String) -> Option<DayPlan> {
        self.day_plans
            .iter()
            .find(|d| d.date == date)
            .cloned()
    }
    pub fn timeline_from_id(&self, id: i32) -> Option<TimelineEvent> {
        self.timeline.iter().find(|t| t.id == id).cloned()
    }
    pub fn ringing_from_id(&self, id: i32) -> Option<Ringing> {
        self.ringing.iter().find(|r| r.id == id).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub user: AccountType,
    pub class_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub mail: String,
    pub gender: Gender,
    pub login: String,
}

impl User {
    pub fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[derive(Debug, Clone)]
pub struct DBI {
    pub classes: Vec<Class>,
    pub classrooms: Vec<Classroom>,
    pub parents: Vec<Parent>,
    pub plans: Vec<Plan>,
    pub students: Vec<Student>,
    pub subjects: Vec<Subject>,
    pub teachers: Vec<Teacher>,
    pub homeworks_enabled: bool,
    pub art_school: bool,
}

impl DBI {
    pub fn class_from_id(&self, id: i32) -> Option<Class> {
        self.classes.iter().find(|c| c.id == id).cloned()
    }
    pub fn classroom_from_id(&self, id: i32) -> Option<Classroom> {
        self.classrooms.iter().find(|c| c.id == id).cloned()
    }
    pub fn parent_from_id(&self, id: i32) -> Option<Parent> {
        self.parents.iter().find(|p| p.id == id).cloned()
    }
    pub fn plan_from_id(&self, id: i32) -> Option<Plan> {
        self.plans.iter().find(|p| p.plan_id == id).cloned()
    }
    pub fn student_from_id(&self, id: i32) -> Option<Student> {
        self.students.iter().find(|s| s.id == id).cloned()
    }
    pub fn subject_from_id(&self, id: i32) -> Option<Subject> {
        self.subjects.iter().find(|s| s.id == id).cloned()
    }
    pub fn teacher_from_id(&self, id: i32) -> Option<Teacher> {
        self.teachers.iter().find(|t| t.id == id).cloned()
    }
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

impl Parent {
    pub fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
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

impl Student {
    pub fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[derive(Debug, Clone)]
pub struct Subject {
    pub id: i32,
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

impl Teacher {
    pub fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[derive(Debug, Clone)]
pub struct Ringing {
    pub id: i32,
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone)]
pub struct DayPlan {
    pub date: String,
    pub lessons: Vec<Lesson>,
}

#[derive(Debug, Clone)]
pub struct Lesson {
    pub subject_id: Option<i32>,
    pub plan_id: Option<i32>,
    pub period: i32,
}

#[derive(Debug, Clone)]
pub struct TimelineEvent {
    pub id: i32,
    pub added: String,
    pub time: Option<String>,
    pub author: AccountType,
    pub recipient: AccountType,
    pub text: String,
    pub data: serde_json::Value,
}