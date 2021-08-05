use crate::config::Config;
use crate::domain::model::error::DomainError;
use crate::domain::model::identity::Id;
use crate::domain::model::{
    email::Email,
    user::{User, UserId, UserName},
};
use crate::domain::traits::user_repository::IUserRepository;
use anyhow::Result;
use async_trait::async_trait;
use dynamodb::model::{AttributeValue, Put, TransactWriteItem};
use dynamodb::Client;
use std::collections::HashMap;
use std::sync::Arc;

pub struct UserRepository {
    table_name: String,
    email_table_name: String,
    gsi_name_email: String,
    client: Arc<Client>,
}

impl UserRepository {
    pub fn new(conf: &Config, client: Arc<Client>) -> Self {
        UserRepository {
            table_name: conf.user_table_name.clone(),
            email_table_name: conf.user_email_table_name.clone(),
            gsi_name_email: "gsi_email".to_string(),
            client,
        }
    }
}

#[derive(Debug, Clone)]
struct UserRecord {
    user_id: String,
    name: String,
    email: String,
}

#[derive(Debug, Clone)]
struct UserEmailRecord {
    email: String,
    user_id: String,
}

impl UserEmailRecord {
    fn to_item(self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert("user_id".to_string(), AttributeValue::S(self.user_id));
        map.insert("email".to_string(), AttributeValue::S(self.email));
        map
    }
}

impl UserRecord {
    fn to_model(self) -> Result<User, DomainError> {
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

    fn from_item(item: HashMap<String, AttributeValue>) -> Result<Self, DomainError> {
        if let (
            Some(AttributeValue::S(user_id)),
            Some(AttributeValue::S(name)),
            Some(AttributeValue::S(email)),
        ) = (item.get("user_id"), item.get("name"), item.get("email"))
        {
            return Ok(UserRecord {
                user_id: user_id.to_owned(),
                name: name.to_owned(),
                email: email.to_owned(),
            });
        }
        Err(DomainError::UserNotFound)
    }

    fn to_item(self) -> HashMap<String, AttributeValue> {
        let mut map: HashMap<String, AttributeValue> = HashMap::new();
        map.insert("user_id".to_string(), AttributeValue::S(self.user_id));
        map.insert("name".to_string(), AttributeValue::S(self.name));
        map.insert("email".to_string(), AttributeValue::S(self.email));
        map
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn find_by_email(&self, email: &Email) -> Result<User, DomainError> {
        let result = self
            .client
            .query()
            .table_name(self.table_name.clone())
            .index_name(self.gsi_name_email.clone())
            .expression_attribute_values(":email", AttributeValue::S(email.string()))
            .key_condition_expression("email = :email")
            .send()
            .await
            .map_err(|e| DomainError::DBError(e.to_string()))?;

        let items = result.items.ok_or(DomainError::UserNotFound)?;
        if items.len() == 0 {
            return Err(DomainError::UserNotFound);
        }

        UserRecord::from_item(items[0].clone())?.to_model()
    }

    async fn find_by_id(&self, user_id: &Id<User>) -> Result<User, DomainError> {
        let result = self
            .client
            .get_item()
            .table_name(self.table_name.clone())
            .key("user_id", AttributeValue::S(user_id.string()))
            .consistent_read(true)
            .send()
            .await
            .map_err(|e| DomainError::DBError(e.to_string()))?;

        let item = result.item.ok_or(DomainError::UserNotFound)?;

        UserRecord::from_item(item)?.to_model()
    }

    async fn create(&self, user: User) -> Result<(), DomainError> {
        let user_record = UserRecord::from_model(user);
        let email = user_record.email.to_string();
        let user_id = user_record.user_id.to_string();
        let user_item = user_record.to_item();

        let email_item = UserEmailRecord {
            user_id: user_id,
            email: email.clone(),
        }
        .to_item();

        self.client
            .transact_write_items()
            .set_transact_items(Some(vec![
                // 重複チェック用のテーブルを使用し,emailのuniquenessを担保
                TransactWriteItem::builder()
                    .put(
                        Put::builder()
                            .table_name(self.email_table_name.clone())
                            .set_item(Some(email_item))
                            .condition_expression("attribute_not_exists(email)")
                            .build(),
                    )
                    .build(),
                TransactWriteItem::builder()
                    .put(
                        Put::builder()
                            .table_name(self.table_name.clone())
                            .set_item(Some(user_item))
                            .build(),
                    )
                    .build(),
            ]))
            .send()
            .await
            .map_err(|e| DomainError::DBError(e.to_string()))?;
        Ok(())
    }
}
