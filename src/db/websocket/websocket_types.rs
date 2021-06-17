
use scylla::IntoTypedRows;
use scylla::macros::{FromUserType, IntoUserType};
use scylla::cql_to_rust::FromCqlVal;

// Define custom struct that matches User Defined Type created earlier
// wrapping field in Option will gracefully handle null field values
#[derive(Debug, IntoUserType, FromUserType)]
struct DrawPoint {
    dx: f64,
    dy: f64,
}