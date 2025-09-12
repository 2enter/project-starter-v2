pub mod error;
pub mod response;

pub use error::*;
pub use response::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct IdOnly {
    pub id: String,
}
