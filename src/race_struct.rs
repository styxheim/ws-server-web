use serde_derive::{Deserialize, Serialize};
use std::default::Default;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Clone)]
pub struct Discipline {
  #[serde(rename = "Id")]
  id: u32,
  #[serde(rename = "Name")]
  name: String,
  #[serde(rename = "Gates")]
  gates: Vec<u32>,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Race {
  #[serde(rename = "CompetitionId")]
  pub id: u64,
  #[serde(rename = "CompetitionName")]
  pub name: String,
  #[serde(rename = "SyncPoint")]
  sync_point: Option<u64>,
  #[serde(rename = "TimeStamp")]
  timestamp: u64,
  #[serde(rename = "Gates")]
  gates: Option<Vec<u32>>,
  #[serde(rename = "Penalties")]
  penalties: Option<Vec<u32>>,
  #[serde(rename = "Crews")]
  crews: Option<Vec<u32>>,
  #[serde(rename = "Disciplines")]
  discipline: Option<Vec<Discipline>>,
}

fn read_race(path: &std::path::Path) -> Result<Race, Box<dyn Error>> {
  let race_path = path.join("race");
  let file = fs::File::open(race_path.clone());

  if file.is_err() {
    return Err(
      format!("Open {:?} fails: {:?}", race_path.clone(), file).into(),
    );
  };

  serde_json::from_reader::<_, Race>(file.unwrap()).map_err(|e| {
    format!("File {:?} has invalid format: {:?}", race_path.clone(), e).into()
  })
}

/// Print to console if result has a error
///
/// Returns Some() if Ok and None if Err
fn race_unwrap_to_console(
  result: Result<Race, Box<dyn Error>>,
) -> Option<Race> {
  match result {
    Ok(race) => Some(race),
    Err(e) => {
      println!("Error while open race: {}", e);
      return None;
    }
  }
}

pub fn list_races(db_path: &Path) -> Vec<Race> {
  fs::read_dir(db_path)
    .unwrap()
    .filter(|x| x.as_ref().unwrap().file_type().unwrap().is_dir())
    .map(|x| race_unwrap_to_console(read_race(x.unwrap().path().as_path())))
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect()
}
