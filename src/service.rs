extern crate reqwest;

use crate::api_response::Course;
use crate::api_response::RunnerResponse;
use crate::api_response::ProgrammeResponse;

const API_BASE_URL: &str = "https://online.turfinfo.api.pmu.fr/rest/client/1/programme";

pub fn get_programme_for_date(date: String) -> ProgrammeResponse {
    let url = format!("{}/{}?meteo=true&specialisation=INTERNET", API_BASE_URL, date);
    println!("Getting Program for {}", date);
    let mut http_response = reqwest::get(url.as_str()).unwrap();
    let json_response: ProgrammeResponse = http_response.json().unwrap();
    json_response
}

pub fn get_participant_for_course(course: &Course, date: String) -> RunnerResponse {
    let url = format!("{}/{}/R{}/C{}/participants?specialisation=INTERNET", API_BASE_URL, date, course.numReunion, course.numOrdre);
    println!("Downloading... {}", url);
    let mut http_response = reqwest::get(url.as_str()).unwrap();
    let json_response: RunnerResponse = http_response.json().unwrap();
    println!("Reunion {} Race {} Done", course.numReunion, course.numOrdre);
    json_response
}
 