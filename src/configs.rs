use std::env;
use regex::Regex;
use std::default::{Default};
use std::io::{Read, Error, ErrorKind};
use std::path::PathBuf;
use std::fs::File;
use toml;

use digest_ext_table::{DigestAlgo, DigestExtTable};


pub static DEFAULT_MAX_SIZE:u64 = 64 * 1024 * 1024; // 64MB

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


//-- ProxyConfigs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProxyConfigs {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub scheme: Option<String>
}

impl ProxyConfigs {
    //copies fields values into target struct only if has Some value
    fn merge_to(&self, target: &mut ProxyConfigs){
        if self.host.is_some() { target.host = self.host.clone(); }
        if self.port.is_some() { target.port = self.port.clone(); }
        if self.scheme.is_some() { target.scheme = self.scheme.clone(); }
    }

    //checks does it have all the required fields to use it
    pub fn is_complete(&self) -> bool {
        self.host.is_some() && self.port.is_some()
    }
}

impl Default for ProxyConfigs {
    fn default() -> ProxyConfigs {
        ProxyConfigs {
            host: None,
            port: None,
            scheme: None
        }
    }
}

//-- Configs for Digest --------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DigestConfigItem {
    pub blocked: Option<bool>,
    pub exts: Option<Vec<String>>
}

