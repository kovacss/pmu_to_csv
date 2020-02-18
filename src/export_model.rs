use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct RunnerCSV {
    pub date: String,

    pub reunion: u16,

    pub course: u16,

    #[serde(rename(serialize = "numero"))]
    pub num: u16,

    #[serde(rename(deserialize = "cheval", serialize = "cheval"))]
    pub name: String,

    #[serde(rename(deserialize = "driver", serialize = "driver"))]
    pub driver: String,

    #[serde(rename(deserialize = "proprietaire", serialize = "proprietaire"))]
    pub owner: String,

    #[serde(rename(deserialize = "entraineur", serialize = "entraineur"))]
    pub trainer: String,

    #[serde(rename(deserialize = "musique", serialize = "musique"))]
    pub music: String,

    #[serde(rename(serialize = "cote"))]
    pub rapport: Option<f32>,

    pub status: String,

    #[serde(rename(serialize = "resultat"))]
    // TODO Change to u16
    pub result: Option<usize>
}