use scylla::{Session, SessionBuilder};
use std::error::Error;
use std::sync::Arc;
use scylla::transport::Compression;

pub async fn migrate(session_arc: &Arc<Session>) -> Result<(), Box<dyn Error>> {
    println!("Migrating Database Scheme");
    let session = Arc::clone(session_arc);

    /** Add color_presets in toolbar_options_background **/
    session
        .query(
            "ALTER TABLE fluffy_board.toolbar_options_background ADD color_presets List<Text>",
            &[],
        )
        .await.ok();
    Ok(())
}