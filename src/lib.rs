mod priority;
pub use self::priority::Priority;

mod transport;
pub use self::transport::Transport;

#[derive(serde::Deserialize, serde::Serialize, getset::Getters, Debug)]
pub struct JournalLog {
    /// The human-readable message string for this entry. This is supposed to be the primary text
    /// shown to the user. It is usually not translated (but might be in some cases), and is not
    /// supposed to be parsed for metadata. In order to encode multiple lines in a single log
    /// entry, separate them by newline characters (ASCII code 10), but encode them as a single
    /// MESSAGE= field. Do not add multiple values of this field type to the same entry (also see
    /// above), as consuming applications generally do not expect this and are unlikely to show all
    /// values in that case.
    #[serde(rename = "MESSAGE")]
    #[getset(get = "pub")]
    message: String,

    /// A 128-bit message identifier ID for recognizing certain message types, if this is
    /// desirable. This should contain a 128-bit ID formatted as a lower-case hexadecimal string,
    /// without any separating dashes or suchlike. This is recommended to be a UUID-compatible ID,
    /// but this is not enforced, and formatted differently. Developers can generate a new ID for
    /// this purpose with systemd-id128 new.
    #[serde(rename = "MESSAGE_ID")]
    #[getset(get = "pub")]
    message_id: Option<String>,

    /// A priority value between 0 ("emerg") and 7 ("debug") formatted as a decimal string. This
    /// field is compatible with syslog's priority concept.
    #[serde(rename = "PRIORITY")]
    #[getset(get = "pub")]
    priority: Option<Priority>,

    /// The code location generating this message, if known. Contains the source filename, the line
    /// number and the function name.
    #[serde(rename = "CODE_FILE")]
    #[getset(get = "pub")]
    code_file: Option<String>,

    /// The code location generating this message, if known. Contains the source filename, the line
    /// number and the function name.
    #[serde(rename = "CODE_LINE")]
    #[getset(get = "pub")]
    code_line: Option<String>,

    /// The code location generating this message, if known. Contains the source filename, the line
    /// number and the function name.
    #[serde(rename = "CODE_FUNC")]
    #[getset(get = "pub")]
    code_func: Option<String>,

    /// The low-level Unix error number causing this entry, if any. Contains the numeric value of
    /// errno(3) formatted as a decimal string.
    #[serde(rename = "ERRNO")]
    #[getset(get = "pub")]
    errno: Option<String>,

    /// A randomized, unique 128-bit ID identifying each runtime cycle of the unit. This is
    /// different from _SYSTEMD_INVOCATION_ID in that it is only used for messages coming from
    /// systemd code (e.g. logs from the system/user manager or from forked processes performing
    /// systemd-related setup).
    #[serde(rename = "INVOCATION_ID")]
    #[getset(get = "pub")]
    invocation_id: Option<String>,

    /// A randomized, unique 128-bit ID identifying each runtime cycle of the unit. This is
    /// different from _SYSTEMD_INVOCATION_ID in that it is only used for messages coming from
    /// systemd code (e.g. logs from the system/user manager or from forked processes performing
    /// systemd-related setup).
    #[serde(rename = "USER_INVOCATION_ID")]
    #[getset(get = "pub")]
    user_invocation_id: Option<String>,

    /// Syslog compatibility fields containing the facility (formatted as decimal string), the
    /// identifier string (i.e. "tag"), the client PID, and the timestamp as specified in the
    /// original datagram. (Note that the tag is usually derived from glibc's
    /// program_invocation_short_name variable, see program_invocation_short_name(3).)
    ///
    /// Note that the journal service does not validate the values of any structured journal fields
    /// whose name is not prefixed with an underscore, and this includes any syslog related fields
    /// such as these. Hence, applications that supply a facility, PID, or log level are expected to
    /// do so properly formatted, i.e. as numeric integers formatted as decimal strings.
    #[serde(rename = "SYSLOG_FACILITY")]
    #[getset(get = "pub")]
    syslog_facility: Option<String>,

