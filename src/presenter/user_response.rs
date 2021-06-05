use crate::domain::model::user::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl UserResponse {
    pub fn from_model(user: User) -> Self {
        let (user_id, name, email) = user.propeties();

        UserResponse {
            id: user_id.string(),
            name: name.string(),
            email: email.string(),
        }
    }
}
