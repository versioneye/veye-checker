use std::env;
use regex::Regex;
use std::default::{Default};
use std::io::{Read, Error, ErrorKind};
use std::path::PathBuf;
use std::fs::File;
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

//-- CSVConfigs ---------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CSVConfigs {
    pub separator: Option<String>,
    pub quote: Option<String>, // which character to use for quoted string
    pub flexible: Option<bool>, //doesnt include empty fields. None = False, only Some(true) is true
}

impl CSVConfigs {
    //copies fields values into target only if it's not None value and overwrites existing value
    fn merge_to(&self, target: &mut CSVConfigs){
        if self.separator.is_some() { target.separator = self.separator.clone(); }
        if self.quote.is_some() { target.quote = self.quote.clone(); }
        if self.flexible.is_some() { target.flexible = self.flexible.clone(); }
    }
}

impl Default for CSVConfigs {
    fn default() -> CSVConfigs {
        CSVConfigs {
            separator: Some(";".to_string()),
            quote: Some("\"".to_string()),
            flexible: Some(false)
        }
    }
}

//-- Configs ------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct Configs {
    pub api: ApiConfigs,
    pub csv: CSVConfigs
}

impl Configs {
    fn merge_to(&self, target: &mut Configs) {
        self.api.merge_to(&mut target.api);
        self.csv.merge_to(&mut target.csv);
    }
}

impl Default for Configs {
    fn default() -> Configs {
        Configs {
            api: ApiConfigs::default(),
            csv: CSVConfigs::default()
        }
    }
}


pub fn read_configs(filepath: Option<String>) -> Configs {
    let conf_filepath = filepath.unwrap_or("veye_checker.toml".to_string());
    let conf_file = PathBuf::from(conf_filepath.clone());
    let mut confs = Configs::default();

    //all every config reader overwrites previous values
    match read_configs_from_toml(&conf_file) {
        Ok(toml_confs)  => toml_confs.merge_to(&mut confs),
        Err(_)          => ()
    };

    match read_configs_from_env() {
        Ok(env_confs)  => env_confs.merge_to(&mut confs),
        Err(_)         => ()
    };

    confs
}

pub fn read_configs_from_env() -> Result<Configs, Error> {
    let re_api_key = Regex::new(r"\AVERSIONEYE_API_(\w+)\z").unwrap();
    let re_csv_key = Regex::new(r"\AVERSIONEYE_CSV_(\w+)\z").unwrap();

    let mut configs = Configs::default();

    for (key, val) in env::vars() {
        if let Some(m) = re_api_key.captures(&key) {
            let api_val = val.clone();

            match m.get(1).unwrap().as_str() {
                "KEY"    => configs.api.key = Some(api_val),
                "HOST"   => configs.api.host = Some(api_val),
                "PORT"   => configs.api.port = api_val.parse::<u32>().ok(),
                "PATH"   => configs.api.path = Some(api_val),
                "SCHEME" => configs.api.scheme = Some(api_val),
                _ => ()
            }
        };

        //read csv configs
        if let Some(m) = re_csv_key.captures(&key) {
            let csv_val = val.clone();

            match m.get(1).unwrap().as_str() {
                "SEPARATOR" => configs.csv.separator = Some(csv_val),
                "QUOTE"     => configs.csv.quote = Some(csv_val),
                "FLEXIBLE"  => {
                    let flex_val = csv_val.clone().to_string().to_lowercase();
                    let is_flexible = match flex_val.as_str() {
                        "1"     => true,
                        "t"     => true,
                        "true"  => true,
                        _       => false
                    };

                    configs.csv.flexible = Some(is_flexible)
                },
                _ => () //ignore unsupported csv keys
            }
        };
    }

    Ok(configs)
}

pub fn read_configs_from_toml(file_path: &PathBuf) -> Result<Configs, Error> {
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

