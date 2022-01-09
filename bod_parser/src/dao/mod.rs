#[derive(Clone, Debug, Deserialize, Serialize)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl From<&str> for Difficulty {
    fn from(s: &str) -> Self {
        match s {
            "easy" => Difficulty::Easy,
            "Easy" => Difficulty::Easy,
            "medium" => Difficulty::Medium,
            "Medium" => Difficulty::Medium,
            "hard" => Difficulty::Hard,
            "Hard" => Difficulty::Hard,
            e => {
                println!("Invalid value passed, {}", e);
                Difficulty::Easy
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Punishment {
    name: String,
    difficulty: Difficulty,
    assigned: bool,
    assignee: Option<String>,
}
