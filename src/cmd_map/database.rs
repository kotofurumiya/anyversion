use crate::cmd_map::{double_hyphenated_version, CommandInfo};

pub const MYSQL: CommandInfo = CommandInfo {
    command_name: "mysql",
    get_version_args: double_hyphenated_version,
};

pub const PSQL: CommandInfo = CommandInfo {
    command_name: "psql",
    get_version_args: double_hyphenated_version,
};

pub const SQLITE3: CommandInfo = CommandInfo {
    command_name: "sqlite3",
    get_version_args: double_hyphenated_version,
};

pub const MONGO: CommandInfo = CommandInfo {
    command_name: "mongo",
    get_version_args: double_hyphenated_version,
};

pub const REDIS: CommandInfo = CommandInfo {
    command_name: "redis-server",
    get_version_args: double_hyphenated_version,
};
