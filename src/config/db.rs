// use sqlx_mysql::{MySqlConnection, MySqlPool};
// use sqlx;

pub async fn config_db() {
    // let conn = MySqlPool::connect("mysql://Golbezz:yj*wHnFu%9jkEo@localhost:3306/restapitest").await;

    // match conn {
    //     Ok(pool) => {
    //         println!("Connection successful!");
    //         let result = sqlx::query(r#"
    //         IF (EXISTS (SELECT * 
    //              FROM restapitest.TABLES 
    //              WHERE TABLE_NAME = 'users'))
    //         BEGIN
    //             CREATE TABLE users (
    //                 id INTEGER PRIMARY KEY,
    //                 name TEXT NOT NULL,
    //                 email TEXT NOT NULL,
    //                 password TEXT NOT NULL,
    //                 ip TEXT NOT NULL,
    //                 session TEXT NOT NULL,
    //             )
    //         END
    //         "#).execute(&pool).await;

    //         if let Ok(res) = result {
    //             println!("Query successful!");
    //         }
    //         else if let Err(res) = result {
    //             println!("Query failed... {}", res);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Connection failed... {}", e);
    //     }
    // }
}