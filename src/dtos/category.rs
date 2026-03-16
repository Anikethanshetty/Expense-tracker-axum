use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use validator::Validate;

use crate::{ models::category::Category};

#[derive(Debug,Clone,Serialize,Deserialize,Validate)]
pub struct FilterCategory {
    pub id: Option<Uuid>,
    #[serde(rename = "categoryName")]
    pub category_name: Option<String>,
    pub description: Option<String>,
}

impl FilterCategory {
    pub fn filter_category(category: &Category) -> Self {
        Self { 
            id: Some(category.id), 
            category_name: Some(category.category_name.clone()), 
            description: Some(category.description.clone())
        }
    }
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct CreateCategoryDto {
    #[validate(length(min=1,max=100,message = "category must greater than or equal to 1 and less than 100 characters"))]
    #[serde(rename="categoryName")]
    pub category_name: String,
    #[validate(length(min=1,max=255,message = "description must be greater than or equal to 1 and less than 255 characters"))]
    pub description: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct CategoryData  {
    pub category: FilterCategory
}

#[derive(Debug,Serialize,Deserialize)]
pub struct CategoryResponseDto {
    pub status: &'static str,
    pub data: CategoryData
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesData {
    pub categories: Vec<FilterCategory>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesResponseDto {
    pub status: &'static str,
    pub data: CategoriesData
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct GetCategoryDto {
    #[serde(rename="categoryId")]
    pub category_id: Uuid
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct GetCategoriesQuery {
    #[validate(range(min=1,message="page must be greater than or equal to one"))]
    pub page: Option<i64>,
    # [validate(range(min=1,max=100,message="limit must greater than 1 or equal to 1 and less than 100"))]
    pub limit: Option<i64>
}

#[derive(Debug,Serialize,Deserialize,Validate)]
pub struct UpdateCategoryDto {
    #[validate(length(min=1,max=100,message = "category must greater than or equal to 1 and less than 100 characters"))]
    pub name: Option<String>,
    #[validate(length(min=1,max=255,message = "description must be greater than or equal to 1 and less than 255 characters"))]
    pub description: Option<String>,
    #[serde(rename="categoryId")]
    pub category_id: Uuid
}

