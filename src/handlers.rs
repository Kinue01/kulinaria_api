use std::collections::HashMap;

use axum::{ debug_handler, extract::{ Json, State, Path }, http::StatusCode };
use sqlx::{ Acquire, PgPool };

use crate::{ 
    errors::MyError, models::{ Base, Dish, Order, OrderCart, Paytype, Product, Structure, Type, User }
};

#[utoipa::path(
    get,
    path = "/dishes",
    responses(
        (status = 200, description = "Success", body = Vec<Dish>),
        (status = NOT_FOUND, description = "Dishes not found")
    )
)]
pub async fn get_dishes(State(pool): State<PgPool>) -> Result<Json<Vec<Dish>>, MyError> {
    
    let mut dishes: Vec<Dish> = sqlx::query_as("select * from tb_dish order by dish_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    let res: Vec<Dish> = dishes.iter_mut().map(|dish| if dish.dish_image.is_empty() { dish.dish_image = String::from("default.png"); return dish.clone(); } else { return dish.clone(); }).collect();

    Ok(Json(res))
    
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Success", body = Vec<User>),
        (status = NOT_FOUND, description = "Users not found")
    )
)]
pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, MyError> {
    
    let users: Vec<User> = sqlx::query_as("select * from tb_user order by user_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(users))
    
}

#[utoipa::path(
    get,
    path = "/types",
    responses(
        (status = 200, description = "Success", body = Vec<Type>),
        (status = NOT_FOUND, description = "Types not found")
    )
)]
pub async fn get_types(State(pool): State<PgPool>) -> Result<Json<Vec<Type>>, MyError> {
    
    let types: Vec<Type> = sqlx::query_as("select * from tb_dish_type order by type_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(types))
    
}

#[utoipa::path(
    get,
    path = "/bases",
    responses(
        (status = 200, description = "Success", body = Vec<Base>),
        (status = NOT_FOUND, description = "Bases not found")
    )
)]
pub async fn get_bases(State(pool): State<PgPool>) -> Result<Json<Vec<Base>>, MyError> {
    
    let types: Vec<Base> = sqlx::query_as("select * from tb_dish_base order by base_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(types))
    
}

#[utoipa::path(
    get,
    path = "/prods",
    responses(
        (status = 200, description = "Success", body = Vec<Product>),
        (status = NOT_FOUND, description = "PRoducts not found")
    )
)]
pub async fn get_prods(State(pool): State<PgPool>) -> Result<Json<Vec<Product>>, MyError> {
    
    let prods: Vec<Product> = sqlx::query_as("select * from tb_product order by prod_id")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(prods))
    
}

#[utoipa::path(
    get,
    path = "/struct_by_dish/{id}",
    responses(
        (status = 200, description = "Success", body = Vec<Structure>),
        (status = NOT_FOUND, description = "Structure not found")
    ),
    params(
        ("id" = i32, Path, description = "Get dish structure by dish id")
    )
)]
pub async fn get_struct_by_dish_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Vec<Structure>>, MyError> {
    
    let struc: Vec<Structure> = sqlx::query_as("select * from tb_structure where dishes_id = $1")
    .bind(&id)
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(struc))
    
}

#[utoipa::path(
    post,
    path = "/add_dish",
    request_body = Dish,
    responses(
        (status = 201, description = "Success"),
        (status = 500, description = "Can`t create dish")
    )
)]
pub async fn add_dish(State(pool): State<PgPool>, Json(dish): Json<Dish>) -> Result<StatusCode, MyError> {

    let _ = sqlx::query("insert into tb_dish (dish_name, dish_type_id, dish_base_id, dish_image) values ($1, $2, $3, $4)")
    .bind(&dish.dish_name).bind(&dish.dish_type_id).bind(&dish.dish_base_id).bind(&dish.dish_image)
    .execute(&pool)
    .await
    .map_err(MyError::DBError);

    Ok(StatusCode::CREATED)

}

