use crate::cmd_map::{double_hyphenated_version, CommandInfo};

pub const GIT: CommandInfo = CommandInfo {
    command_name: "git",
    get_version_args: double_hyphenated_version,
};

pub const GCC: CommandInfo = CommandInfo {
    command_name: "gcc",
    get_version_args: double_hyphenated_version,
};

pub const GPP: CommandInfo = CommandInfo {
    command_name: "g++",
    get_version_args: double_hyphenated_version,
};

pub const CLANG: CommandInfo = CommandInfo {
    command_name: "clang",
    get_version_args: double_hyphenated_version,
};

pub const MAKE: CommandInfo = CommandInfo {
    command_name: "make",
    get_version_args: double_hyphenated_version,
};

pub const CMAKE: CommandInfo = CommandInfo {
    command_name: "cmake",
    get_version_args: double_hyphenated_version,
};