    /// Syslog compatibility fields containing the facility (formatted as decimal string), the
    /// identifier string (i.e. "tag"), the client PID, and the timestamp as specified in the
    /// original datagram. (Note that the tag is usually derived from glibc's
    /// program_invocation_short_name variable, see program_invocation_short_name(3).)
    ///
    /// Note that the journal service does not validate the values of any structured journal fields
    /// whose name is not prefixed with an underscore, and this includes any syslog related fields
    /// such as these. Hence, applications that supply a facility, PID, or log level are expected to
    /// do so properly formatted, i.e. as numeric integers formatted as decimal strings.
    #[serde(rename = "SYSLOG_IDENTIFIER")]
    #[getset(get = "pub")]
    syslog_identifier: Option<String>,

    /// Syslog compatibility fields containing the facility (formatted as decimal string), the
    /// identifier string (i.e. "tag"), the client PID, and the timestamp as specified in the
    /// original datagram. (Note that the tag is usually derived from glibc's
    /// program_invocation_short_name variable, see program_invocation_short_name(3).)
    ///
    /// Note that the journal service does not validate the values of any structured journal fields
    /// whose name is not prefixed with an underscore, and this includes any syslog related fields
    /// such as these. Hence, applications that supply a facility, PID, or log level are expected to
    /// do so properly formatted, i.e. as numeric integers formatted as decimal strings.
    #[serde(rename = "SYSLOG_PID")]
    #[getset(get = "pub")]
    syslog_pid: Option<String>,

    /// Syslog compatibility fields containing the facility (formatted as decimal string), the
    /// identifier string (i.e. "tag"), the client PID, and the timestamp as specified in the
    /// original datagram. (Note that the tag is usually derived from glibc's
    /// program_invocation_short_name variable, see program_invocation_short_name(3).)
    ///
    /// Note that the journal service does not validate the values of any structured journal fields
    /// whose name is not prefixed with an underscore, and this includes any syslog related fields
    /// such as these. Hence, applications that supply a facility, PID, or log level are expected to
    /// do so properly formatted, i.e. as numeric integers formatted as decimal strings.
    #[serde(rename = "SYSLOG_TIMESTAMP")]
    #[getset(get = "pub")]
    syslog_timestamp: Option<String>,

    /// The original contents of the syslog line as received in the syslog datagram. This field is
    /// only included if the MESSAGE= field was modified compared to the original payload or the
    /// timestamp could not be located properly and is not included in SYSLOG_TIMESTAMP=. Message
    /// truncation occurs when the message contains leading or trailing whitespace (trailing and
    /// leading whitespace is stripped), or it contains an embedded NUL byte (the NUL byte and
    /// anything after it is not included). Thus, the original syslog line is either stored as
    /// SYSLOG_RAW= or it can be recreated based on the stored priority and facility, timestamp,
    /// identifier, and the message payload in MESSAGE=.
    #[serde(rename = "SYSLOG_RAW")]
    #[getset(get = "pub")]
    syslog_raw: Option<String>,

    /// A documentation URL with further information about the topic of the log message. Tools such
    /// as journalctl will include a hyperlink to a URL specified this way in their output. Should
    /// be an "http:///", "https:///", "file:/", "man:" or "info:" URL.
    #[serde(rename = "DOCUMENTATION")]
    #[getset(get = "pub")]
    documentation: Option<String>,

    /// The numeric thread ID (TID) the log message originates from.
    #[serde(rename = "TID")]
    #[getset(get = "pub")]
    tid: Option<String>,

    /// The name of a unit. Used by the system and user managers when logging about specific units.
    ///
    /// When --unit=name or --user-unit=name are used with journalctl(1), a match pattern that
    /// includes "UNIT=name.service" or "USER_UNIT=name.service" will be generated.
    #[serde(rename = "UNIT")]
    #[getset(get = "pub")]
    unit: Option<String>,

    /// The name of a unit. Used by the system and user managers when logging about specific units.
    ///
    /// When --unit=name or --user-unit=name are used with journalctl(1), a match pattern that
    /// includes "UNIT=name.service" or "USER_UNIT=name.service" will be generated.
    #[serde(rename = "USER_UNIT")]
    #[getset(get = "pub")]
    user_unit: Option<String>,

