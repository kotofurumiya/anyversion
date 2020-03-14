use crate::cmd_map::{sub_command_version, CommandInfo};

pub const GO: CommandInfo = CommandInfo {
    command_name: "go",
    get_version_args: sub_command_version,
};

pub const GOENV: CommandInfo = CommandInfo {
    command_name: "goenv",
    get_version_args: sub_command_version,
};
