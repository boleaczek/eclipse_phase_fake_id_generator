extern crate rand;

use rand::prelude::*;
use crate::data_getter;
use data_getter::local_data::Job;
use data_getter::api_data::PersonData;

pub struct FakeId<'a> {
    name: &'a str,
    surname: &'a str,
    job: &'a Job
}

impl<'a> FakeId<'a> {
    pub fn pretty_print(&self) -> String{
        format!("{0: <15}{1: <15}:  {2: <15}    {3: <15}", 
            self.name,
            self.surname, 
            self.job.name,
            self.job.rep.pretty_print())
    }
}

pub fn combine<'a>(names: &'a Vec<PersonData>, jobs: &'a Vec<Job>) -> Vec<FakeId<'a>> {
    let mut rng = rand::thread_rng();
    
    let mut ids: Vec<FakeId> = Vec::new();
    for name in names {
        let job = rng.gen_range(0, jobs.len());
        let id = FakeId {
            name: &name.name,
            surname: &name.surname,
            job: &jobs[job]
        };
        ids.push(id);
    }

    ids
}