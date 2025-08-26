// use rusqlite::{ params, Connection, Result };

pub fn config_db() {
    // let conn = Connection::open("data/database.db");

    // match conn {
    //     Ok(connection) => {
    //         let query = connection.execute(
    //     r#"
    //         IF (EXISTS (SELECT * 
    //              FROM INFORMATION_SCHEMA.TABLES 
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
    //         "#, ());
    //     }
    //     Err(_) => {}
    // }
}