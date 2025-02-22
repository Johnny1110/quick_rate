use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn get_settings(config_path: &str) -> HashMap<String, String> {
    match read_yaml_to_hashmap(config_path) {
        Ok(config) => {
            //dbg!("Config Loaded: {:?}", &config);
            config
        }
        Err(e) => {
            dbg!("Failed to read config: {}", e);
            panic!("Failed to read config");
        }
    }
}

fn read_yaml_to_hashmap(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: HashMap<String, String> = serde_yaml::from_str(&contents)?;
    Ok(config)
}