use std::env;
use regex::Regex;
use std::default::{Default};
use std::io::{Read, Error, ErrorKind};
use std::path::Path;
use std::fs::File;
use serde::{Serialize, Deserialize};
//use serde_derive;
use toml;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfigs {
    pub host: Option<String>,
    pub path: Option<String>,
    pub key: Option<String>,
    pub port: Option<u32>,
    pub scheme: Option<String>
}

impl ApiConfigs {
    //copies self field values into target only it's not None value
    fn merge_to(&self, target: &mut ApiConfigs){
        if self.host.is_some() { target.host = self.host.clone(); }
        if self.path.is_some() { target.path = self.path.clone(); }
        if self.key.is_some()  { target.key  = self.key.clone(); }
        if self.port.is_some() { target.port = self.port.clone(); }
        if self.scheme.is_some() { target.scheme = self.scheme.clone(); }
    }
}

impl Default for ApiConfigs {
    fn default() -> ApiConfigs {
        ApiConfigs {
            host: Some( "www.versioneye.com".to_string() ),
            path: Some("api/v2".to_string()),
            key: None,
            port: None,
            scheme: Some("https".to_string())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configs {
    pub api: ApiConfigs
}

impl Configs {
    fn merge_to(&self, target: &mut Configs) {
        self.api.merge_to(&mut target.api);
    }
}

impl Default for Configs {
    fn default() -> Configs {
        Configs { api: ApiConfigs::default() }
    }
}


pub fn read_configs() -> Configs {
    let conf_file = Path::new("veye_checker.toml");
    let mut confs = Configs::default();

    //all every config reader overwrites previous values
    match read_configs_from_toml(conf_file) {
        Ok(toml_confs)  => toml_confs.merge_to(&mut confs),
        Err(_)          => ()
    }

    match read_api_configs_from_env() {
        Ok(env_confs)  => env_confs.merge_to(&mut confs),
        Err(_)         => ()
    }

    confs
}

pub fn read_api_configs_from_env() -> Result<Configs, Error> {
    let re_api_key = Regex::new(r"\AVERSIONEYE_API_(\w+)\z").unwrap();
    let mut configs = Configs::default();

    for (key, val) in env::vars() {
        if let Some(m) = re_api_key.captures(&key) {
            match m.get(1).unwrap().as_str() {
                "KEY"    => configs.api.key = Some(val),
                "HOST"   => configs.api.host = Some(val),
                "PORT"   => configs.api.port = val.parse::<u32>().ok(),
                "PATH"   => configs.api.path = Some(val),
                "SCHEME" => configs.api.scheme = Some(val),
                _ => ()
            }
        }
    }

    Ok(configs)
}

pub fn read_configs_from_toml(file_path: &Path) -> Result<Configs, Error> {
    //todo: check does the file exists
    let mut toml_file = File::open(file_path)?;
    let mut toml_txt = String::new();
    toml_file.read_to_string(&mut toml_txt)?;

    match toml::from_str(toml_txt.as_str()) {
        Err(e) => {
            Err(
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Failed to extract config data from TOML {}", e)
                )
            )
        },
        Ok(configs) => Ok(configs)
    }
}

