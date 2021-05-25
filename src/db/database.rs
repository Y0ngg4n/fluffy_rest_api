use scylla::{IntoTypedRows, Session, SessionBuilder, QueryResult};
use std::error::Error;
use scylla::transport::errors::{NewSessionError, QueryError};

pub async fn connect() -> Result<Session, Box<dyn Error>> {
    println!("Connecting to scylla db ...");
    let uri = std::env::var("SCYLLA_URI")
        .unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    let session = SessionBuilder::new()
        .known_node(uri)
        .build()
        .await?;
    // match session{
    //     Ok(_) => {println!("Connected to database!")}
    //     Err(e) => {panic!("Could not connect to database!")}
    // }
    Ok(session)
}

pub async fn create_keyspace_and_tables(session: &Session) -> Result<(), Box<dyn Error>>{
    println!("Creating Keyspace and tables...");
    session
        .query(
            "CREATE KEYSPACE IF NOT EXISTS fluffy_board WITH REPLICATION = \
            {'class' : 'SimpleStrategy', 'replication_factor' : 1}",
            &[],
        )
        .await?;

    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.whiteboard (\
            id UUID primary key,\
            name text,\
            owner text,\
            group text, \
            password text,\
            created timestamp, \
            retention timestamp \
            )",
            &[],
        )
        .await?;
    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.account (\
            id UUID primary key,\
            name text,\
            email text, \
            password text,\
            created timestamp, \
            )",
            &[],
        )
        .await?;
    Ok(())
}
