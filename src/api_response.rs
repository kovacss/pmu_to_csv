use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct ProgrammeResponse {
    pub programme: Programme
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Programme {
    pub reunions: Vec<Reunion>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Reunion {
    pub numOfficiel: i16,
    hippodrome: Hippodrome,
    pub courses: Vec<Course>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Hippodrome {
    // "VICHY"
    libelleCourt: String,
    // "HIPPODROME DE VICHY"
    libelleLong: String
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Course {
    pub numReunion: u16,
    pub numOrdre: u16,
    pub ordreArrivee: Option<Vec<Vec<u16>>>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct RunnerResponse {
    #[serde(default)]
    pub reunion: u16,
    #[serde(default)]
    pub course: u16,
    #[serde(default)]
    pub date: String,
    
    #[serde(skip)] 
    pub result: Vec<u16>,

    #[serde(rename(deserialize = "participants"))]
    pub runners: Vec<Runner>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Runner {
    #[serde(rename(deserialize = "numPmu"))]
    pub num: u16,

    #[serde(rename(deserialize = "nom"))]
    pub name: String,

    #[serde(rename(deserialize = "driver"))]
    pub driver: Option<String>,

    #[serde(rename(deserialize = "proprietaire"))]
    pub owner: String,

    #[serde(rename(deserialize = "entraineur"))]
    pub trainer: String,

    #[serde(rename(deserialize = "musique"))]
    pub music: String,

    #[serde(rename(deserialize = "statut"))]
    pub status: String,

    #[serde(rename(deserialize = "dernierRapportReference"))]
    pub rapport_reference: Option<Rapport>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Rapport {
    pub rapport: f32
}