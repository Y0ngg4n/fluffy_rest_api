use scylla::{IntoTypedRows, Session, SessionBuilder, QueryResult, BatchResult};
use std::error::Error;
use scylla::transport::errors::{NewSessionError, QueryError};
use std::sync::Arc;
use scylla::frame::value::Timestamp;
use uuid::Uuid;
use scylla::frame::response::result::Row;
use crate::db::models::file::{NewCreateDirectory, InputRenameDirectory, NewRenameDirectory, InputDeleteDirectory, NewCreateWhiteboard, NewRenameWhiteboard, InputDeleteWhiteboard, NewGetDirectory, NewGetWhiteboard, NewDeleteDirectory, ReadGetWhiteboard, NewDeleteWhiteboard};
use scylla::batch::Batch;
use scylla::query::Query;
use scylla::prepared_statement::PreparedStatement;
use crate::db::models::offline::{InputImportScribble, InputImport, InputImportUpload, InputImportTextItem};

pub async fn import_scribbles(session_arc: &Arc<Session>, scribbles: &Vec<InputImportScribble>, whiteboard: &Uuid) -> Result<(), Box<dyn Error>> {
    // Create a batch statement
    let session = Arc::clone(session_arc);
    let mut batch: Batch = Default::default();
    let mut values = Vec::new();
    for scribble in scribbles {
        // Add a simple query to the batch using its text
        let prepared: PreparedStatement = session.prepare("INSERT INTO fluffy_board.wb_scribble(id, whiteboard, selected_figure_type_toolbar, stroke_width, stroke_cap, painting_style, color, points, left_extremity, right_extremity, top_extremity, bottom_extremity) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)").await?;
        batch.append_statement(prepared);
        values.push((&scribble.uuid, whiteboard, &scribble.selected_figure_type_toolbar, &scribble.stroke_width, &scribble.stroke_cap, &scribble.painting_style, &scribble.color, &scribble.points, &scribble.left_extremity, &scribble.right_extremity, &scribble.top_extremity, &scribble.bottom_extremity));
    }
    // Run the batch, doesn't return any rows
    session.batch(&batch, &values).await?;
    Ok(())
}

pub async fn import_uploads(session_arc: &Arc<Session>, uploads: &Vec<InputImportUpload>, whiteboard: &Uuid) -> Result<(), Box<dyn Error>> {
    // Create a batch statement
    let session = Arc::clone(session_arc);
    let mut batch: Batch = Default::default();
    let mut values = Vec::new();
    for upload in uploads {
        // Add a simple query to the batch using its text
        let prepared: PreparedStatement = session.prepare("INSERT INTO fluffy_board.wb_upload (id, whiteboard, upload_type, offset_dx, offset_dy, image_data) VALUES (?, ?, ?, ?, ?, ?);").await?;
        batch.append_statement(prepared);
        values.push((&upload.uuid, whiteboard, &upload.upload_type, &upload.offset_dx, &upload.offset_dy, &upload.uint8list));
    }
    // Run the batch, doesn't return any rows
    session.batch(&batch, &values).await?;
    Ok(())
}

pub async fn import_textitems(session_arc: &Arc<Session>, textItems: &Vec<InputImportTextItem>, whiteboard: &Uuid) -> Result<(), Box<dyn Error>> {
    // Create a batch statement
    let session = Arc::clone(session_arc);
    let mut batch: Batch = Default::default();
    let mut values = Vec::new();
    for textItem in textItems {
        // Add a simple query to the batch using its text
        let prepared: PreparedStatement = session.prepare("INSERT INTO fluffy_board.wb_textitem (id, whiteboard, stroke_width, max_width, max_height, color, content_text, offset_dx, offset_dy) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)").await?;
        batch.append_statement(prepared);
        values.push((&textItem.uuid, whiteboard, &textItem.stroke_width,&textItem.max_width, &textItem.max_height, &textItem.color, &textItem.text, &textItem.offset_dx, &textItem.offset_dy));
    }
    // Run the batch, doesn't return any rows
    session.batch(&batch, &values).await?;
    Ok(())
}
