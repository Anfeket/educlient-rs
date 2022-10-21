#![crate_type = "lib"]
#![crate_name = "educlient"]

use reqwest::blocking;


#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

#[derive(Debug)]
pub enum Position {
    Student,
    Parent,
    Teacher,
}

#[derive(Debug)]
pub enum Error {
    LoginFailed,
    NoResponse,
    ParseError,
    Unknown,
}


pub struct Educlient {
    pub logged_in: bool,
    pub domain: String,
    pub session: blocking::Client,
    pub account: EduAccount,
}

pub struct EduAccount {
    pub username: String,
    pub password: String,
    pub id: i32,
    pub name: String,
    pub gender: Gender,
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
        };
        Educlient {
            logged_in: false,
            domain,
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
            Ok(self)
        } else {
            self.logged_in = true;
            Err(Error::LoginFailed)
        }
    }

    pub fn get_grades(&self) -> Result<serde_json::Value, Error> {
        let url = format!("https://{}.edupage.org/znamky/?", self.domain);
        let res = self.session.get(url).send();
        if res.is_err() {
            Err(Error::NoResponse)
        } else {    
            let to_parse = res.unwrap().text().unwrap();
            let text = to_parse.split(".znamkyStudentViewer(").collect::<Vec<_>>()[1].split(");\r\n\t\t});\r\n\t\t</script>").collect::<Vec<_>>()[0];
            let json = serde_json::from_str(text);
            if json.is_err() {
                Err(Error::ParseError)
            } else {
                Ok(json.unwrap())
            }
        }
    }

    pub fn get_account_info(&self) -> &EduAccount {
        todo!()
        // assign values to EduAccount
    }
}