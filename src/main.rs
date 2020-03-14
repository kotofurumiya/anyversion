use std::env;
use std::process::{exit, Command};

mod cmd_map;

fn main() {
    // ger command name from args.
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1);
    let cmd_name = cmd.expect("Too few arguments");

    // print anyversion itself version.
    if cmd_name == "anyversion" {
        println!("anyversion version {}", env!("CARGO_PKG_VERSION"));
        return;
    };

    // get command-specific version arg from map.
    let cmd_map = cmd_map::build_command_map();
    let cmd_info = match cmd_map.get(cmd_name) {
        Some(c) => c,
        None => {
            eprintln!("Command `{}` is not supported", cmd_name);
            exit(1);
        }
    };

    // execute to get version.
    let exec = Command::new(cmd_info.command_name)
        .args((cmd_info.get_version_args)())
        .output();

    // print it.
    match exec {
        Ok(o) => {
            let stdout = String::from_utf8(o.stdout).expect("");
            let stderr = String::from_utf8(o.stderr).expect("");
            println!("{}", stdout);
            println!("{}", stderr);
        }
        Err(e) => eprintln!("{}", e),
    }
}
