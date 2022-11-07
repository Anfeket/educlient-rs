#![crate_type = "lib"]
#![crate_name = "educlient"]

use reqwest::blocking;
use serde_json::Value;

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

#[derive(Debug)]
pub enum AccountType {
    Student,
    Parent,
    Teacher,
    Unknown,
}

#[derive(Debug)]
pub enum Error {
    LoginFailed,
    NotLoggedIn,
    NoResponse,
    ParseError,
    Unknown,
}

#[derive(Debug)]
pub struct Educlient {
    pub logged_in: bool,
    pub domain: String,
    pub data: serde_json::Value,
    pub session: blocking::Client,
    pub account: EduAccount,
}

#[derive(Debug)]
pub struct EduAccount {
    pub username: String,
    pub password: String,
    pub id: i32,
    pub name: String,
    pub gender: Gender,
    pub account_type: AccountType,
}

impl Educlient {
    pub fn new(username: String, password: String, domain: String) -> Educlient {
        let session = blocking::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        let account = EduAccount {
            username,
            password,
            name: "".to_string(),
            gender: Gender::Unknown,
            id: 0,
            account_type: AccountType::Unknown,
        };
        Educlient {
            logged_in: false,
            domain,
            data: Value::Null,
            session,
            account,
        }
    }

    pub fn login(&mut self) -> Result<&Educlient, Error> {
        let url = format!("https://{}.edupage.org/login/edubarlogin.php", self.domain);
        let params = [
            ("username", self.account.username.as_str()),
            ("password", self.account.password.as_str()),
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

    pub fn get_account_info(&mut self) -> Result<&Educlient, Error> {
        if !self.logged_in {
            return Err(Error::NotLoggedIn);
        }
        let (account_type, id) = Educlient::_get_account_type(&self.data);
        let name = format!("{} {}", self.data["userrow"]["p_meno"].as_str().unwrap(), self.data["userrow"]["p_priezvisko"].as_str().unwrap());
        let gender = match self.data["userrow"]["p_pohlavie"].as_str().unwrap() {
            "1" => Gender::Male,
            "2" => Gender::Female,
            _ => Gender::Unknown
        };
        let account = EduAccount {
            username: self.account.username.clone(),
            password: self.account.password.clone(),
            id,
            name,
            gender,
            account_type,
        };
        self.account = account;
        Ok(self)
    }

    fn _get_account_type(data: &Value) -> (AccountType, i32) {
        if !data["userrow"]["StudentID"].is_null() {
            (AccountType::Student, data["userrow"]["StudentID"].as_str().unwrap().parse::<i32>().unwrap())
        } else if !data["userrow"]["RodicID"].is_null() {
            (AccountType::Parent, data["userrow"]["RodicID"].as_str().unwrap().parse::<i32>().unwrap())
        } else if !data["userrow"]["UcitelID"].is_null() {
            (AccountType::Teacher, data["userrow"]["UcitelID"].as_str().unwrap().parse::<i32>().unwrap())
        } else {
            (AccountType::Unknown, 0)
        }
    }
}