use std::env;

pub fn get_date() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./pmu_to_csv [DATE]");
        println!("Format de date: JourMoisAnnee (Ex: 28121993)");
        panic!("Date non specifie");
    }
    let date = args[1].clone();
    let dates: Vec<String> = date.split('-').map(|d| {
        d.to_string()
    }).collect();
    dates
}