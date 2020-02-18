use csv::Writer;
use crate::api_response::Runner;
use std::fs::File;

mod api_response;
mod export_model;
mod service;
mod utils;

use crate::api_response::RunnerResponse;
use crate::export_model::RunnerCSV;
use crate::utils::get_date;

fn append_runner_to_file(wtr: &mut Writer<std::fs::File>, date: String, runner: &Runner, p: &RunnerResponse) {
    let reunion = p.reunion;
    let course = p.course;
    
    let position = p.result.iter().position(|&s| s == runner.num);
    let position = if position.is_some() {
        Some::<usize>(position.unwrap() + 1)
    } else {
        position
    };

    let rapport = if runner.rapport_reference.is_some() {
        Some::<f32>(runner.rapport_reference.as_ref().unwrap().rapport)
    } else {
        None
    };
    
    wtr.serialize(RunnerCSV {
        date: date,
        reunion,
        course,
        num: runner.num,
        name: runner.name.as_str().to_string(),
        driver: runner.driver.as_ref().unwrap().as_str().to_string(),
        owner: runner.owner.as_str().to_string(),
        trainer: runner.trainer.as_str().to_string(),
        music: runner.music.as_str().to_string(),
        rapport: rapport,
        result: position,
        status: runner.status.as_str().to_string(),
    });
}

fn to_csv(all_runners: Vec<Vec<RunnerResponse>>, date: Vec<String>, file_path: String) {
    let file = File::create(file_path.as_str());
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("File cannot be created: {:?}", error),
    };
    let mut i = 0;
    let mut wtr = csv::Writer::from_writer(file);
    all_runners.into_iter().for_each(|runner_for_date| {
        runner_for_date.into_iter().for_each(|p| {
            p.runners.iter().for_each(|runner| {
                append_runner_to_file(&mut wtr, date.get(i).unwrap().to_string(), &runner, &p);
            });
        });
        i = i + 1;
    });
    wtr.flush().unwrap();
}

fn get_participants_for_date(date: String) -> Vec<RunnerResponse> {
    // Get All Reunions and Races
    let json_response = service::get_programme_for_date(date.clone());
    let reunions = json_response.programme.reunions;
    
    // Iterate through each "race" of each "reunion" and get its runners
    let participants: Vec<RunnerResponse> = reunions
        .iter()
        .flat_map(|reunion| {
            let reunion_participants: Vec<RunnerResponse> = reunion
                .courses
                .iter()
                .map(|course| {
                    let result_order = &course.ordreArrivee;
                    let result: Vec<u16> = match result_order {
                        Some(v) => v
                            .iter()
                            .map(|order| order.get(0))
                            .filter(|order| order.is_some())
                            .map(|order| order.unwrap())
                            .cloned()
                            .collect(),
                        None => Vec::new(),
                    };
                    let mut response: RunnerResponse = service::get_participant_for_course(course, date.clone());
                    response.reunion = course.numReunion;
                    response.course = course.numOrdre;
                    response.result = result;
                    response
                })
                .collect();
            reunion_participants
        })
        .collect();
    participants
}

fn main() -> Result<(), Box<std::error::Error>> {
    let dates = get_date();

    let participants: Vec<Vec<RunnerResponse>> = dates
        .iter()
        .map(|d| {
            let cloned = d.clone();
            get_participants_for_date(cloned)
        })
        .collect();

    let file_path = format!(
        "./partants_{}_{}.csv",
        get_date().get(0).unwrap(),
        get_date().get(get_date().len() - 1).unwrap()
    );
    to_csv(participants, get_date(), file_path);
    Ok(())
}
