use anyhow::Result;

pub struct Config {
    pub user_table_name: String,
    pub user_email_table_name: String,
}

const USER_TABLE_NAME: &'static str = "USER_TABLE_NAME";
const USER_EMAIL_TABLE_NAME: &'static str = "USER_EMAIL_TABLE_NAME";

impl Config {
    pub fn init() -> Result<Self> {
        let user_table_name = std::env::var(USER_TABLE_NAME)?;
        let user_email_table_name = std::env::var(USER_EMAIL_TABLE_NAME)?;

        Ok(Config {
            user_table_name,
            user_email_table_name,
        })
    }
}
