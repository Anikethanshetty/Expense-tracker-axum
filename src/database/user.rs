use async_trait::async_trait;
use uuid::Uuid;
use sqlx::{Error, QueryBuilder};
use crate::{database::DBClient, models::user::User};

#[async_trait]
pub trait UserExt {
    async fn create_user<T:Into<String> + Send>(
        &self,
        username:T,
        email:T,
        password:T
    ) -> Result<User,Error>;

    async fn get_user(
        &self,
        user_id:Uuid,
    ) -> Result<Option<User>,Error>;

    async fn update_user<T:Into<String> + Send>(
        &self,
        user_id:Uuid,
        username:Option<T>,
        email:Option<T>
    )->Result<Option<User>,Error>;

    async fn delete_user(
        &self,
        email:String
    )->Result<(),Error>;
}

#[async_trait]
impl UserExt for DBClient {
      async fn create_user<T:Into<String> + Send>(
        &self,
        username:T,
        email:T,
        password:T
    ) ->Result<User,Error> {
        let username = username.into();
        let email = email.into();
        let password = password.into();

        let user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (username,email,password)
                VALUES ($1,$2,$3)
                RETURNING id,username,email,password,created_at,updated_at                          
            "#,
            username,
            email,
            password
        ).fetch_one(&self.pool).await?;

        Ok(user)
    }

    async fn get_user(
            &self,
            user_id:Uuid
        ) -> Result<Option<User>,Error>
        {
            let row = sqlx::query_as!(
                User,
                r#"
                    SELECT * FROM users
                    WHERE id = $1
                "#,
                user_id
            ).fetch_optional(&self.pool).await?;

            Ok(row)       
        }

       async fn update_user<T:Into<String> + Send>(
            &self,
            user_id: Uuid,
            username: Option<T>,
            email: Option<T>
        ) -> Result<Option<User>,Error>
          {
            let email = email.map(|v| v.into());
            let username = username.map(|v| v.into());

            let mut qb = QueryBuilder::new("UPDATE users SET ");

            let mut separated = qb.separated(", ");

            if let Some(username) = username {
                separated.push("username = ").push_bind(username);
            }
            
            if let Some(email) = email {
                separated.push("email = ").push_bind(email);
            }

            qb.push(" WHERE id =").push_bind(user_id);
            qb.push(" RETURNING id,username,password,created_at,updated_at");
            qb.push(" updated_at = NOW()");

            let user = qb.build_query_as::<User>()
                                                          .fetch_optional(&self.pool)
                                                          .await?;
            
            Ok(user)

}

       async fn delete_user(
            &self,
            email:String
        ) -> Result<(),Error> {
            sqlx::query!(
                r#"
                    DELETE FROM users
                    WHERE email = $1
                "#,
                email
            ).execute(&self.pool).await?;
             Ok(())
    }

}