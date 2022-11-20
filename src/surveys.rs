use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Answer {
    Input(String),
    MultipleChoice(Vec<u16>),
    OneChoice(u16),
    Priority(Vec<u16>),
    Value(f64),
    Checkbox(bool),
}

impl Default for Answer {
    fn default() -> Self {
        Self::Input(String::new())
    }
}
    

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Survey {
    pub id: String,
    pub description: HashMap<String, String>,
    pub questions: Vec<Question>,
    pub start_ts: i64,
    pub end_ts: i64,
    pub target: UserGroups,
    pub required: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PossibleAnswer {
    Input { max_length: u16 },
    MultipleChoice ( Vec<HashMap<String, String>> ),
    OneChoice ( Vec<HashMap<String, String>> ),
    Priority ( Vec<HashMap<String, String>> ),
    Value { min: f64, max: f64, step: f64 },
    Checkbox,
}

impl Default for PossibleAnswer {
    fn default() -> Self {
        Self::Input { max_length: 12 }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Question {
    pub question: HashMap<String, String>,
    pub possible_answer: PossibleAnswer,
    pub editable: bool,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct SurveyAnswers {
    pub id: String,
    pub answers: Vec<Answer>,
    pub last_mod: i64,
}

impl Survey {
    pub fn new(id: String) -> Self {
        let mut questions = HashMap::new();
        questions.insert(0, Question {
            possible_answer: PossibleAnswer::default(),
            editable: true,
            question: HashMap::new(),
        });

        Self {
            id,
            description: HashMap::new(),
            questions: vec![],
            start_ts: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            end_ts: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
                + 10000000000,
            target: UserGroups::new_with_groups(BTreeMap::from([
                ("school".to_string(), "insa-rouen".to_string()),
            ])),
            required: false,
        }
    }
}
