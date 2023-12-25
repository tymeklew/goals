use std::env::var;

#[derive(Debug, Clone)]
pub struct Config {
    pub email: lettre::Address,
    pub domain: String,
    // How many days the session should last for
    pub session_time: u64,
}

impl Config {
    pub fn load() -> Config {
        let email = var("SMTP_EMAIL")
            .expect("Could not load SMTP_EMAIL from env")
            .parse()
            .expect("Failed to parse SMTP_EMAIL");

        let domain = var("DOMAIN").expect("Could not load DOMAIN");

        let session_time = var("SESSION_TIME")
            .unwrap_or("14".into())
            .trim()
            .parse()
            .expect("Failed to parse SESSION_TIME");

        Config {
            email,
            domain,
            session_time,
        }
    }
}
