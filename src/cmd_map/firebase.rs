use crate::cmd_map::{double_hyphenated_version, CommandInfo};

pub const FIREBASE: CommandInfo = CommandInfo {
    command_name: "firebase",
    get_version_args: double_hyphenated_version,
};
