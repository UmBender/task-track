#[derive(Clone, Debug, PartialEq)]
pub enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}

impl Priority {
    pub fn to_str(&self) -> &str {
        return match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
            Self::Urgent => "urgent",
        };
    }

    pub fn to_string(&self) -> String {
        return String::from(self.to_str());
    }

    pub fn from_string(priority: &String) -> Option<Priority> {
        return match priority.as_str() {
            "low" => Some(Self::Low),
            "normal" => Some(Self::Normal),
            "high" => Some(Self::High),
            "urgent" => Some(Self::Urgent),
            _ => None,
        };
    }

    fn to_numeric(&self) -> usize {
        return match self {
            Self::Low => 1,
            Self::Normal => 2,
            Self::High => 3,
            Self::Urgent => 4,
        };
    }
}

impl PartialOrd for Priority {
    fn ge(&self, other: &Self) -> bool {
        return self.to_numeric() >= other.to_numeric();
    }

    fn gt(&self, other: &Self) -> bool {
        return self.to_numeric() > other.to_numeric();
    }

    fn le(&self, other: &Self) -> bool {
        return self.to_numeric() <= other.to_numeric();
    }

    fn lt(&self, other: &Self) -> bool {
        return self.to_numeric() < other.to_numeric();
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self < other {
            return Some(std::cmp::Ordering::Less);
        }
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        return Some(std::cmp::Ordering::Greater);
    }
}

#[cfg(test)]
mod tests_priority {
    use super::*;
    #[test]
    fn test_to_str() {
        assert_eq!(
            Priority::Low.to_str(),
            "low",
            "Low should be equal to \"low\""
        );
        assert_eq!(
            Priority::Normal.to_str(),
            "normal",
            "Normal should be equal to \"normal\""
        );
        assert_eq!(
            Priority::High.to_str(),
            "high",
            "High should be equal to \"high\""
        );
        assert_eq!(
            Priority::Urgent.to_str(),
            "urgent",
            "Urgent should be equal to \"urgent\""
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            Priority::Low.to_string(),
            String::from("low"),
            "Low should be equal to \"low\""
        );
        assert_eq!(
            Priority::Normal.to_string(),
            String::from("normal"),
            "Normal should be equal to \"normal\""
        );
        assert_eq!(
            Priority::High.to_string(),
            String::from("high"),
            "High should be equal to \"high\""
        );
        assert_eq!(
            Priority::Urgent.to_string(),
            String::from("urgent"),
            "Urgent should be equal to \"urgent\""
        );
    }

    #[test]
    fn to_numeric_low() {
        let a = Priority::Low.to_numeric();
        assert_eq!(a, 1, "Low to numeric should be 1");
    }

    #[test]
    fn to_numeric_normal() {
        let a = Priority::Normal.to_numeric();
        assert_eq!(a, 2, "Normal to numeric should be 2");
    }

    #[test]
    fn to_numeric_high() {
        let a = Priority::High.to_numeric();
        assert_eq!(a, 3, "High to numeric should be 3");
    }

    #[test]
    fn to_numeric_urgent() {
        let a = Priority::Urgent.to_numeric();
        assert_eq!(a, 4, "Urgent to numeric should be 4");
    }

    #[test]
    fn test_ord_low() {
        let val = Priority::Low;

        assert!(!(val < Priority::Low), "Low should be not lower than lower");
        assert!(!(val > Priority::Low), "Low should be not lower than lower");

        assert!(val >= Priority::Low, "Low should be eqaul to lower");
        assert!(val <= Priority::Low, "Low should be eqaul to lower");

        assert!(val == Priority::Low, "Low should be eqaul to lower");

        // Normal
        assert!(val < Priority::Normal, "Low should be lower than normal");
        assert!(Priority::Normal > val, "Low should be lower than normal");
        assert!(!(Priority::Normal < val), "Low should be lower than normal");
        assert!(!(val > Priority::Normal), "Low should be lower than normal");

        assert!(val <= Priority::Normal, "Low should be lower than normal");
        assert!(Priority::Normal >= val, "Low should be lower than normal");
        assert!(
            !(Priority::Normal <= val),
            "Low should be lower than normal"
        );
        assert!(
            !(val >= Priority::Normal),
            "Low should be lower than normal"
        );

        assert_ne!(val, Priority::Normal, "Low should be different of normal");

        // High

        assert!(val < Priority::High, "Low should be lower than high");
        assert!(Priority::High > val, "Low should be lower than high");
        assert!(!(Priority::High < val), "Low should be lower than high");
        assert!(!(val > Priority::High), "Low should be lower than high");

        assert!(val <= Priority::High, "Low should be lower than high");
        assert!(Priority::High >= val, "Low should be lower than high");
        assert!(!(Priority::High <= val), "Low should be lower than high");
        assert!(!(val >= Priority::High), "Low should be lower than high");

        assert_ne!(val, Priority::High, "Low should be different of high");

        // Urgent

        assert!(val < Priority::Urgent, "Low should be lower than urgent");
        assert!(Priority::Urgent > val, "Low should be lower than urgent");
        assert!(!(Priority::Urgent < val), "Low should be lower than urgent");
        assert!(!(val > Priority::Urgent), "Low should be lower than urgent");

        assert!(val <= Priority::Urgent, "Low should be lower than urgent");
        assert!(Priority::Urgent >= val, "Low should be lower than urgent");
        assert!(
            !(Priority::Urgent <= val),
            "Low should be lower than urgent"
        );
        assert!(
            !(val >= Priority::Urgent),
            "Low should be lower than urgent"
        );

        assert_ne!(val, Priority::Urgent, "Low should be different of urgent");
    }

