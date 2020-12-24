use crate::domain::model::identity::Id;
use crate::domain::model::user::User;
use crate::domain::service::user_service::IUserRepository;
use async_trait::async_trait;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};
use std::sync::Arc;

pub struct UserRepository {
    client: Arc<DynamoDbClient>,
}

impl UserRepository {
    pub fn new(client: Arc<DynamoDbClient>) -> Self {
        UserRepository { client }
    }
}

static TABLE_NAME: &str = "user_tawashi";

#[async_trait]
impl IUserRepository for UserRepository {
    async fn find_by_id(&self, user_id: &Id<User>) -> Result<User, ()> {
        let mut key: std::collections::hash_map::HashMap<String, AttributeValue> =
            std::collections::hash_map::HashMap::new();
        key.insert(
            String::from("user_id"),
            AttributeValue {
                s: Some(user_id.string()),
                ..Default::default()
            },
        );
        let input = GetItemInput {
            key,
            table_name: String::from(TABLE_NAME),
            ..Default::default()
        };
        match self.client.get_item(input).await {
            Ok(result) => {
                let item = result.item.unwrap();
                let id = item.get("user_id").unwrap().s.as_ref().unwrap();
                let name = item.get("name").unwrap().s.as_ref().unwrap();
                Ok(User::new(Id::new(id.clone()), name.clone()))
            }
            Err(e) => {
                panic!("{:?}",e)
            }
        }
    }
}
