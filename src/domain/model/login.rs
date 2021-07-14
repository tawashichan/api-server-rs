use serde::Serialize;

pub struct LoginPassword(String);

#[derive(Serialize)]
pub struct LoginToken(pub String);
