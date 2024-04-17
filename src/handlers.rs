use axum::{ extract::{ State, Json }, http::StatusCode };
use sqlx::PgPool;

use crate::{ 
    errors::MyError, models::{ BaseFromDb, DishFormDb, ProdFromDb, StructFromDb, TypeFromDb, UserFromDb }
};

pub async fn get_dishes(State(pool): State<PgPool>) -> Result<Json<Vec<DishFormDb>>, MyError> {
    
    let dishes: Vec<DishFormDb> = sqlx::query_as("select * from dishes order by dish_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(dishes))
    
}

pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<UserFromDb>>, MyError> {
    
    let users: Vec<UserFromDb> = sqlx::query_as("select * from tb_user order by user_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(users))
    
}

pub async fn get_types(State(pool): State<PgPool>) -> Result<Json<Vec<TypeFromDb>>, MyError> {
    
    let types: Vec<TypeFromDb> = sqlx::query_as("select * from dish_types order by type_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(types))
    
}

pub async fn get_bases(State(pool): State<PgPool>) -> Result<Json<Vec<BaseFromDb>>, MyError> {
    
    let types: Vec<BaseFromDb> = sqlx::query_as("select * from dish_base order by base_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(types))
    
}

pub async fn get_prods(State(pool): State<PgPool>) -> Result<Json<Vec<ProdFromDb>>, MyError> {
    
    let prods: Vec<ProdFromDb> = sqlx::query_as("select * from products order by prod_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(prods))
    
}

pub async fn get_struct_by_dish_id(State(pool): State<PgPool>, Json(dish): Json<DishFormDb>) -> Result<Json<Vec<StructFromDb>>, MyError> {
    
    let struc: Vec<StructFromDb> = sqlx::query_as("select * from scructure where dishes_id = $1")
    .bind(&dish.dish_id)
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(struc))
    
}

pub async fn add_dish(State(pool): State<PgPool>, Json(dish): Json<DishFormDb>) -> Result<StatusCode, MyError> {

    let _ = sqlx::query("insert into dishes (dish_name, dish_type_id, dish_base_id, dish_image) values ($1, $2, $3, $4)")
    .bind(&dish.dish_name).bind(&dish.dish_type_id).bind(&dish.dish_base_id).bind(&dish.dish_image)
    .execute(&pool)
    .await
    .map_err(MyError::DBError);

    Ok(StatusCode::CREATED)

}

pub async fn update_dish(State(pool): State<PgPool>, Json(dish): Json<DishFormDb>) -> Result<StatusCode, MyError> {

    let _ = sqlx::query("update dishes set dish_name = $1, dish_type_id = $2, dish_base_id = $3, dish_image = $4 where dish_id = $5")
    .bind(&dish.dish_name).bind(&dish.dish_type_id).bind(&dish.dish_base_id).bind(&dish.dish_image).bind(&dish.dish_id)
    .execute(&pool)
    .await
    .map_err(MyError::DBError);

    Ok(StatusCode::CREATED)

}

pub async fn delete_dish(State(pool): State<PgPool>, Json(dish): Json<DishFormDb>) -> Result<StatusCode, MyError> {

    let _ = sqlx::query("delete from dishes where dish_id = $1")
    .bind(&dish.dish_id)
    .execute(&pool)
    .await
    .map_err(MyError::DBError);

    Ok(StatusCode::CREATED)

}