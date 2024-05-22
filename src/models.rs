use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use utoipa::ToSchema;

#[derive(Clone, Debug, FromRow, Deserialize, Serialize, ToSchema)]
pub struct Dish {
    pub dish_id: i32,
    pub dish_name: String,
    pub dish_type_id: i32,
    pub dish_base_id: i32,
    pub dish_image: String
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize, ToSchema)]
pub struct User {
    pub user_id: i32,
    pub user_firstname: String,
    pub user_lastname: String,
    pub user_patronymic: String,
    pub user_birthday: chrono::NaiveDate,
    pub user_login: String,
    pub user_password: String,
    pub user_phone: String,
    pub user_address: String,
    pub user_role_id: i32
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize, ToSchema)]
pub struct Role {
    pub role_id: i32,
    pub role_name: String
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize, ToSchema)]
pub struct UserData {
    pub user_id: i32,
    pub user_passport_ser: i32,
    pub user_passport_num: i32,
    pub user_who_issued: String,
    pub user_issue_date: chrono::NaiveDate,
    pub user_email: String
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize, ToSchema)]
pub struct Type {
    pub type_id: i32,
    pub type_name: String
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize, ToSchema)]
pub struct Base {
    pub base_id: i32,
    pub base_name: String,
    pub base_exit: i32
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize, ToSchema)]
pub struct Product {
    pub prod_id: i32,
    pub prod_name: String,
    pub prod_protein: i32,
    pub prod_fats: i32,
    pub prod_carboh: i32
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize, ToSchema)]
pub struct Structure {
    pub struct_dish_id: i32,
    pub struct_product_id: String,
    pub struct_weight: i32
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize, ToSchema)]
pub struct Paytype {
    pub type_id: i32,
    pub type_name: String
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize, PartialEq, Eq, Hash, ToSchema)]
pub struct Order {
    pub order_id: i32,
    pub order_user_id: i32,
    pub order_address: String,
    pub order_date: chrono::NaiveDate,
    pub order_paytype_id: i32
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize, ToSchema)]
pub struct OrderCart {
    pub cart_order_id: i32,
    pub cart_prod_id: i32,
    pub cart_prod_count: i32
}
