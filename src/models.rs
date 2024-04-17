use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

#[derive(Clone, Debug, FromRow, Deserialize, Serialize)]
pub struct DishFormDb {
    pub dish_id: i32,
    pub dish_name: String,
    pub dish_type_id: i32,
    pub dish_base_id: i32,
    pub dish_image: String
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize)]
pub struct UserFromDb{
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub patronymic: String,
    pub date_of_birthday: chrono::NaiveDate,
    pub login: String,
    pub user_password: String,
    pub phone: String,
    pub adress: String,
    pub user_role_id: i32
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize)]
pub struct TypeFromDb{
    pub type_id: i32,
    pub type_name: String
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize)]
pub struct BaseFromDb{
    pub base_id: i32,
    pub base_name: String,
    pub base_exit: i32
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize)]
pub struct ProdFromDb{
    pub prod_id: i32,
    pub prod_name: String,
    pub prod_protein: i32,
    pub prod_fats: i32,
    pub prod_carboh: i32
}

#[derive(Clone, Debug, FromRow, Deserialize, Serialize)]
pub struct StructFromDb{
    pub dishes_id: i32,
    pub products_id: String,
    pub weight: i32
}