use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HeatmapData {
    pub date: String,
    pub count: u32,
}
