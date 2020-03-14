use crate::cmd_map::{double_hyphenated_version, CommandInfo};

pub const NANO: CommandInfo = CommandInfo {
    command_name: "nano",
    get_version_args: double_hyphenated_version,
};

pub const VI: CommandInfo = CommandInfo {
    command_name: "vi",
    get_version_args: double_hyphenated_version,
};

pub const VIM: CommandInfo = CommandInfo {
    command_name: "vim",
    get_version_args: double_hyphenated_version,
};