    /// The process, user, and group ID of the process the journal entry originates from formatted
    /// as a decimal string. Note that entries obtained via "stdout" or "stderr" of forked processes
    /// will contain credentials valid for a parent process (that initiated the connection to
    /// systemd-journald).
    #[serde(rename = "_PID")]
    #[getset(get = "pub")]
    pid: Option<String>,

    /// The process, user, and group ID of the process the journal entry originates from formatted
    /// as a decimal string. Note that entries obtained via "stdout" or "stderr" of forked processes
    /// will contain credentials valid for a parent process (that initiated the connection to
    /// systemd-journald).
    #[serde(rename = "_UID")]
    #[getset(get = "pub")]
    uid: Option<String>,

    /// The process, user, and group ID of the process the journal entry originates from formatted
    /// as a decimal string. Note that entries obtained via "stdout" or "stderr" of forked processes
    /// will contain credentials valid for a parent process (that initiated the connection to
    /// systemd-journald).
    #[serde(rename = "_GID")]
    #[getset(get = "pub")]
    gid: Option<String>,

    /// The name, the executable path, and the command line of the process the journal entry
    /// originates from.
    #[serde(rename = "_COMM")]
    #[getset(get = "pub")]
    comm: Option<String>,

    /// The name, the executable path, and the command line of the process the journal entry
    /// originates from.
    #[serde(rename = "_EXE")]
    #[getset(get = "pub")]
    exe: Option<String>,

    /// The name, the executable path, and the command line of the process the journal entry
    /// originates from.
    #[serde(rename = "_CMDLINE")]
    #[getset(get = "pub")]
    cmdline: Option<String>,

    // The effective capabilities(7) of the process the journal entry originates from.
    #[serde(rename = "_CAP_EFFECTIVE")]
    #[getset(get = "pub")]
    cap_effective: Option<String>,

    // The session and login UID of the process the journal entry originates from, as maintained by
    // the kernel audit subsystem.
    #[serde(rename = "_AUDIT_SESSION")]
    #[getset(get = "pub")]
    audit_session: Option<String>,

    // The session and login UID of the process the journal entry originates from, as maintained by
    // the kernel audit subsystem.
    #[serde(rename = "_AUDIT_LOGINUID")]
    #[getset(get = "pub")]
    audit_loginuid: Option<String>,

    // The control group path in the systemd hierarchy, the systemd slice unit name, the systemd
    // unit name, the unit name in the systemd user manager (if any), the systemd session ID (if
    // any), and the owner UID of the systemd user unit or systemd session (if any) of the process
    // the journal entry originates from.
    #[serde(rename = "_SYSTEMD_CGROUP")]
    #[getset(get = "pub")]
    systemd_cgroup: Option<String>,

    // The control group path in the systemd hierarchy, the systemd slice unit name, the systemd
    // unit name, the unit name in the systemd user manager (if any), the systemd session ID (if
    // any), and the owner UID of the systemd user unit or systemd session (if any) of the process
    // the journal entry originates from.
    #[serde(rename = "_SYSTEMD_SLICE")]
    #[getset(get = "pub")]
    systemd_slice: Option<String>,

    // The control group path in the systemd hierarchy, the systemd slice unit name, the systemd
    // unit name, the unit name in the systemd user manager (if any), the systemd session ID (if
    // any), and the owner UID of the systemd user unit or systemd session (if any) of the process
    // the journal entry originates from.
    #[serde(rename = "_SYSTEMD_UNIT")]
    #[getset(get = "pub")]
    systemd_unit: Option<String>,

    // The control group path in the systemd hierarchy, the systemd slice unit name, the systemd
    // unit name, the unit name in the systemd user manager (if any), the systemd session ID (if
    // any), and the owner UID of the systemd user unit or systemd session (if any) of the process
    // the journal entry originates from.
    #[serde(rename = "_SYSTEMD_USER_UNIT")]
    #[getset(get = "pub")]
    systemd_user_unit: Option<String>,

    // The control group path in the systemd hierarchy, the systemd slice unit name, the systemd
    // unit name, the unit name in the systemd user manager (if any), the systemd session ID (if
    // any), and the owner UID of the systemd user unit or systemd session (if any) of the process
    // the journal entry originates from.
    #[serde(rename = "_SYSTEMD_USER_SLICE")]
    #[getset(get = "pub")]
    systemd_user_slice: Option<String>,