    #[test]
    fn test_ord_normal() {
        let val = Priority::Normal;

        // Low

        assert!(!(val < Priority::Low), "Normal should be greater than low");
        assert!(!(Priority::Low > val), "Normal should be greater than low");
        assert!(Priority::Low < val, "Normal should be greater than low");
        assert!(val > Priority::Low, "Normal should be greater than low");

        assert!(!(val <= Priority::Low), "Normal should be greater than low");
        assert!(!(Priority::Low >= val), "Normal should be greater than low");
        assert!(Priority::Low <= val, "Normal should be greater than low");
        assert!(val >= Priority::Low, "Normal should be greater than low");

        assert_ne!(val, Priority::Low, "Normal should be different of low");

        // Normal

        assert!(
            !(val < Priority::Normal),
            "Normal should be not lower than normal"
        );
        assert!(
            !(val > Priority::Normal),
            "Normal should be not lower than normal"
        );

        assert!(val >= Priority::Normal, "Normal should be eqaul to normal");
        assert!(val <= Priority::Normal, "Normal should be eqaul to normal");

        assert!(val == Priority::Normal, "Normal should be eqaul to normal");

        // High

        assert!(val < Priority::High, "Normal should be lower than high");
        assert!(Priority::High > val, "Normal should be lower than high");
        assert!(!(Priority::High < val), "Normal should be lower than high");
        assert!(!(val > Priority::High), "Normal should be lower than high");

        assert!(val <= Priority::High, "Normal should be lower than high");
        assert!(Priority::High >= val, "Normal should be lower than high");
        assert!(!(Priority::High <= val), "Normal should be lower than high");
        assert!(!(val >= Priority::High), "Normal should be lower than high");

        assert_ne!(val, Priority::High, "Normal should be different of high");

        // Urgent

        assert!(val < Priority::Urgent, "Normal should be lower than urgent");
        assert!(Priority::Urgent > val, "Normal should be lower than urgent");
        assert!(
            !(Priority::Urgent < val),
            "Normal should be lower than urgent"
        );
        assert!(
            !(val > Priority::Urgent),
            "Normal should be lower than urgent"
        );

        assert!(
            val <= Priority::Urgent,
            "Normal should be lower than urgent"
        );
        assert!(
            Priority::Urgent >= val,
            "Normal should be lower than urgent"
        );
        assert!(
            !(Priority::Urgent <= val),
            "Normal should be lower than urgent"
        );
        assert!(
            !(val >= Priority::Urgent),
            "Normal should be lower than urgent"
        );

        assert_ne!(val, Priority::Urgent, "Normal should be lower than urgent");
    }
    #[test]
    fn test_ord_high() {
        let val = Priority::High;

        // Low

        assert!(!(val < Priority::Low), "High should be greater than low");
        assert!(!(Priority::Low > val), "High should be greater than low");
        assert!(Priority::Low < val, "High should be greater than low");
        assert!(val > Priority::Low, "High should be greater than low");

        assert!(!(val <= Priority::Low), "High should be greater than low");
        assert!(!(Priority::Low >= val), "High should be greater than low");
        assert!(Priority::Low <= val, "High should be greater than low");
        assert!(val >= Priority::Low, "High should be greater than low");

        assert_ne!(val, Priority::Low, "High should be different of low");

        // Normal

        assert!(
            !(val < Priority::Normal),
            "High should be greater than normal"
        );
        assert!(
            !(Priority::Normal > val),
            "High should be greater than normal"
        );
        assert!(Priority::Normal < val, "High should be greater than normal");
        assert!(val > Priority::Normal, "High should be greater than normal");

        assert!(
            !(val <= Priority::Normal),
            "High should be greater than normal"
        );
        assert!(
            !(Priority::Normal >= val),
            "High should be greater than normal"
        );
        assert!(
            Priority::Normal <= val,
            "High should be greater than normal"
        );
        assert!(
            val >= Priority::Normal,
            "High should be greater than normal"
        );

        assert_ne!(val, Priority::Normal, "High should be different of normal");

        // High

        assert!(
            !(val < Priority::High),
            "High should be not lower than high"
        );
        assert!(
            !(val > Priority::High),
            "High should be not lower than high"
        );

        assert!(val >= Priority::High, "High should be eqaul to high");
        assert!(val <= Priority::High, "High should be eqaul to high");

        assert!(val == Priority::High, "High should be eqaul to high");

        // Urgent

        assert!(val < Priority::Urgent, "High should be lower than urgent");
        assert!(Priority::Urgent > val, "High should be lower than urgent");
        assert!(
            !(Priority::Urgent < val),
            "High should be lower than urgent"
        );
        assert!(
            !(val > Priority::Urgent),
            "High should be lower than urgent"
        );

        assert!(val <= Priority::Urgent, "High should be lower than urgent");
        assert!(Priority::Urgent >= val, "High should be lower than urgent");
        assert!(
            !(Priority::Urgent <= val),
            "High should be lower than urgent"
        );
        assert!(
            !(val >= Priority::Urgent),
            "High should be lower than urgent"
        );

        assert_ne!(val, Priority::Urgent, "High should be lower than urgent");
    }

