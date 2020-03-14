use crate::cmd_map::{single_hyphenated_version, CommandInfo};

pub const JAVA: CommandInfo = CommandInfo {
    command_name: "java",
    get_version_args: single_hyphenated_version,
};

pub const JAVAC: CommandInfo = CommandInfo {
    command_name: "javac",
    get_version_args: single_hyphenated_version,
};

pub const KOTLINC: CommandInfo = CommandInfo {
    command_name: "kotlinc",
    get_version_args: single_hyphenated_version,
};

pub const SCALAC: CommandInfo = CommandInfo {
    command_name: "scalac",
    get_version_args: single_hyphenated_version,
};
