// use parking_lot::RwLock;
// use std::collections::HashMap;
// use std::sync::Arc;
use sqlx::postgres::{PgPoolOptions, PgPool, PgRow};
use sqlx::Row;

use crate::types::{
    answer::Answer,
    question::{Question, QuestionId},
};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool, 
}

impl Store {
    pub async fn new(db_url:&str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url).await {
            Ok(pool) => pool,
            Err(e) => panic!("Cound't establish DB connection")
            };
        Store {
            connection: db_pool,
        }
    }

}
