use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use tokio::sync::Mutex;
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CppQml {
    pub name: String,
    pub signals: Vec<String>,
    pub slots: Vec<[String; 2]>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub qmltype: String,
    pub uri: String,
    #[serde(rename(serialize = "versionMajor", deserialize = "versionMajor"))]
    pub version_major: i32,
    #[serde(rename(serialize = "versionMinor", deserialize = "versionMinor"))]
    pub version_minor: i32,
}

pub static GLOBAL_DATA: Lazy<Mutex<Vec<CppQml>>> = Lazy::new(|| {
    let file = || -> anyhow::Result<Vec<CppQml>> {
        let file = File::open("build/types.json")?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let u = serde_json::from_reader(reader)?;

        // Return the `User`.
        Ok(u)
    };
    let qmltypes: Vec<CppQml> = match file() {
        Ok(types) => types,
        Err(_) => vec![],
    };
    Mutex::new(qmltypes)
});

#[test]
fn read_type() {
    let source = include_str!("../misc/types.json");
    let _a: Vec<CppQml> = serde_json::from_str(source).unwrap();
}
