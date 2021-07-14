use crate::{
    domain::{
        model::{
            email::Email,
            error::DomainError,
            login::{LoginPassword, LoginToken},
        },
        traits::{jwt_handler::IJWTHandler, user_repository::IUserRepository},
    },
    infra::{jwt_handler::JWTHandler, user_repository::UserRepository},
};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct LoginRequest {
    pub email: Email,
    pub password: LoginPassword,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    token: LoginToken,
}

#[async_trait]
pub trait ILoginService {
    type JWTHandler: IJWTHandler + Sync + Send;
    type UserRepository: IUserRepository + Sync + Send;

    fn new(jwt_handler: Arc<Self::JWTHandler>, user_repository: Arc<Self::UserRepository>) -> Self;

    fn jwt_handler(&self) -> &Self::JWTHandler;

    fn user_repository(&self) -> &Self::UserRepository;

    async fn login(&self, req: LoginRequest) -> Result<LoginResponse, DomainError> {
        let user = self.user_repository().find_by_email(&req.email).await?;
        let jwt = self.jwt_handler().generate(&user)?;
        Ok(LoginResponse { token: jwt })
    }
}

pub struct LoginService {
    jwt_handler: Arc<JWTHandler>,
    user_repository: Arc<UserRepository>,
}

#[async_trait]
impl ILoginService for LoginService {
    type JWTHandler = JWTHandler;
    type UserRepository = UserRepository;

    fn new(jwt_handler: Arc<Self::JWTHandler>, user_repository: Arc<Self::UserRepository>) -> Self {
        LoginService {
            jwt_handler,
            user_repository,
        }
    }

    fn jwt_handler(&self) -> &Self::JWTHandler {
        &self.jwt_handler
    }

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}
