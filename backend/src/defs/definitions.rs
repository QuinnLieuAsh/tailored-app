// file for the definitions and the definition repo
use uuid::Uuid;
use chrono::{Utc};


//Defition is a definition object that has
//id: the key indentifier
//term: the word in question
//formal: fomral Defition
//useful: defintion in plain language
//bady: defintion using the feynmann technique
#[derive(Clone)]
#[derive(serde::Serialize)]
pub struct Definition {
    pub id: String,
    pub term: String,
    pub formal_def: String,
    pub useful_def: String,
    pub simple_def: String,
    pub date_created: chrono::DateTime<chrono::Utc>
}

impl Definition {


    pub fn new(term: &str, def: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            term: term.to_string(),
            formal_def : "".to_string(),
            useful_def: def.to_string(),
            simple_def: "".to_string(),
            date_created: Utc::now()
        }
    }
}