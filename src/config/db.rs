// use diesel::{sqlite::SqliteConnection, Connection};
// use dotenvy::dotenv;
// use std::env;

// pub fn config_db() {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let db = SqliteConnection::establish(database_url.as_str())
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
// }