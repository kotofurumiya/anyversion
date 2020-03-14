use crate::cmd_map::{double_hyphenated_version, sub_command_version, CommandInfo};

pub const SH: CommandInfo = CommandInfo {
    command_name: "sh",
    get_version_args: double_hyphenated_version,
};

pub const BASH: CommandInfo = CommandInfo {
    command_name: "bash",
    get_version_args: double_hyphenated_version,
};

pub const ZSH: CommandInfo = CommandInfo {
    command_name: "zsh",
    get_version_args: double_hyphenated_version,
};

pub const FISH: CommandInfo = CommandInfo {
    command_name: "fish",
    get_version_args: double_hyphenated_version,
};

pub const CURL: CommandInfo = CommandInfo {
    command_name: "curl",
    get_version_args: double_hyphenated_version,
};

pub const WGET: CommandInfo = CommandInfo {
    command_name: "wget",
    get_version_args: double_hyphenated_version,
};

pub const SCREEN: CommandInfo = CommandInfo {
    command_name: "screen",
    get_version_args: double_hyphenated_version,
};

pub const OPENSSL: CommandInfo = CommandInfo {
    command_name: "openssl",
    get_version_args: sub_command_version,
};
