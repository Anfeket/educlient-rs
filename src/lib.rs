pub mod edupage_data;
pub mod edupage_types;

use edupage_data::*;
use edupage_types::*;
use reqwest::blocking;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Educlient {
    pub logged_in: bool,
    pub domain: String,
    pub json: serde_json::Value,
    pub session: blocking::Client,
}

impl Educlient {
    pub fn new(domain: String) -> Educlient {
        let session = blocking::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        Educlient {
            logged_in: false,
            domain,
            json: serde_json::Value::Null,
            session,
        }
    }

    pub fn login(&mut self, username: String, password: String) -> Result<&Self, Error> {
        let url = format!("https://{}.edupage.org/login/edubarlogin.php", self.domain);
        let params = [("username", username), ("password", password)];
        let res = self.session.post(url).form(&params).send().unwrap();
        if res.url().as_str().contains("bad=1") {
            self.logged_in = false;
            return Err(Error::LoginFailed);
        }
        let data = res.text().unwrap().replace(['\t', '\r', '\n'], "");
        let data = data.split("userhome(").collect::<Vec<_>>();
        let data = data[1].split(");").collect::<Vec<_>>();
        let data = data[0].to_string();
        let data: Value = serde_json::from_str(&data).unwrap();
        if data.is_null() {
            self.logged_in = false;
            return Err(Error::NoResponse);
        }
        self.logged_in = true;
        self.json = data;
        Ok(self)
    }

    pub fn get_grades(&self) -> Result<serde_json::Value, Error> {
        if !self.logged_in {
            return Err(Error::NotLoggedIn);
        }
        let url = format!("https://{}.edupage.org/znamky/?", self.domain);
        let res = self.session.get(url).send();
        if res.is_err() {
            return Err(Error::NoResponse);
        }
        let res = res.unwrap().text().unwrap();
        let to_parse = res.split(".znamkyStudentViewer(").collect::<Vec<_>>()[1]
            .split(");\r\n\t\t});\r\n\t\t</script>")
            .collect::<Vec<_>>()[0];
        let json = serde_json::from_str::<Value>(to_parse);
        if json.is_err() {
            return Err(Error::ParseError);
        }
        Ok(json.unwrap())
    }

