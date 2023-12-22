use std::env::var;

#[derive(Debug, Clone)]
pub struct Config {
    pub email: lettre::Address,
    pub domain: String,
}

impl Config {
    pub fn load() -> Config {
        let email = var("SMTP_EMAIL")
            .expect("Could not load SMTP_EMAIL from env")
            .parse()
            .unwrap();

        let domain = var("DOMAIN").expect("Could not load DOMAIN from env ");

        Config { email, domain }
    }
}
