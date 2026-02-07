use std::fmt::{Debug, Display, Formatter};

/// Errors that can be faced when path planning
#[derive(Debug)]
pub enum FindPathError<K: Debug> {
    StartNotFound(K),
    GoalNotFound(K),
    PathNotFound(K, K),
}

impl<K: Debug> Display for FindPathError<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FindPathError::StartNotFound(k) => write!(f, "Start node {:?} not found in graph", k),
            FindPathError::GoalNotFound(k) => write!(f, "Goal node {:?} not found in graph", k),
            FindPathError::PathNotFound(start, goal) => {
                write!(f, "No path exists from {:?} to {:?}", start, goal)
            }
        }
    }
}

impl<K: Debug> std::error::Error for FindPathError<K> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_not_found_error_message() {
        let error = FindPathError::StartNotFound(42);
        let message = format!("{}", error);

        assert_eq!(message, "Start node 42 not found in graph");

        let error = FindPathError::StartNotFound((4, 2));
        let message = format!("{}", error);

        assert_eq!(message, "Start node (4, 2) not found in graph");
    }

    #[test]
    fn goal_not_found_error_message() {
        let error = FindPathError::GoalNotFound(99);
        let message = format!("{}", error);

        assert_eq!(message, "Goal node 99 not found in graph");

        let error = FindPathError::GoalNotFound((9, 9));
        let message = format!("{}", error);

        assert_eq!(message, "Goal node (9, 9) not found in graph");
    }

    #[test]
    fn path_not_found_error_message() {
        let error = FindPathError::PathNotFound(1, 5);
        let message = format!("{}", error);

        assert_eq!(message, "No path exists from 1 to 5");

        let error = FindPathError::PathNotFound((0, 1), (0, 5));
        let message = format!("{}", error);

        assert_eq!(message, "No path exists from (0, 1) to (0, 5)");
    }
}
