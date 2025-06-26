use actix_web::{web, HttpResponse};

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;

// DB Types.
// If some user wants to change MySQL with PostgresSQL or SQLite
// Should only change SqlConnection type and its import
pub type SqlConnection = SqliteConnection;
pub type SqlPool = Pool<ConnectionManager<SqlConnection>>;
pub type SqlPooledConnection = PooledConnection<ConnectionManager<SqlConnection>>;

pub fn connect_database() -> SqlPool {
    let db_config = crate::configuration::SERVER_CONFIG.clone().database;
    let manager = ConnectionManager::<SqlConnection>::new(db_config.db_url);
    Pool::builder()
        .max_size(db_config.pool_size)
        .build(manager)
        .expect("Error during connection to Data Base!")
}

pub fn pool_handler(
    connection_pool: Option<&web::Data<SqlPool>>,
) -> Result<SqlPooledConnection, HttpResponse> {
    connection_pool
        .ok_or(
            HttpResponse::InternalServerError().json("No Data Base connection exists!".to_owned()),
        )
        .and_then(|pool| {
            pool.get()
                .map_err(|error| HttpResponse::InternalServerError().json(error.to_string()))
        })
}
