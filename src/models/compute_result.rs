use chrono::NaiveDateTime;
use mona_wasm::applications::common::{BuffInterface, CharacterInterface, TargetFunctionInterface, WeaponInterface};
use mona::artifacts::Artifact;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::compute_result;

// used when doing analysis
#[derive(Serialize, Deserialize)]
pub struct ComputeResult {
    pub character: CharacterInterface,
    pub weapon: WeaponInterface,
    pub buffs: Vec<BuffInterface>,
    pub target_function: TargetFunctionInterface,
    pub result_artifacts: Vec<Artifact>
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct ComputeResultInDB {
    pub id: i32,
    pub created: NaiveDateTime,
    pub config_json: Option<String>,
    pub artifacts_json: Option<String>,
}

#[derive(Insertable)]
#[table_name = "compute_result"]
pub struct NewComputeResult {
    pub config_json: String,
    pub artifacts_json: String,
}

#[derive(Serialize, Deserialize)]
pub struct ComputeConfig {
    pub character: CharacterInterface,
    pub weapon: WeaponInterface,
    pub buffs: Vec<BuffInterface>,
    pub target_function: TargetFunctionInterface,
}

impl ComputeResultInDB {
    pub fn to_compute_result(&self) -> Option<ComputeResult> {
        let config = serde_json::from_str::<ComputeConfig>(self.config_json.as_ref()?.as_str()).ok()?;
        let result_artifacts = serde_json::from_str::<Vec<Artifact>>(self.artifacts_json.as_ref()?.as_str()).ok()?;

        let compute_result = ComputeResult {
            result_artifacts,
            character: config.character,
            weapon: config.weapon,
            buffs: config.buffs,
            target_function: config.target_function
        };

        Some(compute_result)
    }
}
