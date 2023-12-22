// File for custom validators for crate validate

use validator::ValidationError;

const PASSWORD_BLACKLIST: &str = r#"[^(){}[]|`¬¦!"£$%^&*"<>:;#~_-+=,@]"#;
const MIN_PASSWORD_LEN: u8 = 3;
const MAX_PASSWORD_LEN: u8 = 20;

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let len = password.len() as u8;

    if len > MAX_PASSWORD_LEN || len < MIN_PASSWORD_LEN {
        return Err(ValidationError::new(
            "Password must be between 3 and 20 characters long",
        ));
    }

    for char in password.chars() {
        if PASSWORD_BLACKLIST.chars().any(|f| f == char) {
            return Err(ValidationError::new("Invalid character in password"));
        }
    }

    Ok(())
}
