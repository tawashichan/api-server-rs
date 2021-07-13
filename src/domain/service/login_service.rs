use crate::domain::{
    model::{
        email::Email,
        login::{LoginPassword, LoginToken},
    },
    traits::{jwt_handler::IJWTHandler, user_repository::IUserRepository},
};
use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use std::sync::Arc;

pub struct LoginRequest {
    email: Email,
    password: LoginPassword,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: LoginToken,
}

#[async_trait]
pub trait ILoginService {
    type JWTHandler: IJWTHandler + Sync + Send;
    type UserRepository: IUserRepository + Sync + Send;

    fn new(jwt_handler: Arc<Self::JWTHandler>) -> Self;

    fn jwt_handler(&self) -> &Self::JWTHandler;

    fn user_repository(&self) -> &Self::UserRepository;

    async fn login(&self, req: LoginRequest) -> Result<LoginResponse> {
        let user = self.user_repository().find_by_email(&req.email).await?;
        let jwt = self.jwt_handler().generate(&user)?;
        Ok(LoginResponse { token: jwt })
    }
}
