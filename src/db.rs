use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
use std::env;
use dotenv::dotenv;

pub async fn init_pool() -> Pool {
    dotenv().ok();
    let mut cfg = Config::new();
    cfg.dbname = Some("mydb".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("password".to_string());
    cfg.host = Some("localhost".to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    
    cfg.create_pool(NoTls).unwrap()
}
