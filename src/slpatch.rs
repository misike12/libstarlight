/*
.slpatch and hexpatching handler
*/
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{Error, Read, Seek, SeekFrom},
};
use windows::Win32::System::SystemInformation::*;

pub type PatchData = Vec<(String, String)>;

#[derive(Serialize, Deserialize)]
pub struct Patch {
    pub module: String,
    pub patterns: HashMap<String, PatchData>,
}

#[derive(Serialize, Deserialize)]
pub struct PatchRoot {
    pub name: String,
    pub version: String,
    pub process: String,
    pub patches: Vec<Patch>,
}

pub fn open_slpatch(path: &str) -> Result<PatchRoot, Error> {
    let data: PatchRoot;
    let res = fs::read_to_string(path)?;
    data = serde_json::from_str(&res)?;
    Ok(data)
}

pub fn check_machine(filename: &str) -> Result<String, String> {
    let mut file = fs::File::open(filename).map_err(|_| "Failed to open file".to_string())?;

    let mut buffer = [0; 4];
    let _ = file.seek(SeekFrom::Start(0x3C));
    _ = file.read_exact(&mut buffer);
    let coff_offset = u32::from_le_bytes(buffer);

    _ = file.seek(SeekFrom::Start(coff_offset as u64));
    _ = file.read_exact(&mut buffer);
    _ = file.read_exact(&mut buffer);
    let machine = u16::from_le_bytes([buffer[0], buffer[1]]);
    match IMAGE_FILE_MACHINE(machine) {
        IMAGE_FILE_MACHINE_AMD64 => Ok("amd64".to_string()),
        IMAGE_FILE_MACHINE_I386 => Ok("i386".to_string()),
        IMAGE_FILE_MACHINE_ARM | IMAGE_FILE_MACHINE_ARMNT => Ok("arm".to_string()),
        IMAGE_FILE_MACHINE_ARM64 => Ok("arm64".to_string()),
        _ => Err("Unsupported machine header".to_string()),
    }
}

pub fn patch_module(patches: &PatchData, content: &Vec<u8>) -> Result<Vec<u8>, String> {
    let mut hexdata: String = hex::encode(content);
    for (_0, _1) in patches {
        let (pattern, subst) = (
            _0.replace(" ", "").to_lowercase(),
            _1.replace(" ", "").to_lowercase(),
        );
        let regex = Regex::new(format!("(?mix){}", pattern).as_str())
            .map_err(|_| "Invalid hex pattern".to_string())?;
        hexdata = regex.replace_all(&hexdata, subst).to_string();
    }
    hex::decode(&hexdata).map_err(|_| "Patch is corrupt".to_string())
}
