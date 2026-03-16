use async_trait::async_trait;
use uuid::Uuid;
use sqlx::{Error, QueryBuilder};
use crate::{database::DBClient, models::user::User, utils::password};

#[async_trait]
pub trait AuthExt {
    async fn login<T: Into<String> + Send>(
        &self,
        email:T,
    ) -> Result<Option<User>,Error>;
}

#[async_trait]
impl AuthExt for DBClient {
    async fn login<T: Into<String> + Send>(
        &self,
        email:T,
    ) -> Result<Option<User>,Error> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users
                WHERE email = $1
            "#,
            email.into(),
        ).fetch_optional(&self.pool).await?;

        Ok(user)
    }
}