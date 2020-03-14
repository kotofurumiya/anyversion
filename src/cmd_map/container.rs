use crate::cmd_map::{sub_command_version, CommandInfo};

pub const DOCKER: CommandInfo = CommandInfo {
    command_name: "docker",
    get_version_args: sub_command_version,
};

pub const DOCKER_COMPOSE: CommandInfo = CommandInfo {
    command_name: "docker-compose",
    get_version_args: sub_command_version,
};

pub const KUBECTL: CommandInfo = CommandInfo {
    command_name: "kubectl",
    get_version_args: sub_command_version,
};
