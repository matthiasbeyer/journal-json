#[derive(serde::Serialize, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Priority {
    // system is unusable
    EMERG = 0,

    // action must be taken immediately
    ALERT = 1,

    // critical conditions
    CRIT = 2,

    // error conditions
    ERR = 3,

    // warning conditions
    WARNING = 4,

    // normal, but significant, condition
    NOTICE = 5,

    // informational message
    INFO = 6,

    // debug-level message
    DEBUG = 7,

    // If deserializing to another variant failed
    Unknown(String),
}

impl<'de> serde::Deserialize<'de> for Priority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_ref() {
            "0" => Priority::EMERG,
            "1" => Priority::ALERT,
            "2" => Priority::CRIT,
            "3" => Priority::ERR,
            "4" => Priority::WARNING,
            "5" => Priority::NOTICE,
            "6" => Priority::INFO,
            "7" => Priority::DEBUG,
            _ => Priority::Unknown(s),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Priority;

    #[derive(Debug, serde::Deserialize)]
    struct Obj {
        p: Priority,
    }

    #[test]
    fn test_deser_emerg() {
        let s = r#"{"p":"0"}"#;
        let o: Obj = serde_json::from_str(s).unwrap();
        assert_eq!(o.p, Priority::EMERG);
    }

    #[test]
    fn test_deser_alert() {
        let s = r#"{"p":"1"}"#;
        let o: Obj = serde_json::from_str(s).unwrap();
        assert_eq!(o.p, Priority::ALERT);
    }

    #[test]
    fn test_deser_crit() {
        let s = r#"{"p":"2"}"#;
        let o: Obj = serde_json::from_str(s).unwrap();
        assert_eq!(o.p, Priority::CRIT);
    }

    #[test]
    fn test_deser_err() {
        let s = r#"{"p":"3"}"#;
        let o: Obj = serde_json::from_str(s).unwrap();
        assert_eq!(o.p, Priority::ERR);
    }

    #[test]
    fn test_deser_warning() {
        let s = r#"{"p":"4"}"#;
        let o: Obj = serde_json::from_str(s).unwrap();
        assert_eq!(o.p, Priority::WARNING);
    }

    #[test]
    fn test_deser_notice() {
        let s = r#"{"p":"5"}"#;
        let o: Obj = serde_json::from_str(s).unwrap();
        assert_eq!(o.p, Priority::NOTICE);
    }

    #[test]
    fn test_deser_info() {
        let s = r#"{"p":"6"}"#;
        let o: Obj = serde_json::from_str(s).unwrap();
        assert_eq!(o.p, Priority::INFO);
    }

    #[test]
    fn test_deser_debug() {
        let s = r#"{"p":"7"}"#;
        let o: Obj = serde_json::from_str(s).unwrap();
        assert_eq!(o.p, Priority::DEBUG);
    }
}
