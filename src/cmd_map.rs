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
mod ruby;
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
    cmd_list::COMMAND_LIST.iter().map(|ci| (ci.command_name.to_string(), *ci)).collect()
}

#[test]
fn test_rust_toolchain() {
    let map = build_command_map();
    let rustc = map.get("rustc").expect("error!");
    let cargo = map.get("cargo").expect("error!");

    assert_eq!(rustc.command_name, "rustc");
    assert_eq!((rustc.get_version_args)()[0], "--version");

    assert_eq!(cargo.command_name, "cargo");
    assert_eq!((cargo.get_version_args)()[0],"--version");
}
