extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::env;

pub mod data_getter;
pub mod data_combiner;

use data_getter::api_data::PersonData;
use data_getter::local_data::Job;

fn generate_data<'a>(number_to_generate: u32, custom_data_path: Option<&str>)
 -> Result<(Vec<PersonData>, Vec<Job>), Box<dyn std::error::Error>> {
    let api_data = data_getter::api_data::get(number_to_generate)?;
    let local_data = data_getter::local_data::get(custom_data_path)?;
    Ok((api_data, local_data))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let number_to_generate;
    let local_data_path;
    if args.len() > 1 {
        number_to_generate = args[1].parse().unwrap();
        if args.len() > 2 {
            local_data_path = Some(args[2].as_ref());
        }
        else {
            local_data_path = None;
        }
    }
    else {
        number_to_generate = 25;
        local_data_path = None;
    }

    let data_read_result = generate_data(number_to_generate, local_data_path);
    
    match data_read_result {
        Ok((person_data, job_data)) => {
            let combined_data = data_combiner::combine(&person_data, &job_data);
            for id in combined_data {
                println!("{}", id.pretty_print());
            }
        },
        Err(err) => {
            println!("{}", err);
        }
    }
}
