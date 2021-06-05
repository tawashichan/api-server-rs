use crate::config::Config;
use crate::domain::model::identity::Id;
use crate::domain::model::{
    email::Email,
    user::{User, UserError, UserId, UserName},
};
use crate::domain::traits::user_repository::IUserRepository;
use anyhow::Result;
use async_trait::async_trait;
use dynomite::Item;
use dynomite::{
    dynamodb::{DynamoDb, DynamoDbClient, GetItemInput, PutItemInput},
    Attributes, FromAttributes,
};
use std::sync::Arc;

pub struct UserRepository {
    table_name: String,
    client: Arc<DynamoDbClient>,
}

impl UserRepository {
    pub fn new(conf: &Config, client: Arc<DynamoDbClient>) -> Self {
        UserRepository {
            table_name: conf.user_table_name.clone(),
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
    fn to_model(self) -> Result<User> {
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
    async fn find_by_email(&self, email: &Email) -> Result<User> {
        unimplemented!()
    }

    async fn find_by_id(&self, user_id: &Id<User>) -> Result<User> {
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
            .await?;

        let rec: UserRecord = UserRecord::from_attrs(result.item.ok_or(UserError::NotFound)?)?;
        Ok(rec.to_model()?)
    }

    async fn save(&self, user: User) -> Result<()> {
        let record = UserRecord::from_model(user);

        let _ = self
            .client
            .put_item(PutItemInput {
                table_name: self.table_name.clone(),
                item: record.into(),
                ..Default::default()
            })
            .await?;

        Ok(())
    }
}
