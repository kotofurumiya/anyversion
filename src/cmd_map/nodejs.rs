use crate::cmd_map::{double_hyphenated_version, CommandInfo};

pub const NODE: CommandInfo = CommandInfo {
    command_name: "node",
    get_version_args: double_hyphenated_version,
};

pub const NODEJS: CommandInfo = CommandInfo {
    command_name: "nodejs",
    get_version_args: double_hyphenated_version,
};

pub const NPM: CommandInfo = CommandInfo {
    command_name: "npm",
    get_version_args: double_hyphenated_version,
};

pub const NPX: CommandInfo = CommandInfo {
    command_name: "npx",
    get_version_args: double_hyphenated_version,
};

pub const YARN: CommandInfo = CommandInfo {
    command_name: "yarn",
    get_version_args: double_hyphenated_version,
};
