pub mod edupage_data;
pub mod edupage_types;

use edupage_types::*;
use reqwest::blocking;
use serde_json::Value;

#[derive(Debug)]
pub struct Educlient {
    pub logged_in: bool,
    pub domain: String,
    pub data: serde_json::Value,
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
            data: serde_json::Value::Null,
            session,
        }
    }

    pub fn login(&mut self, username: String, password: String) -> Result<&Educlient, Error> {
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
        self.data = data;
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

    pub fn deserialize(&self) -> Result<edupage_data::Data, Error> {
        if !self.logged_in {
            return Err(Error::NotLoggedIn);
        }
        let mut data = self.data.clone();

        data["year"] = data["dp"]["year"].clone();

        let user = match data["userid"]
            .as_str()
            .unwrap()
            .chars()
            .next()
            .unwrap()
            .to_string()
            .as_str()
        {
            "S" => AccountType::Student,
            "R" => AccountType::Parent,
            "U" => AccountType::Teacher,
            _ => return Err(Error::ParseError),
        };
        data["userrow"]["user_type"] = serde_json::Value::from(match user {
            AccountType::Student => "Student",
            AccountType::Parent => "Parent",
            AccountType::Teacher => "Teacher",
        });

        for i in data["zvonenia"].as_array_mut().unwrap() {
            i["start"] = i["starttime"].clone();
            i["end"] = i["endtime"].clone();
            i["id"] = serde_json::Value::from(
                i["id"]
                    .as_str()
                    .unwrap()
                    .replace("zvonenie", "")
                    .parse::<i32>()
                    .unwrap(),
            );
        }
        data["ringing"] = data["zvonenia"].clone();

        let id = match user {
            AccountType::Student => data["userid"].as_str().unwrap().replace("Student", ""),
            AccountType::Parent => data["userid"].as_str().unwrap().replace("Rodic", ""),
            AccountType::Teacher => data["userid"].as_str().unwrap().replace("Ucitel", ""),
        };
        let id = id.parse::<i32>().unwrap();
        data["userrow"]["id"] = serde_json::Value::from(id);
        data["userrow"]["gender"] = match data["userrow"]["p_pohlavie"].as_str().unwrap() {
            "1" => serde_json::Value::from("Male"),
            "2" => serde_json::Value::from("Female"),
            _ => return Err(Error::ParseError),
        };

        if !data["userrow"]["TriedaID"].is_null() {
            data["userrow"]["TriedaID"] = serde_json::Value::Number(
                data["userrow"]["TriedaID"]
                    .as_str()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()
                    .into(),
            );
        } else {
            data["userrow"]["TriedaID"] = serde_json::Value::Number(0.into());
        }

        for i in data["dbi"]["parents"].as_object_mut().unwrap().values_mut() {
            i["gender"] = match i["gender"].as_str().unwrap() {
                "M" => serde_json::Value::from("Male"),
                "F" => serde_json::Value::from("Female"),
                _ => todo!(),
            };
            i["first_name"] = i["firstname"].clone();
            i["last_name"] = i["lastname"].clone();
            i["id"] =
                serde_json::Value::Number(i["id"].as_str().unwrap().parse::<i64>().unwrap().into());
        }

        for i in data["dbi"]["students"]
            .as_object_mut()
            .unwrap()
            .values_mut()
        {
            let mut parents: Vec<i64> = Vec::new();
            for y in 0..3 {
                let parent = format!("parent{}id", y + 1);
                if i[&parent].as_str().unwrap() == "" {
                    parents.push(0);
                    continue;
                }
                parents.push(i[&parent].as_str().unwrap().parse::<i64>().unwrap());
            }
            i["parents"] = serde_json::Value::Array(
                parents
                    .into_iter()
                    .map(|x| serde_json::Value::Number(x.into()))
                    .collect(),
            );

            i["class_id"] = serde_json::Value::Number(
                i["classid"]
                    .as_str()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()
                    .into(),
            );
            i["id"] =
                serde_json::Value::Number(i["id"].as_str().unwrap().parse::<i64>().unwrap().into());
            i["class_position"] = serde_json::Value::Number(
                i["numberinclass"]
                    .as_str()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()
                    .into(),
            );
            i["gender"] = match i["gender"].as_str().unwrap() {
                "M" => serde_json::Value::from("Male"),
                "F" => serde_json::Value::from("Female"),
                _ => return Err(Error::ParseError),
            };
            i["first_name"] = i["firstname"].clone();
            i["last_name"] = i["lastname"].clone();
            i["since"] = i["datefrom"].clone();
        }

        for i in data["dbi"]["subjects"]
            .as_object_mut()
            .unwrap()
            .values_mut()
        {
            i["id"] =
                serde_json::Value::Number(i["id"].as_str().unwrap().parse::<i64>().unwrap().into());
            i["name_short"] = i["short"].clone();
        }

        for i in data["dbi"]["teachers"]
            .as_object_mut()
            .unwrap()
            .values_mut()
        {
            i["first_name"] = i["firstname"].clone();
            i["last_name"] = i["lastname"].clone();
            i["short_name"] = i["short"].clone();
            i["since"] = i["datefrom"].clone();
            i["gender"] = match i["gender"].as_str().unwrap() {
                "M" => serde_json::Value::from("Male"),
                "F" => serde_json::Value::from("Female"),
                _ => todo!(),
            };
            i["id"] =
                serde_json::Value::Number(i["id"].as_str().unwrap().parse::<i64>().unwrap().into());
            if i["classroomid"] != "" {
                i["classroom_id"] = serde_json::Value::Number(
                    i["classroomid"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap()
                        .into(),
                );
            } else {
                i["classroom_id"] = serde_json::Value::Number(0.into());
            }
        }

        for i in data["dbi"]["plans"].as_object_mut().unwrap().values_mut() {
            let mut ids = Vec::new();
            for j in i["ucitelids"].as_array().unwrap() {
                ids.push(serde_json::Value::Number(
                    j.as_str().unwrap().parse::<i64>().unwrap().into(),
                ));
            }
            i["teachers"] = serde_json::Value::Array(ids);
            let mut ids = Vec::new();
            for j in i["students"].as_array().unwrap() {
                if j.is_i64() {
                    ids.push(j.clone());
                    continue;
                }
                ids.push(serde_json::Value::Number(
                    j.as_str().unwrap().parse::<i64>().unwrap().into(),
                ));
            }
            i["students"] = serde_json::Value::Array(ids);
            i["plan_id"] = serde_json::Value::Number(
                i["planid"].as_str().unwrap().parse::<i64>().unwrap().into(),
            );
            i["subject_id"] = serde_json::Value::Number(
                i["predmetid"]
                    .as_str()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()
                    .into(),
            );
            i["class_ids"] = serde_json::Value::Array(Vec::new());
            for y in i["triedy"].as_array().unwrap().clone() {
                if y.is_string() {
                    i["class_ids"]
                        .as_array_mut()
                        .unwrap()
                        .push(serde_json::Value::Number(
                            y.as_str().unwrap().parse::<i64>().unwrap().into(),
                        ));
                } else {
                    i["class_ids"].as_array_mut().unwrap().push(y.clone());
                }
            }
            i["name"] = i["nazovPlanu"].clone();
        }

        for i in data["dbi"]["classrooms"]
            .as_object_mut()
            .unwrap()
            .values_mut()
        {
            i["id"] =
                serde_json::Value::Number(i["id"].as_str().unwrap().parse::<i64>().unwrap().into());
            i["name_short"] = i["short"].clone();
        }

        for i in data["dbi"]["classes"].as_object_mut().unwrap().values_mut() {
            for y in ["classroomid", "grade", "id", "teacherid", "teacher2id"] {
                if i[y].as_str().unwrap() == "" {
                    i[y] = serde_json::Value::Number(0.into());
                    continue;
                }
                i[y] = serde_json::Value::Number(
                    i[y].as_str().unwrap().parse::<i64>().unwrap().into(),
                );
            }
            i["classroom_id"] = i["classroomid"].clone();
            i["name_short"] = i["short"].clone();
            i["teacher_id"] = i["teacherid"].clone();
            i["teacher2_id"] = i["teacher2id"].clone();
        }

        for i in [
            "classes",
            "classrooms",
            "plans",
            "students",
            "subjects",
            "teachers",
            "parents",
        ]
        .iter()
        {
            let mut arr = Vec::new();
            for j in data["dbi"][i].as_object().unwrap().values() {
                arr.push(j.clone());
            }
            data["dbi"][i] = serde_json::Value::Array(arr);
        }

        let mut dayplans = Vec::new();
        for i in data["dp"]["dates"].as_object().unwrap() {
            let mut dayplan = serde_json::json!({});
            dayplan["date"] = serde_json::Value::String(i.0.clone());
            dayplan["lessons"] = serde_json::Value::Array(Vec::new());
            for y in i.1["plan"].as_array().unwrap() {
                let mut y = y.clone();
                if y["subjectid"].as_str().is_some() {
                    y["subject_id"] = y["subjectid"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap()
                        .into();
                } else {
                    y["subject_id"] = serde_json::Value::Number(0.into());
                }
                if !y["groupsubjectids"].as_array().unwrap().len() == 0 {
                    y["plan_id"] = serde_json::Value::Number(
                        y["groupsubjectids"]
                            .as_array()
                            .unwrap()
                            .first()
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .parse::<i64>()
                            .unwrap()
                            .into(),
                    );
                } else {
                    y["plan_id"] = serde_json::Value::Number(0.into());
                }
                y["period"] = y["period"].as_str().unwrap().parse::<i64>().unwrap().into();
                dayplan["lessons"].as_array_mut().unwrap().push(y.clone());
            }
            dayplans.push(dayplan);
        }
        data["day_plans"] = serde_json::Value::Array(dayplans);

        data["userdata"] = data["userrow"].clone();
        data["nameday_today"] = data["meninyDnes"].clone();
        data["nameday_tomorrow"] = data["meninyZajtra"].clone();

        data["userdata"]["class_id"] = data["userdata"]["TriedaID"].clone();
        data["userdata"]["first_name"] = data["userdata"]["p_meno"].clone();
        data["userdata"]["last_name"] = data["userdata"]["p_priezvisko"].clone();
        data["userdata"]["mail"] = data["userdata"]["p_mail"].clone();
        data["userdata"]["login"] = data["userdata"]["p_www_login"].clone();

        data["dbi"]["homeworks_enabled"] = data["dbi"]["homeworksEnabled"].clone();
        data["dbi"]["art_school"] = data["dbi"]["jeZUS"].clone();
        let data: edupage_data::Data = serde::Deserialize::deserialize(data).unwrap();
        Ok(data)
    }
}