    #[test]
    fn test_ord_urgent() {
        let val = Priority::Urgent;

        // Low

        assert!(!(val < Priority::Low), "Urgent should be greater than low");
        assert!(!(Priority::Low > val), "Urgent should be greater than low");
        assert!(Priority::Low < val, "Urgent should be greater than low");
        assert!(val > Priority::Low, "Urgent should be greater than low");

        assert!(!(val <= Priority::Low), "Urgent should be greater than low");
        assert!(!(Priority::Low >= val), "Urgent should be greater than low");
        assert!(Priority::Low <= val, "Urgent should be greater than low");
        assert!(val >= Priority::Low, "Urgent should be greater than low");

        assert_ne!(val, Priority::Low, "Urgent should be different of low");

        // Normal

        assert!(
            !(val < Priority::Normal),
            "Urgent should be greater than normal"
        );
        assert!(
            !(Priority::Normal > val),
            "Urgent should be greater than normal"
        );
        assert!(
            Priority::Normal < val,
            "Urgent should be greater than normal"
        );
        assert!(
            val > Priority::Normal,
            "Urgent should be greater than normal"
        );

        assert!(
            !(val <= Priority::Normal),
            "Urgent should be greater than normal"
        );
        assert!(
            !(Priority::Normal >= val),
            "Urgent should be greater than normal"
        );
        assert!(
            Priority::Normal <= val,
            "Urgent should be greater than normal"
        );
        assert!(
            val >= Priority::Normal,
            "Urgent should be greater than normal"
        );

        assert_ne!(
            val,
            Priority::Normal,
            "Urgent should be different of normal"
        );

        // High

        assert!(
            !(val < Priority::High),
            "Urgent should be greater than high"
        );
        assert!(
            !(Priority::High > val),
            "Urgent should be greater than high"
        );
        assert!(Priority::High < val, "Urgent should be greater than high");
        assert!(val > Priority::High, "Urgent should be greater than high");

        assert!(
            !(val <= Priority::High),
            "Urgent should be greater than high"
        );
        assert!(
            !(Priority::High >= val),
            "Urgent should be greater than high"
        );
        assert!(Priority::High <= val, "Urgent should be greater than high");
        assert!(val >= Priority::High, "Urgent should be greater than high");

        assert_ne!(val, Priority::High, "Urgent should be different of high");

        // Urgent

        assert!(
            !(val < Priority::Urgent),
            "Urgent should be not lower than urgent"
        );
        assert!(
            !(val > Priority::Urgent),
            "Urgent should be not lower than urgent"
        );

        assert!(val >= Priority::Urgent, "Urgent should be eqaul to urgent");
        assert!(val <= Priority::Urgent, "Urgent should be eqaul to urgent");

        assert!(val == Priority::Urgent, "Urgent should be eqaul to urgent");
    }
}
