use async_trait::async_trait;
use bigdecimal::BigDecimal;
use chrono::{ NaiveDate};
use sqlx::{Error, QueryBuilder};
use uuid::Uuid;
use crate::{database::DBClient, models::expense::Expense};


#[async_trait]
pub trait ExpenseExt {
    async fn create_expense(
        &self,
        amount:BigDecimal,
        expense_date:NaiveDate,
        user_id:Uuid,
        category_id:Uuid
    )->Result<Expense,Error>;

    async fn get_all_expenses(
        &self,
        page:i64,
        limit:i64,
        user_id:Uuid,
        category_id:Uuid,
    )->Result<Option<Vec<Expense>>,Error>;

    async fn get_expense(
        &self,
        user_id:Uuid,
        category_id:Uuid,
        expense_id:Uuid
    )->Result<Option<Expense>,Error>;

    async fn update_expense(
        &self,
        amount:Option<BigDecimal>,
        expense_date:Option<NaiveDate>,
        user_id:Uuid,
        category_id:Uuid,
        expense_id:Uuid
    )->Result<Option<Expense>,Error>;

    async fn delete_expense(
        &self,
        user_id:Uuid,
        category_id:Uuid,
        expense_id:Uuid
    )->Result<(),Error>;
}


#[async_trait]
impl ExpenseExt for DBClient {

    async fn create_expense(
        &self,
        amount:BigDecimal,
        expense_date:NaiveDate,
        user_id:Uuid,
        category_id:Uuid
    )->Result<Expense,Error> {
        let expense = sqlx::query_as!(
            Expense,
            r#"
                INSERT INTO expenses(amount,expense_date,user_id,category_id)
                VALUES($1,$2,$3,$4)
                RETURNING id,amount,expense_date,user_id,category_id,
                created_at,updated_at
            "#,
            amount,
            expense_date,
            user_id,
            category_id
        ).fetch_one(&self.pool).await?;

        Ok(expense)
    }

    async fn get_all_expenses(
        &self,
        page:i64,
        limit:i64,
        user_id:Uuid,
        category_id:Uuid,
    )->Result<Option<Vec<Expense>>,Error> {
        let offset = (page - 1) * limit;

        let expenses = sqlx::query_as!(
            Expense,
            r#"
                SELECT * FROM expenses
                WHERE user_id = $1 
                AND category_id = $2
                ORDER BY created_at DESC
                LIMIT $3 OFFSET $4
            
            "#,
            user_id,
            category_id,
            limit,
            offset
        ).fetch_all(&self.pool).await?;

        if expenses.len() > 0 {
            
            Ok(Some(expenses))
        }else {
            Ok(None)
        }

    }

    async fn get_expense(
        &self,
        user_id:Uuid,
        category_id:Uuid,
        expense_id:Uuid
    )->Result<Option<Expense>,Error> {
        let expense = sqlx::query_as!(
            Expense,
            r#"
                SELECT * FROM expenses
                WHERE user_id = $1
                AND category_id = $2
                AND id = $3
            "#,
            user_id,
            category_id,
            expense_id
        ).fetch_optional(&self.pool).await?;

        Ok(expense)
    }

    async fn update_expense(
        &self,
        amount:Option<BigDecimal>,
        expense_date:Option<NaiveDate>,
        user_id:Uuid,
        category_id:Uuid,
        expense_id:Uuid
    )->Result<Option<Expense>,Error> {
        let mut qb = QueryBuilder
                                                ::new("UPDATE expenses SET ");
                                                
        let mut separator = qb.separated(",");

        if let Some(amount) = amount {
            separator.push(" amount = ").push_bind(amount);
        } 
         if let Some(expense_date) = expense_date {
            separator.push(" expense_date = ").push_bind(expense_date);
        } 

        qb.push(" WHERE user_id = ").push_bind(user_id);
        qb.push(" AND category_id = ").push_bind(category_id);
        qb.push(" AND id = ").push_bind(expense_id);

        let updated_expense = qb.build_query_as::<Expense>()
                                              .fetch_optional(&self.pool).await?;

        Ok(updated_expense)
    }

    async fn delete_expense(
        &self,
        user_id:Uuid,
        category_id:Uuid,
        expense_id:Uuid
    )->Result<(),Error> {
        sqlx::query!(
            r#"
                DELETE FROM expenses
                WHERE user_id = $1
                AND category_id = $2
                AND id = $3
            "#,
            user_id,
            category_id,
            expense_id
        ).execute(&self.pool).await?;
        
        Ok(())
    }
}