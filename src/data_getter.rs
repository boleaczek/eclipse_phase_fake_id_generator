pub mod api_data {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PersonData {
        pub name: String,
        pub surname: String,
        pub gender: String,
        pub region: String
    }

    pub fn get(ammount: u32) -> Result<Vec<PersonData>, Box<dyn std::error::Error>> {
        let request = format!("https://uinames.com/api/?amount={}", ammount);
        let resp: Vec<PersonData> = reqwest::get(&request)?
            .json()?;

        Ok(resp)
    }
}

pub mod local_data {
    use serde_json;
    use serde::{Serialize, Deserialize};
    use std::fs;
    
    #[derive(Serialize, Deserialize, Debug)]
    pub enum Rep {
        ARep(u32),
        CRep(u32),
        FRep(u32),
        GRep(u32),
        IRep(u32),
        XRep(u32),
        RRep(u32)
    }

    impl Rep {
        pub fn pretty_print(&self) -> String {
            match self {
                Rep::ARep(value) => format!("@-rep({})", value),
                Rep::CRep(value) => format!("c-rep({})", value),
                Rep::FRep(value) => format!("f-rep({})", value),
                Rep::GRep(value) => format!("g-rep({})", value),
                Rep::IRep(value) => format!("i-rep({})", value),
                Rep::XRep(value) => format!("x-rep({})", value),
                Rep::RRep(value) => format!("r-rep({})", value)
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Job {
        pub name: String,
        pub rep: Rep
    }

    pub fn get(local_data_file: Option<&str>) -> Result<Vec<Job>, Box<dyn std::error::Error>> {
        let contents;
        if let Some(filename) = local_data_file {
            contents = fs::read_to_string(filename)?;
        }
        else {
            contents = fs::read_to_string("local_data.json")?;
        }

        let local_data = serde_json::from_str(&contents)?;

        Ok(local_data)
    }
}