    // The control group path in the systemd hierarchy, the systemd slice unit name, the systemd
    // unit name, the unit name in the systemd user manager (if any), the systemd session ID (if
    // any), and the owner UID of the systemd user unit or systemd session (if any) of the process
    // the journal entry originates from.
    #[serde(rename = "_SYSTEMD_SESSION")]
    #[getset(get = "pub")]
    systemd_session: Option<String>,

    // The control group path in the systemd hierarchy, the systemd slice unit name, the systemd
    // unit name, the unit name in the systemd user manager (if any), the systemd session ID (if
    // any), and the owner UID of the systemd user unit or systemd session (if any) of the process
    // the journal entry originates from.
    #[serde(rename = "_SYSTEMD_OWNER_UID")]
    #[getset(get = "pub")]
    systemd_owner_uid: Option<String>,

    // The SELinux security context (label) of the process the journal entry originates from.
    #[serde(rename = "_SELINUX_CONTEXT")]
    #[getset(get = "pub")]
    selinux_context: Option<String>,

    // The earliest trusted timestamp of the message, if any is known that is different from the
    // reception time of the journal. This is the time in microseconds since the epoch UTC,
    // formatted as a decimal string.
    #[serde(rename = "_SOURCE_REALTIME_TIMESTAMP")]
    #[getset(get = "pub")]
    source_realtime_timestamp: Option<String>,

    // The kernel boot ID for the boot the message was generated in, formatted as a 128-bit
    // hexadecimal string.
    #[serde(rename = "_BOOT_ID")]
    #[getset(get = "pub")]
    boot_id: Option<String>,

    // The machine ID of the originating host, as available in machine-id(5).
    #[serde(rename = "_MACHINE_ID")]
    #[getset(get = "pub")]
    machine_id: Option<String>,

    // The invocation ID for the runtime cycle of the unit the message was generated in, as
    // available to processes of the unit in $INVOCATION_ID (see systemd.exec(5)).
    #[serde(rename = "_SYSTEMD_INVOCATION_ID")]
    #[getset(get = "pub")]
    systemd_invocation_id: Option<String>,

    // The name of the originating host.
    #[serde(rename = "_HOSTNAME")]
    #[getset(get = "pub")]
    hostname: Option<String>,

    // How the entry was received by the journal service. Valid transports are:
    //
    #[serde(rename = "_TRANSPORT")]
    #[getset(get = "pub")]
    transport: Transport,

    // Only applies to "_TRANSPORT=stdout" records: specifies a randomized 128-bit ID assigned to
    // the stream connection when it was first created. This ID is useful to reconstruct individual
    // log streams from the log records: all log records carrying the same stream ID originate from
    // the same stream.
    #[serde(rename = "_STREAM_ID")]
    #[getset(get = "pub")]
    stream_id: Option<String>,

    // Only applies to "_TRANSPORT=stdout" records: indicates that the log message in the standard
    // output/error stream was not terminated with a normal newline character ("\n", i.e. ASCII
    // 10). Specifically, when set this field is one of nul (in case the line was terminated by a
    // NUL byte), line-max (in case the maximum log line length was reached, as configured with
    // LineMax= in journald.conf(5)), eof (if this was the last log record of a stream and the
    // stream ended without a final newline character), or pid-change (if the process which
    // generated the log output changed in the middle of a line). Note that this record is not
    // generated when a normal newline character was used for marking the log line end.
    #[serde(rename = "_LINE_BREAK")]
    #[getset(get = "pub")]
    line_break: Option<String>,

    // If this file was written by a systemd-journald instance managing a journal namespace that is
    // not the default, this field contains the namespace identifier. See
    // systemd-journald.service(8) for details about journal namespaces.
    #[serde(rename = "_NAMESPACE")]
    #[getset(get = "pub")]
    namespace: Option<String>,

    // A string field that specifies the runtime scope in which the message was logged. If
    // "initrd", the log message was processed while the system was running inside the initrd. If
    // "system", the log message was generated after the system switched execution to the host root
    // filesystem.
    #[serde(rename = "_RUNTIME_SCOPE")]
    #[getset(get = "pub")]
    runtime_scope: Option<String>,
}
