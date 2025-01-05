#[derive(Debug, Clone, PartialEq)]
pub enum TaskState {
    Pending,
    InProgress,
    Ended,
}

impl TaskState {
    pub fn to_str(&self) -> &str {
        return match self {
            Self::Pending => "pending",
            Self::InProgress => "in progress",
            Self::Ended => "ended",
        };
    }
    pub fn to_string(&self) -> String {
        return String::from(self.to_str());
    }

    pub fn from_string(state: &String) -> Option<TaskState> {
        return match state.as_str() {
            "pending" => Some(Self::Pending),
            "in progress" => Some(Self::InProgress),
            "ended" => Some(Self::Ended),
            _ => None,
        };
    }
}
#[cfg(test)]
mod tests_task_state {
    use super::*;

    #[test]
    fn test_to_str() {
        assert_eq!(
            TaskState::Pending.to_str(),
            "pending",
            "Pending should be equal to \"pending\""
        );
        assert_eq!(
            TaskState::InProgress.to_str(),
            "in progress",
            "InProgress should be equal to \"in progress\""
        );
        assert_eq!(
            TaskState::Ended.to_str(),
            "ended",
            "Ended should be equal to \"ended\""
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            TaskState::Pending.to_string(),
            String::from("pending"),
            "Pending should be equal to \"pending\""
        );
        assert_eq!(
            TaskState::InProgress.to_string(),
            String::from("in progress"),
            "InProgress should be equal to \"in progress\""
        );
        assert_eq!(
            TaskState::Ended.to_string(),
            String::from("ended"),
            "Ended should be equal to \"ended\""
        );
    }
}
