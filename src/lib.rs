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


pub struct Educlient {
    pub logged_in: bool,
    pub domain: String,
    pub data: serde_json::Value,
    pub session: blocking::Client,
    pub account: EduAccount,
}

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
        println!("{}", data);
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
            Err(Error::NoResponse)
        } else {
            let res = res.unwrap().text().unwrap();
            let to_parse = res.split(".znamkyStudentViewer(").collect::<Vec<_>>()[1].split(");\r\n\t\t});\r\n\t\t</script>").collect::<Vec<_>>()[0];
            let json = serde_json::from_str(to_parse);
            if json.is_err() {
                Err(Error::ParseError)
            } else {
                Ok(json.unwrap())
            }
        }
    }

    pub fn get_account_info(&self) -> &EduAccount {
        &self.account
    }
}