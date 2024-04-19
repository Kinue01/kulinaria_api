use axum::{ extract::{ State, Json }, http::StatusCode };
use sqlx::{Acquire, PgPool, Pool };

use crate::{ 
    errors::MyError, models::{ Base, Dish, Order, OrderCart, Paytype, Product, Structure, Type, User }
};

pub async fn get_dishes(State(pool): State<PgPool>) -> Result<Json<Vec<Dish>>, MyError> {
    
    let dishes: Vec<Dish> = sqlx::query_as("select * from dishes order by dish_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(dishes))
    
}

pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, MyError> {
    
    let users: Vec<User> = sqlx::query_as("select * from tb_user order by user_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(users))
    
}

pub async fn get_types(State(pool): State<PgPool>) -> Result<Json<Vec<Type>>, MyError> {
    
    let types: Vec<Type> = sqlx::query_as("select * from dish_types order by type_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(types))
    
}

pub async fn get_bases(State(pool): State<PgPool>) -> Result<Json<Vec<Base>>, MyError> {
    
    let types: Vec<Base> = sqlx::query_as("select * from dish_base order by base_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(types))
    
}

pub async fn get_prods(State(pool): State<PgPool>) -> Result<Json<Vec<Product>>, MyError> {
    
    let prods: Vec<Product> = sqlx::query_as("select * from products order by prod_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(prods))
    
}

pub async fn get_struct_by_dish_id(State(pool): State<PgPool>, Json(dish): Json<Dish>) -> Result<Json<Vec<Structure>>, MyError> {
    
    let struc: Vec<Structure> = sqlx::query_as("select * from scructure where dishes_id = $1")
    .bind(&dish.dish_id)
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(struc))
    
}

pub async fn add_dish(State(pool): State<PgPool>, Json(dish): Json<Dish>) -> Result<StatusCode, MyError> {

    let _ = sqlx::query("insert into dishes (dish_name, dish_type_id, dish_base_id, dish_image) values ($1, $2, $3, $4)")
    .bind(&dish.dish_name).bind(&dish.dish_type_id).bind(&dish.dish_base_id).bind(&dish.dish_image)
    .execute(&pool)
    .await
    .map_err(MyError::DBError);

    Ok(StatusCode::CREATED)

}

pub async fn update_dish(State(pool): State<PgPool>, Json(dish): Json<Dish>) -> Result<StatusCode, MyError> {

    let _ = sqlx::query("update dishes set dish_name = $1, dish_type_id = $2, dish_base_id = $3, dish_image = $4 where dish_id = $5")
    .bind(&dish.dish_name).bind(&dish.dish_type_id).bind(&dish.dish_base_id).bind(&dish.dish_image).bind(&dish.dish_id)
    .execute(&pool)
    .await
    .map_err(MyError::DBError);

    Ok(StatusCode::CREATED)

}

pub async fn delete_dish(State(pool): State<PgPool>, Json(dish): Json<Dish>) -> Result<StatusCode, MyError> {

    let _ = sqlx::query("delete from dishes where dish_id = $1")
    .bind(&dish.dish_id)
    .execute(&pool)
    .await
    .map_err(MyError::DBError);

    Ok(StatusCode::CREATED)

}

pub async fn get_orders_by_user_id(State(pool): State<PgPool>, Json(user): Json<User>) -> Result<Json<Vec<Order>>, MyError> {
    
    let order: Vec<Order> = sqlx::query_as("select * from tb_order where user_id = $1")
    .bind(&user.user_id)
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(order))
    
}

pub async fn get_cart_by_order_id(State(pool): State<PgPool>, Json(order): Json<Order>) -> Result<Json<Vec<OrderCart>>, MyError> {
    
    let cart: Vec<OrderCart> = sqlx::query_as("select * from tb_order_cart where cart_order_id = $1")
    .bind(&order.order_id)
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(cart))
    
}

pub async fn get_paytypes(State(pool): State<PgPool>) -> Result<Json<Vec<Paytype>>, MyError> {
    
    let types: Vec<Paytype> = sqlx::query_as("select * from tb_paytype")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(types))
    
}

pub async fn add_order(State(pool): State<PgPool>, Json(order): Json<Order>, Json(cart): Json<Vec<OrderCart>>) -> Result<StatusCode, MyError> {

    let mut trans = Pool::begin(&pool).await.unwrap();

    let mut i = 1;

    trans.begin().await.unwrap();

    let _ = sqlx::query("insert into tb_order (order_address, order_user_id, order_date, order_paytype_id) values ($1, $2, $3, $4)")
    .bind(&order.order_address).bind(&order.order_user_id).bind(&order.order_date).bind(&order.order_paytype_id)
    .execute(&pool)
    .await
    .map_err(MyError::DBError);

    while i < cart.len() {
        let _ = sqlx::query("insert into tb_order_cart (cart_order_id, cart_prod_id, cart_prod_count) values ($1, $2, $3)")
        .bind(&cart[i].cart_order_id).bind(&cart[i].cart_prod_id).bind(&cart[i].cart_prod_count)
        .execute(&pool)
        .await
        .map_err(MyError::DBError);
        i += 1;
    }

    trans.commit().await.unwrap();

    Ok(StatusCode::CREATED)

}