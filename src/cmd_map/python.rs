use crate::cmd_map::{hyphenated_capital_v_version, CommandInfo};

pub const PYTHON: CommandInfo = CommandInfo {
    command_name: "python",
    get_version_args: hyphenated_capital_v_version,
};

pub const PYTHON2: CommandInfo = CommandInfo {
    command_name: "python2",
    get_version_args: hyphenated_capital_v_version,
};

pub const PYTHON3: CommandInfo = CommandInfo {
    command_name: "python3",
    get_version_args: hyphenated_capital_v_version,
};

pub const PIP: CommandInfo = CommandInfo {
    command_name: "pip",
    get_version_args: hyphenated_capital_v_version,
};

pub const PIP2: CommandInfo = CommandInfo {
    command_name: "pip2",
    get_version_args: hyphenated_capital_v_version,
};

pub const PIP3: CommandInfo = CommandInfo {
    command_name: "pip3",
    get_version_args: hyphenated_capital_v_version,
};
