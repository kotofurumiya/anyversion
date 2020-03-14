use crate::cmd_map::{double_hyphenated_version, CommandInfo};

pub const TSC: CommandInfo = CommandInfo {
    command_name: "tsc",
    get_version_args: double_hyphenated_version,
};

pub const ELM: CommandInfo = CommandInfo {
    command_name: "elm",
    get_version_args: double_hyphenated_version,
};
