#[derive(serde::Deserialize, serde::Serialize, Debug, Eq, PartialEq)]
pub enum Transport {
    // for those read from the kernel audit subsystem
    #[serde(rename = "audit")]
    Audit,

    // for internally generated messages
    #[serde(rename = "driver")]
    Driver,

    // for those received via the local syslog socket with the syslog protocol
    #[serde(rename = "syslog")]
    Syslog,

    // for those received via the native journal protocol
    #[serde(rename = "journal")]
    Journal,

    // for those read from a service's standard output or error output
    #[serde(rename = "stdout")]
    Stdout,

    // for those read from the kernel
    #[serde(rename = "kernel")]
    Kernel,
}

#[cfg(test)]
mod tests {
    use super::Transport;
    #[derive(serde::Deserialize)]
    struct Val {
        field: Transport,
    }

    #[test]
    fn test_deserialize_transport_audit() {
        let s = r#"{"field":"audit"}"#;
        let j: Val = serde_json::from_str(s).unwrap();
        assert_eq!(j.field, Transport::Audit);
    }

    #[test]
    fn test_deserialize_transport_driver() {
        let s = r#"{"field":"driver"}"#;
        let j: Val = serde_json::from_str(s).unwrap();
        assert_eq!(j.field, Transport::Driver);
    }

    #[test]
    fn test_deserialize_transport_syslog() {
        let s = r#"{"field":"syslog"}"#;
        let j: Val = serde_json::from_str(s).unwrap();
        assert_eq!(j.field, Transport::Syslog);
    }

    #[test]
    fn test_deserialize_transport_journal() {
        let s = r#"{"field":"journal"}"#;
        let j: Val = serde_json::from_str(s).unwrap();
        assert_eq!(j.field, Transport::Journal);
    }

    #[test]
    fn test_deserialize_transport_stdout() {
        let s = r#"{"field":"stdout"}"#;
        let j: Val = serde_json::from_str(s).unwrap();
        assert_eq!(j.field, Transport::Stdout);
    }

    #[test]
    fn test_deserialize_transport_kernel() {
        let s = r#"{"field":"kernel"}"#;
        let j: Val = serde_json::from_str(s).unwrap();
        assert_eq!(j.field, Transport::Kernel);
    }
}
