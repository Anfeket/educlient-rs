#![crate_type = "lib"]
#![crate_name = "educlient"]

use reqwest::blocking;

#[derive(Debug)]
pub struct Educlient {
    pub domain: String,
    pub session: blocking::Client,
    pub logged_in: bool,
}

impl Educlient {
    pub fn new(domain: &str) -> Educlient {
        let session = blocking::Client::new();
        Educlient {
            domain: domain.to_string(),
            session,
            logged_in: false,
        }
    }

    pub fn login(&mut self, username: String, password: String) -> &Educlient {
        self.session.post(format!("https://{}.edupage.org/login/edubarlogin.php", self.domain))
            .form(&[("username", username), ("password", password)])
            .send()
            .unwrap();
        self.logged_in = true;
        self
    }

    pub fn get_grades(&self) -> (&Educlient, String) {
        let session = self.session.get(format!("https://{}.edupage.org/znamky/?", self.domain))
            .send()
            .unwrap();
        let mut response = session.text().unwrap();
        let grades_vec1: Vec<_> = response.split(".znamkyStudentViewer(").collect();
        response = grades_vec1[1].to_owned();
        let grades_vec2: Vec<_> = response.split(");\r\n\t\t});\r\n\t\t</script>").collect();
        let grades = grades_vec2[0].to_owned();
        return (self, grades);
    }
}