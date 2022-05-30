use mona::artifacts::Artifact;
use crate::models::compute_result::ComputeConfig;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct CreateComputeResultRequest {
    pub config: ComputeConfig,
    pub result_artifacts: Vec<Artifact>,
}