#[utoipa::path(
    put,
    path = "/update_dish",
    request_body = Dish,
    responses(
        (status = 200, description = "Success"),
        (status = 500, description = "Dishes not found")
    )
)]
pub async fn update_dish(State(pool): State<PgPool>, Json(dish): Json<Dish>) -> Result<StatusCode, MyError> {

    let _ = sqlx::query("update tb_dish set dish_name = $1, dish_type_id = $2, dish_base_id = $3, dish_image = $4 where dish_id = $5")
    .bind(&dish.dish_name).bind(&dish.dish_type_id).bind(&dish.dish_base_id).bind(&dish.dish_image).bind(&dish.dish_id)
    .execute(&pool)
    .await
    .map_err(MyError::DBError);

    Ok(StatusCode::OK)

}

#[utoipa::path(
    delete,
    path = "/delete_dish",
    responses(
        (status = 200, description = "Success"),
        (status = 500, description = "Dishes not found")
    )
)]
pub async fn delete_dish(State(pool): State<PgPool>, Json(dish): Json<Dish>) -> Result<StatusCode, MyError> {

    let _ = sqlx::query("delete from tb_dish where dish_id = $1")
    .bind(&dish.dish_id)
    .execute(&pool)
    .await
    .map_err(MyError::DBError);

    Ok(StatusCode::OK)

}

#[utoipa::path(
    get,
    path = "/order_by_user/{id}",
    responses(
        (status = 200, description = "Success", body = Vec<Order>),
        (status = NOT_FOUND, description = "Orders not found")
    ),
    params(
        ("id" = i32, Path, description = "Get orders by user id")
    )
)]
pub async fn get_orders_by_user_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Vec<Order>>, MyError> {
    
    let order: Vec<Order> = sqlx::query_as("select * from tb_order where user_id = $1")
    .bind(&id)
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(order))
}

#[utoipa::path(
    get,
    path = "/cart_by_order",
    responses(
        (status = 200, description = "Success", body = Vec<OrderCart>),
        (status = NOT_FOUND, description = "Dishes not found")
    ),
    params(
        ("id" = i32, Path, description = "Get orders by user id")
    )
)]
pub async fn get_cart_by_order_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Vec<OrderCart>>, MyError> {
    
    let cart: Vec<OrderCart> = sqlx::query_as("select * from tb_order_cart where cart_order_id = $1")
    .bind(&id)
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(cart))
    
}

#[utoipa::path(
    get,
    path = "/paytypes",
    responses(
        (status = 200, description = "Success", body = Vec<Paytype>),
        (status = NOT_FOUND, description = "Dishes not found")
    )
)]
pub async fn get_paytypes(State(pool): State<PgPool>) -> Result<Json<Vec<Paytype>>, MyError> {
    
    let types: Vec<Paytype> = sqlx::query_as("select * from tb_paytype")
    .fetch_all(&pool)
    .await
    .map_err(MyError::DBError)?;

    Ok(Json(types))
    
}

#[utoipa::path(
    post,
    path = "/dishes",
    request_body = HashMap<Order, Vec<OrderCart>>,
    responses(
        (status = 201, description = "Success"),
        (status = 500, description = "Dishes not found")
    )
)]
pub async fn add_order(State(pool): State<PgPool>, Json(order): Json<HashMap<Order, Vec<OrderCart>>>) -> Result<StatusCode, MyError> {

    let mut trans = PgPool::begin(&pool).await.unwrap();

    let mut i = 1;

    let ord = order.keys().next().unwrap();
    let cart = order.values().next().unwrap();

    trans.begin().await.unwrap();

    let _ = sqlx::query("insert into tb_order (order_address, order_user_id, order_date, order_paytype_id) values ($1, $2, $3, $4)")
    .bind(&ord.order_address).bind(&ord.order_user_id).bind(&ord.order_date).bind(&ord.order_paytype_id)
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
