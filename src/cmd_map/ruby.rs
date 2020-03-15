use crate::cmd_map::{double_hyphenated_version, CommandInfo};

pub const RUBY: CommandInfo = CommandInfo {
    command_name: "ruby",
    get_version_args: double_hyphenated_version,
};

pub const GEM: CommandInfo = CommandInfo {
    command_name: "gem",
    get_version_args: double_hyphenated_version,
};

pub const BUNDLE: CommandInfo = CommandInfo {
    command_name: "bundle",
    get_version_args: double_hyphenated_version,
};
