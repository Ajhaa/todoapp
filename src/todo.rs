use serde::{Serialize, Deserialize};
use sqlx::postgres::{PgPool, PgQueryAs};
use sqlx::{Cursor, FromRow};

#[derive(Serialize, Deserialize, Debug)]
#[derive(sqlx::FromRow)]
pub struct Todo {
    id: String,
    content: String,
    done: bool,
}

impl Todo {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
        let mut todos: Vec<Todo> = Vec::new();
        
        let mut cursor = sqlx::query("SELECT id::text, content, done FROM todos")
            .fetch(pool);
        
        while let Some(row) = cursor.next().await? {
            todos.push(Todo::from_row(&row).unwrap());
        }
        
        Ok(todos)
    }

    pub async fn toggle_done(pool: &PgPool, id: &String) -> Result<Todo, sqlx::Error> {
        sqlx::query_as("UPDATE todos SET done = NOT done WHERE id = $1::uuid RETURNING id::text, content, done ")
            .bind(id)
            .fetch_one(pool).await
    }

    pub async fn find_one(pool: &PgPool, id: &String) -> Result<Todo, sqlx::Error> {
        sqlx::query_as("SELECT id::text, content, done FROM todos WHERE id = $1::uuid")
            .bind(id)
            .fetch_one(pool).await
    }

    pub async fn create(pool: &PgPool, content: &String) -> Result<Todo, sqlx::Error> {
        sqlx::query_as("INSERT INTO todos (content, done) VALUES ($1, 'false') RETURNING id::text, content, done")
            .bind(content)
            .fetch_one(pool).await
    }

    pub async fn delete(pool: &PgPool, id: &String) -> Result<Todo, sqlx::Error> {
        sqlx::query_as("DELETE FROM todos WHERE id = $1::uuid RETURNING id::text, content, done")
            .bind(id)
            .fetch_one(pool).await
    }
}
