//! dao
//! This module handles any Data Transfer Objects that might be used for communication with the underlying database
//! Any code that helps translate the data from the database to something usable by the service should be held under here

/// Describes a difficulty of the challenge. This is mostly for tagging certain challenges of the bucket, but it could also be used
/// to create helper functions that are used to simply pluck a challenge out by a difficulty level, or to default to find a challenge by the given
/// difficulty level that's lowest (given the crew this is for, its more likely the default behavior)
///
/// The labels themselves are meant to be arbitrary; the idea is simply to allow challenges to be divided per the user's desire and not by any sort
/// of objective or data-driven fields.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl From<&str> for Difficulty {
    fn from(s: &str) -> Self {
        match s {
            "Easy" => Difficulty::Easy,
            "Medium" => Difficulty::Medium,
            "Hard" => Difficulty::Hard,
            e => {
                println!("Invalid value passed, {}", e);
                Difficulty::Easy
            }
        }
    }
}

/// Describes a punishment in the bucket.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Punishment {
    /// A label or name of the punishment
    name: String,
    /// The derived `Difficulty` of the punishment
    difficulty: Difficulty,
    /// Indicates if the punishment has been assigned or claimed by anyone
    assigned: bool,
    /// Indicates the assigned crew member to the punishment. If the punishment is unassigned this will be `None`
    assignee: Option<String>,
}

impl Default for Punishment {
    fn default() -> Self {
        Self::default()
    }
}

impl Punishment {
    fn default() -> Self {
        Self {
            name: "Fat Face".to_string(),
            difficulty: Difficulty::Easy,
            assigned: false,
            assignee: None,
        }
    }

    /// Creates a new `Punishment`
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the Punishment
    /// * `difficulty` - The derived `Difficulty` of the Punishment
    /// * `assigned` - Indicates if the Punishment is assigned
    /// * `assignee` - Indicates the assigned crew member to the punishment
    ///
    /// # Example
    ///
    /// ```
    /// use self::bod_parser::dao::Punishment;
    /// let p = Punishment::new("joe".to_string(), "Easy", false, None);
    /// assert_eq!(false, p.get_assigned().clone());
    ///
    /// ```
    pub fn new(name: String, difficulty: &str, assigned: bool, assignee: Option<String>) -> Self {
        Self {
            name,
            difficulty: Difficulty::from(difficulty),
            assigned,
            assignee,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_difficulty(&self) -> &Difficulty {
        &self.difficulty
    }

    pub fn get_assigned(&self) -> &bool {
        &self.assigned
    }

    pub fn get_assignee(&self) -> &Option<String> {
        &self.assignee
    }
}
