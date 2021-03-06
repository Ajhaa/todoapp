mod todo;

use std::env;
use dotenv::dotenv;
use std::convert::Infallible;
use warp::Filter;
use warp::reply;
use sqlx::postgres::{PgPool};
use std::collections::HashMap;
use todo::Todo;

fn with_db(db_pool: PgPool) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

async fn handle_todo(pool: PgPool, id: String) -> Result<impl warp::Reply, Infallible> {
    let todo = Todo::find_one(&pool, &id).await.unwrap();   
    Ok(reply::json(&todo))
}

async fn post_todo(pool: PgPool, content: String) -> Result<impl warp::Reply, Infallible> {
    let todo = Todo::create(&pool, &content).await.unwrap();
    Ok(reply::json(&todo))
}

async fn list_todos(pool: PgPool) -> Result<impl warp::Reply, Infallible> {
    let todos = Todo::find_all(&pool).await.unwrap();
    Ok(reply::json(&todos))
}

async fn toggle_todo(pool: PgPool, id: String) -> Result<impl warp::Reply, Infallible> {
    let todo = Todo::toggle_done(&pool, &id).await.unwrap();
    Ok(reply::json(&todo))
}

async fn delete_todo(pool: PgPool, id: String) -> Result<impl warp::Reply, Infallible> {
    let todo = Todo::delete(&pool, &id).await.unwrap();
    Ok(reply::json(&todo))
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let with_cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "GET", "DELETE", "PATCH"])
        .allow_header("Content-Type");

    let pg_pool = PgPool::builder()
        .max_size(5)
        .build(&env::var("DATABASE_URL").unwrap()).await?;
    
    let todos = warp::get()
        .and(warp::path("todos"))
        .and(with_db(pg_pool.clone()))
        .and_then(|pool| list_todos(pool));

    let toggle = warp::patch()
        .and(warp::path!("todos" / String))
        .and(with_db(pg_pool.clone()))
        .and_then(|id, pool: PgPool| toggle_todo(pool, id));

    let a_todo = warp::get()
        .and(warp::path!("todos" / String))
        .and(with_db(pg_pool.clone()))
        .and_then(|id, pool: PgPool| handle_todo(pool, id));

    let new_todo = warp::post()
        .and(warp::path("todos"))
        .and(with_db(pg_pool.clone()))
        .and(warp::body::json())
        .and_then(|pool: PgPool, body: HashMap<String, String>| post_todo(pool, body.get("content").unwrap().to_owned()));

    let del_todo = warp::delete()
        .and(warp::path!("todos" / String))
        .and(with_db(pg_pool.clone()))
        .and_then(|id, pool| delete_todo(pool, id));

    let all_routes = a_todo.or(del_todo).or(toggle).or(new_todo).or(todos);


    warp::serve(all_routes.with(with_cors))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}