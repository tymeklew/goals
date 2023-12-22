use std::env::var;

#[derive(Debug, Clone)]
pub struct Config {
    pub email: lettre::Address,
}

impl Config {
    pub fn load() -> Config {
        let email = var("SMTP_EMAIL")
            .expect("Could not load smtp email")
            .parse()
            .unwrap();

        Config { email }
    }
}
