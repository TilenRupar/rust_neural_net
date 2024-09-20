use serde::Serialize;
use sqlx::Sqlite;
use std::sync::OnceLock;
// use core::cell::OnceCell;
use sqlx::Pool;
use std::path::Path;
use sqlx::SqlitePool;
use std::fs::File;
use sqlx::FromRow;
use serde::Deserialize;
use chrono::Utc;
use chrono::DateTime;

// use sqlx::error::Error as SqlxError; // Import the base Error type from sqlx
pub static DB_POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

pub async fn initialize() -> Result<(), String> {
    // Creates the database file if it doesnt exist
    let database_path: &str = "database.sqlite";
    if Path::new(database_path).exists() == false {
        File::create(database_path).map_err(|e| e.to_string())?;
    }
    // Creates a new pool
    let pool = SqlitePool::connect("./database.sqlite")
        .await        
        .map_err(|e| format!("{:?}", e))?;
    DB_POOL
        .set(pool)
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

pub async fn fetch_all_candles(
    pair: String,
  ) -> Result<Vec<Candle>, String> {
    let connection = DB_POOL.get().unwrap();
    let candles: Vec<Candle> = sqlx::query_as("SELECT * FROM candles WHERE asset = ?1 LIMIT 7200")
      .bind(pair.to_string())
      .fetch_all(connection)
      .await
      .map_err(|e| e.to_string())?;
    Ok(candles)
  }
  
#[derive(FromRow, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct Candle {
  pub open_time: DateTime<Utc>,
  pub close_time: DateTime<Utc>,
  pub open: f64,
  pub high: f64,
  pub low: f64,
  pub close: f64,
  pub volume: f64,
  pub trade_count: i64,
}


pub fn get_candle_vector<F>(candles: &Vec<Candle>, field_selector: F) -> Vec<f64> 
where
    F: Fn(&Candle) -> f64,
{
    candles.iter().map(|candle| field_selector(candle)).collect()
}   