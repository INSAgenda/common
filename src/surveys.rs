use std::collections::{HashMap, BTreeMap};

use serde::{Serialize, Deserialize};

use crate::UserGroups;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Answer {
    Input(String),
    MultipleChoice(Vec<u16>),
    OneChoice(u16),
    Priority(HashMap<u16, u16>),
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
    pub description: (String, String),
    pub questions: HashMap<u16, Question>,
    pub date_start: i64,
    pub date_end: i64,
    pub target: UserGroups,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PossibleAnswer {
    Input(u16),
    MultipleChoice(HashMap<u16, (String, String)>),
    OneChoice(HashMap<u16, (String, String)>),
    Priority(HashMap<u16, (String, String)>),
    Value { min: f64, max: f64, step: f64 },
}

impl Default for PossibleAnswer {
    fn default() -> Self {
        Self::Input(12)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Question {
    pub question: (String, String),
    pub possible_answer: PossibleAnswer,
    pub can_edit: bool,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct SurveyAnswers {
    pub id: String,
    pub answers: HashMap<u16, Answer>,
    pub last_mod: i64,
}

impl Survey {
    pub fn new(id: String) -> Self {
        let mut questions = HashMap::new();
        questions.insert(0, Question {
            question: (String::from("Question"), String::from("Question")),
            possible_answer: PossibleAnswer::Input(12),
            can_edit: true,
        });

        Self {
            id,
            description: (String::from("Description"), String::from("Description")),
            questions,
            date_start: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            date_end: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
                + 10000000000,
            target: UserGroups::new_with_groups(BTreeMap::from([
                ("school".to_string(), "insa-rouen".to_string()),
            ])),
        }
    }
}