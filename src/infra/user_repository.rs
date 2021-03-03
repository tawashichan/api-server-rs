use crate::domain::model::identity::Id;
use crate::domain::model::user::{User, UserError};
use crate::domain::service::user_service::IUserRepository;
use anyhow::Result;
use async_trait::async_trait;
use dynomite::Item;
use dynomite::{
    dynamodb::{DynamoDb, DynamoDbClient, GetItemInput},FromAttributes,Attributes,
};
use std::sync::Arc;

pub struct UserRepository {
    client: Arc<DynamoDbClient>,
}

impl UserRepository {
    pub fn new(client: Arc<DynamoDbClient>) -> Self {
        UserRepository { client }
    }
}

#[derive(Item, Debug, Clone)]
struct UserRecord {
    #[dynomite(partition_key)]
    user_id: String,
    name: String,
}

impl UserRecord {
    fn to_model(self) -> User {
        User::new(Id::<User>::new(self.user_id), self.name)
    }
}

static TABLE_NAME: &str = "user_tawashi";

#[async_trait]
impl IUserRepository for UserRepository {
    async fn find_by_id(&self, user_id: &Id<User>) -> Result<User> {
        let user_id = user_id.string();
        let key = UserRecordKey { user_id };
        let key: Attributes = key.into();

        let result = self
            .client
            .get_item(GetItemInput {
                table_name: TABLE_NAME.into(),
                key,
                ..Default::default()
            })
            .await?;

        let rec: UserRecord = FromAttributes::from_attrs(result.item.ok_or(UserError::NotFound)?)?;
        Ok(rec.to_model())
    }
}
