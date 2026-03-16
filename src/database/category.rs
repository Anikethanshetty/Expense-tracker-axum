use async_trait::async_trait;
use uuid::Uuid;
use sqlx::{Error, QueryBuilder};

use crate::{database::DBClient, models::{budget::Budget, category::Category}};

#[async_trait]
pub trait CategoryExt {

    async fn create_category<T:Into<String> + Send>(
        &self,
        category_name: T,
        description: T,
        user_id: Uuid,
    )->Result<Category,Error>;

    async fn get_all_categories(
        &self,
        user_id:Uuid,
        page:Option<i64>,
        limit:Option<i64>
    )->Result<Option<Vec<Category>>,Error>;

    async fn get_category(
        &self,
        user_id:Uuid,
        category_id:Uuid,
    )->Result<Option<Category>,Error>;

    async fn update_categoy<T:Into<String> + Send>(
        &self,
        name:Option<T>,
        description:Option<T>,
        user_id:Uuid,
        category_id:Uuid
    )-> Result<Option<Category>,Error>;

    async fn delete_category(
        &self,
        user_id:Uuid,
        category_id:Uuid
    )->Result<(),Error>;
}

#[async_trait]
impl CategoryExt for DBClient {
     async fn create_category<T:Into<String> + Send>(
        &self,
        category_name: T,
        description: T,
        user_id: Uuid,
    ) -> Result<Category,Error> 
    {
        let name = category_name.into();
        let description = description.into();

        
        let category = sqlx::query_as!(
            Category,
            r#"
                INSERT INTO categories (category_name,description,user_id)
                VALUES ($1,$2,$3)
                RETURNING id, user_id, category_name, description, created_at, updated_at
            "#,
            name,
            description,
            user_id
        ).fetch_one(&self.pool).await?;
        
        Ok(category)
    }

        async fn get_all_categories(
                &self,
                user_id:Uuid,
                page:Option<i64>,
                limit:Option<i64>
            ) ->Result<Option<Vec<Category>>,Error> 
            {
                let page = page.unwrap_or(1);
                let limit = limit.unwrap_or(10);
                let offset = (page -1 ) * limit ;

                let categories = sqlx::query_as!(
                    Category,
                    r#"
                        SELECT * FROM categories 
                        WHERE id = $1
                        ORDER BY created_at DESC
                        LIMIT $2 OFFSET $3
                    "#,
                    user_id,
                    limit,
                    offset
                ).fetch_all(&self.pool).await?;

                if categories.len() > 0 {
                    Ok(Some(categories))
                }else {
                    Ok(None)
                }

        }

        async fn get_category(
        &self,
        user_id:Uuid,
        category_id:Uuid,
    )->Result<Option<Category>,Error> {
        let category = sqlx::query_as!(
            Category,
            r#"
                SELECT * FROM categories
                WHERE id = $1 
                AND user_id = $2
            "#,
            category_id,
            user_id
        ).fetch_optional(&self.pool).await?;

        Ok(category)
    }

     async fn update_categoy<T:Into<String> + Send>(
        &self,
        name:Option<T>,
        description:Option<T>,
        user_id:Uuid,
        category_id:Uuid
    )->Result<Option<Category>,Error> {
        let name = name.map(|v| v.into());
        let description = description.map(|v|v.into());

        let mut qb = QueryBuilder::new("UPDATE categories SET ");

        let mut separator = qb.separated(",");

        if let Some(name) = name {
            separator.push("name = ").push_bind(name);
        }

        if let Some(description) = description {
            separator.push(" description = ").push_bind(description);
        }

        qb.push(" WHERE user_id = ").push_bind(user_id);
        qb.push(" AND id = ").push_bind(category_id);
        qb.push(" updated_at = NOW()");

        let updated_category = qb.build_query_as::<Category>()
                                        .fetch_optional(&self.pool).await?;

        Ok(updated_category)
    }

     async fn delete_category(
        &self,
        user_id:Uuid,
        category_id:Uuid
    ) -> Result<(),Error> 
    {
        sqlx::query!(
            r#"
                DELETE FROM categories 
                WHERE user_id = $1
                AND id = $2
            "#,
            user_id,
            category_id
        ).execute(&self.pool).await?;
        
        Ok(())
    }
}