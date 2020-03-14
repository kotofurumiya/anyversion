use crate::cmd_map::{single_hyphenated_version, CommandInfo};

pub const FFMPEG: CommandInfo = CommandInfo {
    command_name: "ffmpeg",
    get_version_args: single_hyphenated_version,
};

pub const IMAGEMAGIC_CONVERT: CommandInfo = CommandInfo {
    command_name: "convert",
    get_version_args: single_hyphenated_version,
};

pub const IMAGEMAGIC_COMPOSITE: CommandInfo = CommandInfo {
    command_name: "composite",
    get_version_args: single_hyphenated_version,
};