    pub fn data(&self) -> Result<Data, Error> {
        if self.json.is_null() {
            if self.logged_in {
                return Err(Error::NotFound);
            } else {
                return Err(Error::NotLoggedIn);
            }
        }

        if cfg!(debug_assertions) {
            println!("Deserializing userdata");
        }
        let user = if let Some(id) = self.json["userrow"]["UcitelID"].as_str() {
            let id = id.parse::<i32>().unwrap();
            AccountType::Teacher(id)
        } else if let Some(id) = self.json["userrow"]["StudentID"].as_str() {
            let id = id.parse::<i32>().unwrap();
            AccountType::Student(id)
        } else if let Some(id) = self.json["userrow"]["RodicID"].as_str() {
            let id = id.parse::<i32>().unwrap();
            AccountType::Parent(id)
        } else {
            return Err(Error::ParseError);
        };
        let class_id = self.json["userrow"]["TriedaID"]
            .as_str()
            .map(|id| id.parse::<i32>().unwrap());
        let first_name = self.json["userrow"]["p_meno"].as_str().unwrap().to_string();
        let last_name = self.json["userrow"]["p_priezvisko"]
            .as_str()
            .unwrap()
            .to_string();
        let mail = self.json["userrow"]["p_mail"].as_str().unwrap().to_string();
        let gender = match self.json["userrow"]["p_pohlavie"].as_str().unwrap() {
            "1" => Gender::Male,
            "2" => Gender::Female,
            _ => return Err(Error::ParseError),
        };
        let login = self.json["userrow"]["p_www_login"]
            .as_str()
            .unwrap()
            .to_string();
        let userdata = User {
            user,
            class_id,
            first_name,
            last_name,
            mail,
            gender,
            login,
        };

        if cfg!(debug_assertions) {
            println!("Deserializing ringing");
        }
        let mut ringing: Vec<Ringing> = Vec::new();
        for ring in self.json["zvonenia"].as_array().unwrap() {
            ringing.push(Ringing {
                id: ring["id"].as_str().unwrap().parse::<i32>().unwrap(),
                start: ring["starttime"].as_str().unwrap().to_string(),
                end: ring["endtime"].as_str().unwrap().to_string(),
            })
        }

        if cfg!(debug_assertions) {
            println!("Deserializing year");
        }
        let year = self.json["dp"]["year"].as_i64().unwrap() as i32;

        if cfg!(debug_assertions) {
            println!("Deserializing namedays");
        }
        let nameday_today = self.json["meninyDnes"].as_str().unwrap().to_string();
        let nameday_tomorrow = self.json["meninyZajtra"].as_str().unwrap().to_string();

        if cfg!(debug_assertions) {
            println!("Deserializing dbi");
        }
        if cfg!(debug_assertions) {
            println!("Deserializing dbi/classes");
        }
        let mut classes: Vec<Class> = Vec::new();
        for class in self.json["dbi"]["classes"].as_object().unwrap().values() {
            let teacher2_id = if let Some(id) = class["teacher2id"].as_str() {
                if !id.is_empty() {
                    Some(id.parse::<i32>().unwrap())
                } else {
                    None
                }
            } else {
                None
            };
            let classroom_id = if let Some(id) = class["classroomid"].as_str() {
                if !id.is_empty() {
                    Some(id.parse::<i32>().unwrap())
                } else {
                    None
                }
            } else {
                None
            };
            classes.push(Class {
                classroom_id,
                grade: class["grade"].as_str().unwrap().parse::<i32>().unwrap(),
                id: class["id"].as_str().unwrap().parse::<i32>().unwrap(),
                name: class["name"].as_str().unwrap().to_string(),
                name_short: class["short"].as_str().unwrap().to_string(),
                teacher_id: Some(class["teacherid"].as_str().unwrap().parse::<i32>().unwrap()),
                teacher2_id,
            })
        }

        if cfg!(debug_assertions) {
            println!("Deserializing dbi/classrooms");
        }
        let mut classrooms: Vec<Classroom> = Vec::new();
        for classroom in self.json["dbi"]["classrooms"].as_object().unwrap().values() {
            classrooms.push(Classroom {
                id: classroom["id"].as_str().unwrap().parse::<i32>().unwrap(),
                name: classroom["name"].as_str().unwrap().to_string(),
                name_short: classroom["short"].as_str().unwrap().to_string(),
            })
        }

        if cfg!(debug_assertions) {
            println!("Deserializing dbi/parents");
        }
        let mut parents: Vec<Parent> = Vec::new();
        for parent in self.json["dbi"]["parents"].as_object().unwrap().values() {
            parents.push(Parent {
                first_name: parent["firstname"].as_str().unwrap().to_string(),
                last_name: parent["lastname"].as_str().unwrap().to_string(),
                gender: match parent["gender"].as_str().unwrap() {
                    "M" => Gender::Male,
                    "F" => Gender::Female,
                    _ => return Err(Error::ParseError),
                },
                id: parent["id"].as_str().unwrap().parse::<i32>().unwrap(),
            })
        }

        if cfg!(debug_assertions) {
            println!("Deserializing dbi/plans");
        }
        let mut plans: Vec<Plan> = Vec::new();
        if self.json["dbi"]["plans"].is_object() {
            for plan in self.json["dbi"]["plans"].as_object().unwrap().values() {
                let mut class_ids: Vec<i32> = Vec::new();
                for class in plan["triedy"].as_array().unwrap() {
                    if class.is_string() {
                        class_ids.push(class.as_str().unwrap().parse::<i32>().unwrap());
                        continue;
                    }
                    if class.is_i64() {
                        class_ids.push(class.as_i64().unwrap() as i32);
                        continue;
                    }
                    return Err(Error::ParseError);
                }
                let mut students: Vec<i32> = Vec::new();
                for student in plan["students"].as_array().unwrap() {
                    if student.is_string() {
                        students.push(student.as_str().unwrap().parse::<i32>().unwrap());
                        continue;
                    }
                    if student.is_i64() {
                        students.push(student.as_i64().unwrap() as i32);
                        continue;
                    }
                    if student.is_null() {
                        break;
                    }
                    return Err(Error::ParseError);
                }
                plans.push(Plan {
                    plan_id: plan["planid"].as_str().unwrap().parse::<i32>().unwrap(),
                    name: plan["predmetMeno"].as_str().unwrap().to_string(),
                    subject_id: plan["predmetid"].as_str().unwrap().parse::<i32>().unwrap(),
                    teachers: plan["ucitelids"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|x| x.as_str().unwrap().parse::<i32>().unwrap())
                        .collect(),
                    class_ids,
                    students,
                })
            }
        }

        if cfg!(debug_assertions) {
            println!("Deserializing dbi/students");
        }
        let mut students: Vec<Student> = Vec::new();
        for student in self.json["dbi"]["students"].as_object().unwrap().values() {
            let mut parents = Vec::new();
            for i in ["parent1id", "parent2id", "parent3id"].iter() {
                if let Some(id) = student[i].as_str() {
                    if !id.is_empty() {
                        parents.push(id.parse::<i32>().unwrap());
                    }
                }
            }
            students.push(Student {
                class_id: student["classid"].as_str().unwrap().parse::<i32>().unwrap(),
                first_name: student["firstname"].as_str().unwrap().to_string(),
                last_name: student["lastname"].as_str().unwrap().to_string(),
                id: student["id"].as_str().unwrap().parse::<i32>().unwrap(),
                parents,
                gender: match student["gender"].as_str().unwrap() {
                    "M" => Gender::Male,
                    "F" => Gender::Female,
                    _ => return Err(Error::ParseError),
                },
                since: student["datefrom"].as_str().unwrap().to_string(),
                class_position: student["numberinclass"]
                    .as_str()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
            })
        }

        if cfg!(debug_assertions) {
            println!("Deserializing dbi/subjects");
        }
        let mut subjects: Vec<Subject> = Vec::new();
        for subject in self.json["dbi"]["subjects"].as_object().unwrap().values() {
            subjects.push(
                Subject {
                    id: subject["id"].as_str().unwrap().parse::<i32>().unwrap(),
                    name: subject["name"].as_str().unwrap().to_string(),
                    name_short: subject["short"].as_str().unwrap().to_string(),
                },
            );
        }

        if cfg!(debug_assertions) {
            println!("Deserializing dbi/teachers");
        }
        let mut teachers: Vec<Teacher> = Vec::new();
        for teacher in self.json["dbi"]["teachers"].as_object().unwrap().values() {
            let classroom_id = if let Some(id) = teacher["classroomid"].as_str() {
                if !id.is_empty() {
                    Some(id.parse::<i32>().unwrap())
                } else {
                    None
                }
            } else {
                None
            };
            teachers.push(Teacher {
                first_name: teacher["firstname"].as_str().unwrap().to_string(),
                last_name: teacher["lastname"].as_str().unwrap().to_string(),
                gender: match teacher["gender"].as_str().unwrap() {
                    "M" => Gender::Male,
                    "F" => Gender::Female,
                    _ => return Err(Error::ParseError),
                },
                id: teacher["id"].as_str().unwrap().parse::<i32>().unwrap(),
                short_name: teacher["short"].as_str().unwrap().to_string(),
                since: teacher["datefrom"].as_str().unwrap().to_string(),
                classroom_id,
            })
        }

        if cfg!(debug_assertions) {
            println!("Deserializing dbi/homeworks_enabled");
        }
        let homeworks_enabled = self.json["dbi"]["homeworksEnabled"].as_bool().unwrap();

        if cfg!(debug_assertions) {
            println!("Deserializing dbi/art_school");
        }
        let art_school = self.json["dbi"]["jeZUS"].as_bool().unwrap();

        if cfg!(debug_assertions) {
            println!("Deserializing dbi/dayplan");
            println!("Deserializing dbi/day_plans/lessons");
        }
        let mut day_plans: Vec<DayPlan> = Vec::new();
        for date in self.json["dp"]["dates"].as_object().unwrap() {
            let mut lessons: Vec<Lesson> = Vec::new();
            for plan in date.1["plan"].as_array().unwrap() {
                if plan["periodorbreak"].as_str().unwrap() == "ZZZ" {
                    continue;
                }
                let subject_id = if let Some(id) = plan["subjectid"].as_str() {
                    if !id.is_empty() {
                        Some(id.parse::<i32>().unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                };
                let plan_id = if let Some(id) = plan["groupsubjectids"].as_array().unwrap().first()
                {
                    if id.is_null() {
                        None
                    } else {
                        Some(id.as_str().unwrap().parse::<i32>().unwrap())
                    }
                } else {
                    None
                };
                if plan["period"].as_str().is_none() {
                    continue;
                }
                let period = plan["period"].as_str().unwrap().parse::<i32>().unwrap();
                let lesson: Lesson = Lesson {
                    subject_id,
                    plan_id,
                    period,
                };
                lessons.push(lesson)
            }

            day_plans.push(DayPlan {
                date: date.0.to_string(),
                lessons,
            })
        }

        let dbi = DBI {
            students,
            teachers,
            subjects,
            plans,
            homeworks_enabled,
            art_school,
            classes,
            classrooms,
            parents,
        };

        if cfg!(debug_assertions) {
            println!("Deserializing timeline");
        }
        let mut timeline = Vec::<TimelineEvent>::new();
        for event in self.json["items"].as_array().unwrap() {
            let time = if event["cas_udalosti"].is_null() {
                None
            } else {
                Some(event["cas_udalosti"].as_str().unwrap().to_string())
            };
            let id = event["timelineid"]
                .as_str()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let added = event["cas_pridania"].as_str().unwrap().to_string();
            let author= Self::account_from_string(event["vlastnik"].as_str().unwrap().to_string());
            let recipient = Self::account_from_string(event["user"].as_str().unwrap().to_string());
            let text = event["text"].as_str().unwrap().to_string();
            let data = event["data"].clone();
            let timeline_event = TimelineEvent {
                id,
                added,
                time,
                author,
                recipient,
                text,
                data,
            };
            timeline.push(timeline_event)
        }

        if cfg!(debug_assertions) {
            println!("Finished deserializing");
        }
        
        Ok(Data {
            ringing,
            user: userdata,
            dbi,
            nameday_today,
            nameday_tomorrow,
            day_plans,
            year,
            timeline,
        })
    }

    pub fn account_from_string(name: String) -> AccountType {
        println!("{}", name);
        if name.starts_with("Student") {
            let id = name[8..].parse::<i32>();
            match id {
                Ok(id) => return AccountType::Student(id),
                Err(_) => return AccountType::Other(name),
            }
        }
        if name.starts_with("Rodic-") {
            let id = name[6..].parse::<i32>();
            match id {
                Ok(id) => return AccountType::Parent(id),
                Err(_) => return AccountType::Other(name),
            }
        }
        if name.starts_with("Ucitel") {
            let id = name[7..].parse::<i32>();
            match id {
                Ok(id) => return AccountType::Teacher(id),
                Err(_) => return AccountType::Other(name),
            }
        }
        if name.starts_with("Admin") {
            return AccountType::Admin;
        }
        AccountType::Other(name)
    }
}
