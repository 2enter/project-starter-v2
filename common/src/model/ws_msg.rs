use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum WsMsg {
    String(String),
    Number(i32),
}
