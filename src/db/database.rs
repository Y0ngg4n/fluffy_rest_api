use scylla::{IntoTypedRows, Session, SessionBuilder, QueryResult};
use std::error::Error;
use scylla::transport::errors::{NewSessionError, QueryError};
use std::sync::Arc;
use scylla::transport::Compression;

pub async fn connect() -> Result<Session, Box<dyn Error>> {
    println!("Connecting to scylla db ...");
    let uri = std::env::var("SCYLLA_URI")
        .unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    let session = SessionBuilder::new()
        .known_node(uri)
        .compression(Some(Compression::Snappy))
        .build()
        .await?;
    // match session{
    //     Ok(_) => {println!("Connected to database!")}
    //     Err(e) => {panic!("Could not connect to database!")}
    // }
    Ok(session)
}

pub async fn create_keyspace_and_tables(session_arc: &Arc<Session>) -> Result<(), Box<dyn Error>>{
    println!("Creating Keyspace and tables...");
    let session = Arc::clone(session_arc);
    session
        .query(
            "CREATE KEYSPACE IF NOT EXISTS fluffy_board WITH REPLICATION = \
            {'class' : 'SimpleStrategy', 'replication_factor' : 1}",
            &[],
        )
        .await?;
    // Account
    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.account ( \
            id UUID, \
            name text, \
            email text, \
            password text,\
            created timestamp, \
            PRIMARY KEY(id)\
            )",
            &[],
        )
        .await?;
    session
        .query(
            "CREATE INDEX IF NOT EXISTS ON fluffy_board.account(email)",
            &[],
        )
        .await?;
    // Filemanager
    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.whiteboard (\
            id UUID, \
            owner UUID, \
            directory UUID, \
            name text,\
            password text,\
            created timestamp, \
            view_id UUID, \
            edit_id UUID, \
            data UUID, \
            PRIMARY KEY(id)\
            )",
            &[],
        )
        .await?;
    session
        .query(
            "CREATE INDEX IF NOT EXISTS ON fluffy_board.whiteboard(owner)",
            &[],
        )
        .await?;
    session
        .query(
            "CREATE INDEX IF NOT EXISTS ON fluffy_board.whiteboard(directory)",
            &[],
        )
        .await?;
    // Filemanager ext
    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.ext_whiteboard (\
            id UUID, \
            account UUID, \
            directory UUID, \
            name text, \
            edit Boolean, \
            data UUID, \
            PRIMARY KEY(id) \
            )",
            &[],
        )
        .await?;
    session
        .query(
            "CREATE INDEX IF NOT EXISTS ON fluffy_board.ext_whiteboard(account)",
            &[],
        )
        .await?;
    session
        .query(
            "CREATE INDEX IF NOT EXISTS ON fluffy_board.ext_whiteboard(directory)",
            &[],
        )
        .await?;
    // Directory
    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.wb_directory (\
            id UUID,\
            owner UUID, \
            parent UUID, \
            filename TEXT, \
            created timestamp, \
            PRIMARY KEY(id) \
            )",
            &[],
        )
        .await?;
    session
        .query(
            "CREATE INDEX IF NOT EXISTS ON fluffy_board.wb_directory(owner)",
            &[],
        )
        .await?;
    session
        .query(
            "CREATE INDEX IF NOT EXISTS ON fluffy_board.wb_directory(parent)",
            &[],
        )
        .await?;


    // #############################
    // ToolbarOptions
    // #############################
    // Pencil
    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.toolbar_options_pencil (\
            owner UUID, \
            color_presets List<Text>, \
            stroke_width double, \
            selected_color int, \
            PRIMARY KEY(owner) \
            )",
            &[],
        )
        .await?;
    // Highlighter
    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.toolbar_options_highlighter (\
            owner UUID, \
            color_presets List<Text>, \
            stroke_width double, \
            selected_color int, \
            PRIMARY KEY(owner) \
            )",
            &[],
        )
        .await?;
    // Eraser
    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.toolbar_options_eraser (\
            owner UUID, \
            stroke_width double, \
            PRIMARY KEY(owner) \
            )",
            &[],
        )
        .await?;
    // Straight Line
    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.toolbar_options_straight_line (\
            owner UUID, \
            color_presets List<Text>, \
            stroke_width double, \
            selected_color int, \
            selected_cap int, \
            PRIMARY KEY(owner) \
            )",
            &[],
        )
        .await?;
    // Figure
    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.toolbar_options_figure (\
            owner UUID, \
            color_presets List<Text>, \
            stroke_width double, \
            selected_color int, \
            selected_figure int, \
            selected_fill int, \
            PRIMARY KEY(owner) \
            )",
            &[],
        )
        .await?;
    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.toolbar_options_background (\
            owner UUID, \
            stroke_width double, \
            selected_background int, \
            PRIMARY KEY(owner) \
            )",
            &[],
        )
        .await?;

    // #############################
    // Whiteboard Data
    // #############################
    // Scribble
    session
        .query(
            "CREATE TYPE IF NOT EXISTS fluffy_board.drawpoint (dx double, dy double)",
            &[],
        )
        .await?;
    session
        .query(
            "CREATE TABLE IF NOT EXISTS fluffy_board.wb_scribble (\
            id UUID, \
            whiteboard UUID, \
            selected_figure_type_toolbar int, \
            stroke_width double, \
            stroke_cap double, \
            color text, \
            points List<frozen <drawpoint>>, \
            left_extremity double, \
            right_extremity double, \
            top_extremity double, \
            bottom_extremity double, \
            PRIMARY KEY(id) \
            )",
            &[],
        )
        .await?;

    Ok(())
}
