use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Res<'a, T> {
    pub code: u32,
    pub msg: &'a str,
    pub data: T,
}