use crate::cmd_map::{sub_command_version, CommandInfo};

pub const GCLOUD: CommandInfo = CommandInfo {
    command_name: "gcloud",
    get_version_args: sub_command_version,
};

pub const BQ: CommandInfo = CommandInfo {
    command_name: "bq",
    get_version_args: sub_command_version,
};

pub const GSUTIL: CommandInfo = CommandInfo {
    command_name: "gsutil",
    get_version_args: sub_command_version,
};
