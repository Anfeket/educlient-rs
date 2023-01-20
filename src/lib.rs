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
        let params = [
            ("username", username),
            ("password", password),
        ];
        let res = self.session.post(url).form(&params).send().unwrap();
        if res.url().as_str().contains("bad=1") {
            self.logged_in = false;
            return Err(Error::LoginFailed);
        }
        let data = res.text().unwrap()
            .replace("\t", "")
            .replace("\r", "")
            .replace("\n", "");
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
        let to_parse = res.split(".znamkyStudentViewer(").collect::<Vec<_>>()[1].split(");\r\n\t\t});\r\n\t\t</script>").collect::<Vec<_>>()[0];
        let json = serde_json::from_str(to_parse);
        if json.is_err() {
            return Err(Error::ParseError);
        } else {
            return Ok(json.unwrap());
        }
    }

    pub fn deserialize(&self) -> Result<edupage_data::Data, Error> {
        if !self.logged_in {
            return Err(Error::NotLoggedIn);
        }
        let mut data = self.data.clone();

        data["year"] = data["dp"]["year"].clone();
        
        let user = match data["userid"].as_str().unwrap().chars().nth(0).unwrap().to_string().as_str() {
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
            i["id"] = serde_json::Value::from(i["id"].as_str().unwrap().replace("zvonenie", "").parse::<i32>().unwrap());
        }
        
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
        data["userrow"]["TriedaID"] = serde_json::Value::Number(data["userrow"]["TriedaID"].as_str().unwrap().parse::<i64>().unwrap().into());
        
        for i in data["dbi"]["parents"].as_object_mut().unwrap().values_mut() {
            i["gender"] = match i["gender"].as_str().unwrap() {
                "M" => serde_json::Value::from("Male"),
                "F" => serde_json::Value::from("Female"),
                _ => todo!()
            };
            i["id"] = serde_json::Value::Number(i["id"].as_str().unwrap().parse::<i64>().unwrap().into());
        }
        
        for i in data["dbi"]["students"].as_object_mut().unwrap().values_mut() {
            let mut parents: Vec<i64> = Vec::new();
            for y in 0..3 {
                let parent = format!("parent{}id", y+1);
                if i[&parent].as_str().unwrap() == "" {
                    parents.push(0);
                    continue;
                }
                parents.push(i[&parent].as_str().unwrap().parse::<i64>().unwrap());
            }
            i["parents"] = serde_json::Value::Array(parents.into_iter().map(|x| serde_json::Value::Number(x.into())).collect());

            i["classid"] = serde_json::Value::Number(i["classid"].as_str().unwrap().parse::<i64>().unwrap().into());
            i["id"] = serde_json::Value::Number(i["id"].as_str().unwrap().parse::<i64>().unwrap().into());
            i["numberinclass"] = serde_json::Value::Number(i["numberinclass"].as_str().unwrap().parse::<i64>().unwrap().into());

            
            i["gender"] = match i["gender"].as_str().unwrap() {
                "M" => serde_json::Value::from("Male"),
                "F" => serde_json::Value::from("Female"),
                _ => return Err(Error::ParseError),
            };
        }

        for i in data["dbi"]["subjects"].as_object_mut().unwrap().values_mut() {
            i["id"] = serde_json::Value::Number(i["id"].as_str().unwrap().parse::<i64>().unwrap().into());
        }
        
        for i in data["dbi"]["teachers"].as_object_mut().unwrap().values_mut() {
            i["gender"] = match i["gender"].as_str().unwrap() {
                "M" => serde_json::Value::from("Male"),
                "F" => serde_json::Value::from("Female"),
                _ => todo!()
            };
            i["id"] = serde_json::Value::Number(i["id"].as_str().unwrap().parse::<i64>().unwrap().into());
            if i["classroomid"] != "" {
                i["classroomid"] = serde_json::Value::Number(i["classroomid"].as_str().unwrap().parse::<i64>().unwrap().into());
            } else {
                i["classroomid"] = serde_json::Value::Number(0.into());
            }
        }
        
        for i in data["dbi"]["plans"].as_object_mut().unwrap().values_mut() {
            let mut ids = Vec::new();
            for j in i["ucitelids"].as_array().unwrap() {
                ids.push(serde_json::Value::Number(j.as_str().unwrap().parse::<i64>().unwrap().into()));
            }
            i["ucitelids"] = serde_json::Value::Array(ids);
            let mut ids = Vec::new();
            for j in i["students"].as_array().unwrap() {
                if j.is_i64() {
                    ids.push(j.clone());
                    continue;
                }
                ids.push(serde_json::Value::Number(j.as_str().unwrap().parse::<i64>().unwrap().into()));
            }
            i["students"] = serde_json::Value::Array(ids);
            i["planid"] = serde_json::Value::Number(i["planid"].as_str().unwrap().parse::<i64>().unwrap().into());
            i["predmetid"] = serde_json::Value::Number(i["predmetid"].as_str().unwrap().parse::<i64>().unwrap().into());
            for y in i["triedy"].as_array_mut().unwrap() {
                if y.is_string() {
                    *y = serde_json::Value::Number(y.as_str().unwrap().parse::<i64>().unwrap().into());
                }
            }
        }

        for i in data["dbi"]["classrooms"].as_object_mut().unwrap().values_mut() {
            i["id"] = serde_json::Value::Number(i["id"].as_str().unwrap().parse::<i64>().unwrap().into());
        }

        for i in data["dbi"]["classes"].as_object_mut().unwrap().values_mut() {
            for y in ["classroomid", "grade", "id", "teacherid", "teacher2id"] {
                if i[y].as_str().unwrap() == "" {
                    i[y] = serde_json::Value::Number(0.into());
                    continue;
                }
                i[y] = serde_json::Value::Number(i[y].as_str().unwrap().parse::<i64>().unwrap().into());
            }
        }
        
        for i in ["classes", "classrooms", "plans", "students", "subjects", "teachers", "parents"].iter() {
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
            dayplan["plans"] = serde_json::Value::Array(Vec::new());
            dayplans.push(dayplan);
        }
        data["day_plans"] = serde_json::Value::Array(dayplans);
        
        // write data to file
        let mut file = std::fs::File::create("data.json").unwrap();
        std::io::Write::write_all(&mut file, serde_json::to_string(&data).unwrap().as_bytes()).unwrap();
        let data: edupage_data::Data = serde::Deserialize::deserialize(data).unwrap();
        Ok(data)
    }
}