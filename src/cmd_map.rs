use std::collections::HashMap;

mod alt_js;
mod cmd_list;
mod container;
mod database;
mod dev_essentials;
mod editor;
mod firebase;
mod gcloud;
mod go;
mod jvm;
mod linux_essentials;
mod media;
mod nodejs;
mod python;
mod rust;

#[derive(Copy, Clone)]
pub struct CommandInfo {
    pub command_name: &'static str,
    pub get_version_args: fn() -> Vec<&'static str>,
}

fn single_hyphenated_version() -> Vec<&'static str> {
    return vec!["-version"];
}

fn double_hyphenated_version() -> Vec<&'static str> {
    return vec!["--version"];
}

fn hyphenated_capital_v_version() -> Vec<&'static str> {
    return vec!["-V"];
}

fn sub_command_version() -> Vec<&'static str> {
    return vec!["version"];
}

pub fn build_command_map() -> HashMap<String, CommandInfo> {
    let mut cmd_map = HashMap::new();

    for cmd in cmd_list::get_cmd_list().iter() {
        let cmd_name = cmd.command_name.to_string();
        cmd_map.insert(cmd_name, *cmd);
    }

    cmd_map
}

#[test]
fn test_rust_toolchain() {
    let map = build_command_map();
    let rustc = map.get("rustc").expect("error!");
    let cargo = map.get("cargo").expect("error!");

    assert!(rustc.command_name == "rustc");
    assert!((rustc.get_version_args)()[0] == "--version");

    assert!(cargo.command_name == "cargo");
    assert!((cargo.get_version_args)()[0] == "--version");
}
