#![crate_type = "lib"]
#![crate_name = "educlient"]

use reqwest::blocking;
use serde_json::Value;


pub struct Educlient {
    pub logged_in: bool,
    pub domain: String,
    pub session: blocking::Client,
    pub account: EduAccount,
}

pub struct EduAccount {
    pub username: String,
    pub password: String,
}

impl Educlient {
    pub fn new(username: String, password: String, domain: String) -> Educlient {
        let session = blocking::Client::new();
        let account = EduAccount {
            username: username,
            password: password,
        };
        let logged_in = false;
        Educlient {
            logged_in: logged_in,
            domain: domain,
            session: session,
            account: account,
        }
    }

    pub fn login(&mut self) -> &Educlient {
        let url = format!("https://{}.edupage.org/login/edubarlogin.php", self.domain);
        let params = [
            ("username", self.account.username.as_str()),
            ("password", self.account.password.as_str()),
        ];
        let res = self.session.post(url).form(&params).send().unwrap();
        if res.url().as_str().contains("bad=1") {
            self.logged_in = false;
            self
        } else {
            println!("Login successful");
            self.logged_in = true;
            self
        }
    }

    pub fn get_grades(&self) -> (&Educlient, Value) {
        let url = format!("https://{}.edupage.org/znamky/?", self.domain);
        let res = self.session.get(url).send().unwrap();
        let to_parse = res.text().unwrap();
        let text = to_parse.split(".znamkyStudentViewer(").collect::<Vec<_>>()[1].split(");\r\n\t\t});\r\n\t\t</script>").collect::<Vec<_>>()[0];
        let json = serde_json::from_str(text).unwrap();
        (self, json)
    }
}