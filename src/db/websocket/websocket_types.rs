
use scylla::IntoTypedRows;
use scylla::macros::{FromUserType, IntoUserType};
use scylla::cql_to_rust::FromCqlVal;
use serde::{Deserialize, Serialize};

// Define custom struct that matches User Defined Type created earlier
// wrapping field in Option will gracefully handle null field values
#[derive(Debug, IntoUserType, FromUserType, Serialize, Deserialize)]
pub struct DrawPoint {
    dx: f64,
    dy: f64,
}