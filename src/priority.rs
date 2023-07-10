#[derive(serde::Deserialize, Debug, Eq, PartialEq)]
#[serde(untagged)]
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
}
