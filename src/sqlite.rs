use sqlx::{migrate::MigrateDatabase, Connection, Sqlite, SqliteConnection};

mod session;
use session::Session;

const URI: &str = "sqlite://bacik.db";
pub struct SQLite {
    connection: SqliteConnection
}

impl SQLite {

    pub async fn init () -> Result<Self, sqlx::Error> {

        if !Sqlite::database_exists(URI).await? {
           Sqlite::create_database(URI).await?;
        }

        let mut connection = SqliteConnection::connect(URI).await?;

        Session::init( &mut connection ).await?;

        let session = Session::create(&mut connection, Some("foo")).await?;
        println!("{:?}", session);
        drop(session);

        Ok(
            Self{ connection }
        )
    }

}
