use crate::config::Config;
use crate::domain::model::error::DomainError;
use crate::domain::model::identity::Id;
use crate::domain::model::{
    email::Email,
    user::{User,UserId, UserName},
};
use crate::domain::traits::user_repository::IUserRepository;
use anyhow::Result;
use async_trait::async_trait;
use dynomite::dynamodb::QueryInput;
use dynomite::Item;
use dynomite::{
    dynamodb::{DynamoDb, DynamoDbClient, GetItemInput, PutItemInput},
    Attributes, FromAttributes,
};
use std::sync::Arc;

pub struct UserRepository {
    table_name: String,
    gsi_name_email: String,
    client: Arc<DynamoDbClient>,
}

impl UserRepository {
    pub fn new(conf: &Config, client: Arc<DynamoDbClient>) -> Self {
        UserRepository {
            table_name: conf.user_table_name.clone(),
            gsi_name_email: "gsi_email".to_string(),
            client,
        }
    }
}

#[derive(Item, Debug, Clone)]
struct UserRecord {
    #[dynomite(partition_key)]
    user_id: String,
    name: String,
    email: String,
}

impl UserRecord {
    fn to_model(self) -> Result<User,DomainError> {
        let id = UserId::new_from_string(&self.user_id)?;
        let name = UserName::new(&self.name)?;
        let email = Email::new(&self.name)?;

        Ok(User::new(id, name, email))
    }

    fn from_model(user_model: User) -> Self {
        let (id, name, email) = user_model.propeties();
        UserRecord {
            user_id: id.string(),
            name: name.string(),
            email: email.string(),
        }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn find_by_email(&self, email: &Email) -> Result<User,DomainError> {
        let email = email.string();

        let result = self
            .client
            .query(QueryInput {
                table_name: self.table_name.clone(),
                index_name: Some(self.gsi_name_email.clone()),
                key_condition_expression: Some(
                    format!("{:?} = {:?}", self.gsi_name_email.clone(), email).to_string(),
                ),
                ..Default::default()
            })
            .await.map_err(|e| DomainError::DBError)?;

        unimplemented!()
    }

    async fn find_by_id(&self, user_id: &Id<User>) -> Result<User,DomainError> {
        let user_id = user_id.string();
        let key = UserRecordKey { user_id };
        let key: Attributes = key.into();

        let result = self
            .client
            .get_item(GetItemInput {
                table_name: self.table_name.clone(),
                key,
                ..Default::default()
            })
            .await.map_err(|e| DomainError::DBError)?;

        let rec: UserRecord = UserRecord::from_attrs(result.item.ok_or(DomainError::UserNotFound)?).map_err(|err| DomainError::UserNotFound)?;
        Ok(rec.to_model()?)
    }

    async fn save(&self, user: User) -> Result<(),DomainError> {
        let record = UserRecord::from_model(user);

        let _ = self
            .client
            .put_item(PutItemInput {
                table_name: self.table_name.clone(),
                item: record.into(),
                ..Default::default()
            })
            .await.map_err(|e| DomainError::DBError)?;

        Ok(())
    }
}
