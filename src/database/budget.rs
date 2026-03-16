use uuid::Uuid;
use sqlx::{Error};
use crate::{database::DBClient, models::budget::Budget};
use async_trait::async_trait;
use bigdecimal::{BigDecimal};

#[async_trait]
pub trait BudgetExt {
    async fn create_budget(
        &self,
        user_id:Uuid,
        category_id:Uuid,
        amount:BigDecimal
    )->Result<Budget,Error>;

    async fn get_budget(
        &self,
        user_id:Uuid,
        category_id:Uuid
    )->Result<Option<Budget>,Error>;

    async fn update_budget(
        &self,
        user_id:Uuid,
        category_id:Uuid,
        budget_id:Uuid,
        amount:BigDecimal
    )->Result<Option<Budget>,Error>;

    async fn delete_budget(
        &self,
        user_id:Uuid,
        category_id:Uuid,
        budget_id:Uuid
    )->Result<(),Error>;
}

#[async_trait]
impl BudgetExt for DBClient {
    async fn create_budget(
        &self,
        user_id:Uuid,
        category_id:Uuid,
        amount:BigDecimal
    )->Result<Budget,Error>{
        let budget = sqlx::query_as!(
            Budget,
            r#"
                INSERT INTO budgets(amount,user_id,category_id)
                VALUES ($1,$2,$3)
                RETURNING id,amount,user_id,category_id,created_at,updated_at
            "#,
            amount,
            user_id,
            category_id,
        ).fetch_one(&self.pool).await?;

        Ok(budget)
    }

     async fn get_budget(
        &self,
        user_id:Uuid,
        category_id:Uuid
    )->Result<Option<Budget>,Error> {
        let budget = sqlx::query_as!(
            Budget,
            r#"
                SELECT id, amount, user_id, category_id, created_at, updated_at FROM budgets
                WHERE user_id = $1
                AND category_id = $2
            "#,
            user_id,
            category_id
        ).fetch_optional(&self.pool).await?;

        Ok(budget)
    }

    async fn update_budget(
        &self,
        user_id:Uuid,
        category_id:Uuid,
        budget_id:Uuid,
        amount:BigDecimal
    )->Result<Option<Budget>,Error> {
        let updated_budget = sqlx::query_as!(
            Budget,
            r#"
               UPDATE budgets 
               SET amount = $1,
               updated_at = NOW()
               WHERE user_id = $2 
               AND category_id = $3
               AND id = $4 
               RETURNING id, amount, user_id, category_id, created_at, updated_at
            "#,
            amount,
            user_id,
            category_id,
            budget_id
        ).fetch_optional(&self.pool).await?;

        Ok(updated_budget)
    }

    async fn delete_budget(
        &self,
        user_id:Uuid,
        category_id:Uuid,
        budget_id:Uuid
    )->Result<(),Error> {
        sqlx::query!(
            r#"
                DELETE FROM budgets
                WHERE user_id = $1 
                AND category_id = $2
                AND id = $3  
            "#,
              user_id,
            category_id,
            budget_id
        ).execute(&self.pool).await?;
        Ok(())
    }
}