impl DigestConfigItem {
    pub fn new(blocked: bool, exts: Vec<String>) -> DigestConfigItem {
        DigestConfigItem {
            blocked: Some(blocked),
            exts: Some(exts)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DigestConfigs {
    pub md5: Option<DigestConfigItem>,
    pub sha1: Option<DigestConfigItem>,
    pub sha512: Option<DigestConfigItem>
}

impl DigestConfigs {
    pub fn new(
        md5: Option<DigestConfigItem>, sha1: Option<DigestConfigItem>, sha512: Option<DigestConfigItem>
    ) -> DigestConfigs {
        DigestConfigs {
            md5: md5,
            sha1: sha1,
            sha512: sha512
        }
    }

    // turns DigestConfigs into DigestExtTable
    pub fn into_digest_ext_table(&self) -> DigestExtTable {
        let mut ext_table = DigestExtTable::default();

        if let Some(md5_confs) = self.md5.to_owned() {
            self.insert_algo_confs(&mut ext_table, DigestAlgo::Md5, &md5_confs )
        }

        if let Some(sha1_confs) = self.sha1.to_owned() {
            self.insert_algo_confs(&mut ext_table, DigestAlgo::Sha1, &sha1_confs )
        }

        if let Some(sha512_confs) = self.sha512.to_owned() {
            self.insert_algo_confs(&mut ext_table, DigestAlgo::Sha512, &sha512_confs )
        }

        ext_table
    }

    fn insert_algo_confs(&self, ext_table: &mut DigestExtTable, algo: DigestAlgo, config_item: &DigestConfigItem){

        //add algorithm into blocked list only if it blocked fields is specified and equals true
        if let Some(is_blocked) = config_item.blocked {
            if is_blocked == true {
                ext_table.block(algo);
                return; //there's no point to insert extensions for blocked items
            }
        }

        if let Some(exts) = config_item.exts.to_owned() {
            ext_table.clear(algo);
            ext_table.add_many(algo, exts);
        }

    }
}


//-- Scan configs -------------------------------------------------------------
// used to limit file sizes
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanConfigs {
    pub max_file_size: Option<u64>,
    pub min_file_size: Option<u64>
}

impl ScanConfigs {

    //copies fields values into target struct only if has Some value
    fn merge_to(&self, target: &mut ScanConfigs){
        if let Some(new_max_size) = self.max_file_size {
            target.max_file_size = Some( new_max_size );
        }

        if let Some(new_min_size) = self.min_file_size {
            target.min_file_size = Some( new_min_size );
        }

    }

}

impl Default for ScanConfigs {
    fn default() -> ScanConfigs {
        ScanConfigs {
            max_file_size: Some(DEFAULT_MAX_SIZE),
            min_file_size: Some(0)
        }
    }
}

//-- Configs ------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configs {
    pub api: ApiConfigs,
    pub csv: CSVConfigs,
    pub proxy: ProxyConfigs,
    pub digests: DigestExtTable,
    pub scan: ScanConfigs
}

impl Configs {
    fn merge_to(&self, target: &mut Configs) {
        self.api.merge_to(&mut target.api);
        self.csv.merge_to(&mut target.csv);
        self.proxy.merge_to(&mut target.proxy);
        self.scan.merge_to(&mut target.scan);

        target.digests = self.digests.clone();

    }
}

impl Default for Configs {
    fn default() -> Configs {
        Configs {
            api: ApiConfigs::default(),
            csv: CSVConfigs::default(),
            proxy: ProxyConfigs::default(),
            digests: DigestExtTable::default(),
            scan: ScanConfigs::default()
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
    let re_api_key   = Regex::new(r"\AVERSIONEYE_API_(\w+)\z").unwrap();
    let re_csv_key   = Regex::new(r"\AVERSIONEYE_CSV_(\w+)\z").unwrap();
    let re_proxy_key = Regex::new(r"\AVERSIONEYE_PROXY_(\w+)\z").unwrap();
    let re_scan_key  = Regex::new(r"\AVERSIONEYE_SCAN_(\w+)\z").unwrap();

    let mut configs = Configs::default();

    for (key, val) in env::vars() {

        // read API configs
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

        //read proxy configs
        if let Some(m) = re_proxy_key.captures(&key) {
            let proxy_val = val.clone();

            match m.get(1).unwrap().as_str() {
                "HOST"      => configs.proxy.host = Some(proxy_val),
                "PORT"      => configs.proxy.port = proxy_val.parse::<u16>().ok(),
                "SCHEME"    => configs.proxy.scheme = Some(proxy_val),
                _           => ()
            }
        }

        //read scan configs
        if let Some(m) = re_scan_key.captures(&key){
            let scan_val = val.clone();

            match m.get(1).unwrap().as_str() {
                "MAX_FILE_SIZE" => configs.scan.max_file_size = scan_val.parse::<u64>().ok(),
                "MIN_FILE_SIZE" => configs.scan.min_file_size = scan_val.parse::<u64>().ok(),
                _               => ()
            }
        }

    }

    Ok(configs)
}

#[derive(Deserialize, Debug)]
struct TomlConfigs {
    api: Option<ApiConfigs>,
    csv: Option<CSVConfigs>,
    proxy: Option<ProxyConfigs>,
    digests: Option<DigestConfigs>,
    scan: Option<ScanConfigs>
}

impl TomlConfigs {

    //move optional values into Configs structure
    fn into_configs(&self) -> Configs {
        let mut confs = Configs::default();

        //TODO: how to get rid of those clone()'s
        if let Some(toml_api) = self.api.clone() {
            confs.api = toml_api;
        }

        if let Some(toml_csv) = self.csv.clone() {
            confs.csv = toml_csv;
        }

        if let Some(toml_proxy) = self.proxy.clone() {
            confs.proxy = toml_proxy;
        }

        if let Some(toml_digests) = self.digests.clone() {
            confs.digests = toml_digests.into_digest_ext_table();
        }

        if let Some(toml_scan) = self.scan.clone() {
            confs.scan = toml_scan;
        }

        confs.clone()
    }
}

pub fn read_configs_from_toml(file_path: &PathBuf) -> Result<Configs, Error> {
    let mut toml_file = File::open(file_path)?;
    let mut toml_txt = String::new();
    toml_file.read_to_string(&mut toml_txt)?;

    match toml::from_str::<TomlConfigs>(toml_txt.as_str()) {
        Ok(toml_configs) => Ok(toml_configs.into_configs()),
        Err(_) => {
            Err(
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Failed to extract config data from TOML {:?}", file_path.as_os_str())
                )
            )
        }

    }
}

