use crate::cmd_map::{double_hyphenated_version, CommandInfo};

pub const RUSTC: CommandInfo = CommandInfo {
    command_name: "rustc",
    get_version_args: double_hyphenated_version,
};

pub const RUSTFMT: CommandInfo = CommandInfo {
    command_name: "rustfmt",
    get_version_args: double_hyphenated_version,
};

pub const RUSTDOC: CommandInfo = CommandInfo {
    command_name: "rustdoc",
    get_version_args: double_hyphenated_version,
};

pub const CARGO: CommandInfo = CommandInfo {
    command_name: "cargo",
    get_version_args: double_hyphenated_version,
};
