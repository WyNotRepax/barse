use std::{collections::HashMap, env, fs::File, io::Write, path::PathBuf};

use php_exec::{PhpExec, PhpResult};

fn main() {
    let mut php = PhpExec::new().unwrap();
    let constants = get_constants(&mut php);
    create_constants(&constants);
    create_token_name(&constants);
}

fn get_constants(php: &mut PhpExec) -> HashMap<String, u32> {
    match php
        .exec::<HashMap<String, u32>>("get_defined_constants(true)['tokenizer']".to_string())
        .unwrap()
    {
        PhpResult::Ok { result, .. } => result,
        PhpResult::Error { message, .. } => {
            panic!("{message}");
        }
    }
}

fn create_constants(constants: &HashMap<String, u32>) {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut file = File::create(out_dir.join("constants.rs")).unwrap();
    for (constant_name, constant_value) in constants {
        file.write_fmt(format_args!(
            "pub const {constant_name}: u32 = {constant_value};\n"
        ))
        .unwrap();
    }
}
fn create_token_name(constants: &HashMap<String, u32>) {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut file = File::create(out_dir.join("token_name.rs")).unwrap();
    file.write_all(b"use crate::constants::*;\n").unwrap();
    file.write_all(b"use serde_repr::{Deserialize_repr, Serialize_repr};\n\n")
        .unwrap();
    file.write_all(b"#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]\n")
        .unwrap();
    file.write_all(b"#[repr(u32)]\n").unwrap();
    file.write_all(b"pub enum TokenName {\n").unwrap();
    let mut constants = constants.iter().collect::<Vec<_>>();
    constants.sort_by_key(|(_, i)| **i);
    for (constant_name, _) in constants {
        if !constant_name.starts_with("T_") || constant_name == "T_PAAMAYIM_NEKUDOTAYIM" {
            continue;
        }
        let variant_name = constant_name_to_variant_name(constant_name);
        file.write_fmt(format_args!("    {variant_name} = {constant_name},\n"))
            .unwrap();
    }
    file.write_all(b"}\n").unwrap();
}

fn constant_name_to_variant_name(name: &str) -> String {
    name[2..]
        .split("_")
        .flat_map(|s| [s[..1].to_uppercase(), s[1..].to_lowercase()])
        .collect()
}
