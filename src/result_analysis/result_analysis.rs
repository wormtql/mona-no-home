use std::collections::HashMap;
use mona::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName};
use mona::artifacts::eff::ARTIFACT_EFF5;
use mona::character::CharacterName;
use mona::common::StatName;
use mona::weapon::WeaponName;
use serde::{Serialize, Deserialize};
use crate::models::compute_result::ComputeResult;

#[derive(Hash, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum ArtifactSetUsage {
    Set4(ArtifactSetName),
    Set22(ArtifactSetName, ArtifactSetName),
    Set2(ArtifactSetName),
    Chiri
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterAnalysisResult {
    pub weapon_usage: Vec<(WeaponName, f64)>,
    pub artifact_set_usage: Vec<(ArtifactSetUsage, f64)>,
    pub artifact_sub_stat_statistics: HashMap<StatName, f64>,
    pub main_stat_usage: HashMap<ArtifactSlotName, HashMap<StatName, f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponAnalysisResult {
    pub character_usage: Vec<(CharacterName, f64)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub weapon_result: HashMap<WeaponName, WeaponAnalysisResult>,
    pub character_result: HashMap<CharacterName, CharacterAnalysisResult>
}

fn get_artifacts_set(artifacts: &[Artifact]) -> ArtifactSetUsage {
    let mut map: HashMap<ArtifactSetName, usize> = HashMap::new();

    for artifact in artifacts.iter() {
        (*map.entry(artifact.set_name).or_insert(0)) += 1;
    }

    let set_ge2: Vec<(ArtifactSetName, usize)> = map.iter()
        .filter(|x| *x.1 >= 2)
        .map(|x| (*x.0, *x.1))
        .collect();
    if set_ge2.len() == 1 && set_ge2[0].1 >= 4 {
        ArtifactSetUsage::Set4(set_ge2[0].0)
    } else if set_ge2.len() == 1 {
        ArtifactSetUsage::Set2(set_ge2[0].0)
    } else if set_ge2.len() == 0 {
        ArtifactSetUsage::Chiri
    } else {
        let s1 = set_ge2[0].0 as usize;
        let s2 = set_ge2[1].0 as usize;
        if s1 < s2 {
            ArtifactSetUsage::Set22(set_ge2[0].0, set_ge2[1].0)
        } else {
            ArtifactSetUsage::Set22(set_ge2[1].0, set_ge2[0].0)
        }
    }
}

fn get_sub_stat_values(artifacts: &[Artifact]) -> HashMap<StatName, f64> {
    let mut result = HashMap::new();

    for artifact in artifacts.iter() {
        for &(name, value) in artifact.sub_stats.iter() {
            *result.entry(name).or_insert(0.0) += value;
        }
    }

    result
}

fn normalize<T, U>(map: &mut HashMap<T, HashMap<U, f64>>) {
    for (_, inner_map) in map.iter_mut() {
        let sum = inner_map.iter()
            .map(|x| *x.1)
            .sum::<f64>();

        if sum > 0.0 {
            for (_, value) in inner_map.iter_mut() {
                *value /= sum
            }
        }
    }
}

fn map_to_sorted_vec<T: Clone>(map: &HashMap<T, f64>) -> Vec<(T, f64)> {
    let mut result: Vec<(T, f64)> = Vec::new();

    for (k, v) in map.iter() {
        result.push((k.clone(), *v));
    }

    result.sort_by(|x, y| (y.1).partial_cmp(&x.1).unwrap());

    result
}

pub fn result_analysis(results: &[ComputeResult]) -> AnalysisResult {
    let mut weapon_usage: HashMap<CharacterName, HashMap<WeaponName, f64>> = HashMap::new();
    let mut character_usage: HashMap<WeaponName, HashMap<CharacterName, f64>> = HashMap::new();
    let mut artifact_set_usage: HashMap<CharacterName, HashMap<ArtifactSetUsage, f64>> = HashMap::new();
    let mut sub_stat_avg: HashMap<CharacterName, HashMap<StatName, f64>> = HashMap::new();
    let mut sub_stat_count: HashMap<CharacterName, usize> = HashMap::new();
    let mut main_stat_usage: HashMap<CharacterName, HashMap<ArtifactSlotName, HashMap<StatName, f64>>> = HashMap::new();

    for result in results.iter() {
        let character_name = result.character.name;
        let weapon_name = result.weapon.name;
        let artifact_set = get_artifacts_set(&result.result_artifacts);

        (*weapon_usage.entry(character_name).or_insert(HashMap::new())
            .entry(weapon_name).or_insert(0.0)) += 1.0;
        (*character_usage.entry(weapon_name).or_insert(HashMap::new())
            .entry(character_name).or_insert(0.0)) += 1.0;
        (*artifact_set_usage.entry(character_name).or_insert(HashMap::new())
            .entry(artifact_set).or_insert(0.0)) += 1.0;
        for artifact in result.result_artifacts.iter() {
            let slot_name = artifact.slot;
            let main_stat_name = artifact.main_stat.0;

            *main_stat_usage.entry(character_name).or_insert(HashMap::new())
                .entry(slot_name)
                .or_insert(HashMap::new())
                .entry(main_stat_name)
                .or_insert(0.0) += 1.0;
        }

        let sub_stat_values = get_sub_stat_values(&result.result_artifacts);
        let temp = sub_stat_avg.entry(character_name).or_insert(HashMap::new());
        for (&name, &value) in sub_stat_values.iter() {
            *temp.entry(name).or_insert(0.0) += value;
        }
        *sub_stat_count.entry(character_name).or_insert(0) += 1;
    }

    // do normalize
    normalize(&mut weapon_usage);
    normalize(&mut character_usage);
    normalize(&mut artifact_set_usage);
    for (_, inner_map) in main_stat_usage.iter_mut() {
        normalize(inner_map);
    }

    // avg sub stat
    for (&character_name, stat_map) in sub_stat_avg.iter_mut() {
        let count = *sub_stat_count.get(&character_name).unwrap();
        if count > 0 {
            for (&stat_name, value) in stat_map.iter_mut() {
                *value /= count as f64;
                // normalize to artifact eff
                *value /= ARTIFACT_EFF5.get_value(stat_name, 3);
            }
        }
    }

    let character_names: Vec<_> = weapon_usage.keys().cloned().collect();
    let weapon_names: Vec<_> = character_usage.keys().cloned().collect();

    let mut character_result: HashMap<CharacterName, CharacterAnalysisResult> = HashMap::new();
    for &character_name in character_names.iter() {
        let weapon_usage_vec = map_to_sorted_vec(weapon_usage.get(&character_name).unwrap());
        let artifact_set_usage_vec = map_to_sorted_vec(artifact_set_usage.get(&character_name).unwrap());
        let sub_stat_statistics = sub_stat_avg.remove(&character_name).unwrap();

        let character_analysis = CharacterAnalysisResult {
            weapon_usage: weapon_usage_vec,
            artifact_set_usage: artifact_set_usage_vec,
            artifact_sub_stat_statistics: sub_stat_statistics,
            main_stat_usage: main_stat_usage.remove(&character_name).unwrap_or_default()
        };

        character_result.insert(character_name, character_analysis);
    }

    let mut weapon_result: HashMap<WeaponName, WeaponAnalysisResult> = HashMap::new();
    for &weapon_name in weapon_names.iter() {
        let character_usage_vec = map_to_sorted_vec(character_usage.get(&weapon_name).unwrap());

        let weapon_analysis = WeaponAnalysisResult {
            character_usage: character_usage_vec
        };

        weapon_result.insert(weapon_name, weapon_analysis);
    }

    AnalysisResult {
        character_result,
        weapon_result
    }
}