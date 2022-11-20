use std::collections::{HashMap, BTreeMap};

use serde::{Serialize, Deserialize};

use crate::UserGroups;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PossibleAnswers {
    Input(u16),
    MultipleChoice(HashMap<u16, (String, String)>),
    OneChoice(HashMap<u16, (String, String)>),
    Priority(HashMap<u16, (String, String)>),
    Value { min: f64, max: f64, step: f64 },
}

impl Default for PossibleAnswers {
    fn default() -> Self {
        Self::Input(12)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Answer {
    Input(String),
    MultipleChoice(Vec<u16>),
    OneChoice(u16),
    Priority(Vec<(u16, u16)>),
    Value(f64),
}

impl Default for Answer {
    fn default() -> Self {
        Self::Input(String::new())
    }
}
    

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Survey {
    pub id: String,
    pub question: (String, String),
    pub answers: PossibleAnswers,
    pub date_start: i64,
    pub date_end: i64,
    pub can_edit: bool,
    pub target: UserGroups,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct SurveyAnswer {
    pub id: String,
    pub answer: Answer,
    pub last_mod: i64,
}

impl Survey {
    pub fn new(id: String) -> Self {
        Self {
            id,
            question: (
                "What is your favorite color?".to_string(),
                "Quelle est votre couleur préférée?".to_string(),
            ),
            
            date_start: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            date_end: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
                + 10000000000,
            can_edit: false,
            answers: PossibleAnswers::default(),
            target: UserGroups::new_with_groups(BTreeMap::from([
                ("school".to_string(), "insa-rouen".to_string()),
            ])),
            
        }
    }
}