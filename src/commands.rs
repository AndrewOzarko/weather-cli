use clap::ArgMatches;
use std::io::{Read, Write};

use serde::{Serialize, Deserialize};

use crate::{openweather_service, weatherapi_service};

#[derive(Debug, Serialize, Deserialize)]
struct WeatherapiConfig {
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenweatherConfig {
    grpc_url: String,
}


pub async fn get_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let provider = match matches.value_of("provider") {
        None => "weatherapi".to_string(),
        Some(str) => str.to_string(),
    };
    let location = match matches.value_of("location") {
        None => "".to_string(),
        Some(str) => str.to_string(),
    };
    
    let date = matches.value_of("date").unwrap();

    let response = match provider.as_str() {
        "openweather" => {
            if date != "now" {
                panic!("openweather local grpc provider couldn't support date filtering");
            }
            
            let home_dir = home::home_dir().unwrap();
            let path = format!("{}/.weather-cli/{}/config.json", home_dir.to_string_lossy(), "openweather");
        
            let config_exists = std::path::Path::new(&path).exists();
            if !config_exists {
                panic!("Need to configure openweather provider before use.")
            }
        
            let mut file = std::fs::File::open(path.clone())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let current_config: OpenweatherConfig = match serde_json::from_str(&contents) {
                Ok(config) => config,
                Err(e) => {
                    _ = e;
                    OpenweatherConfig{grpc_url: String::from("")}
                }
            };

            let op = openweather_service::OpenweatherService{service_url: Box::leak(current_config.grpc_url.into_boxed_str())};
            op.get(location).await?
        },
        "weatherapi" => {


            let home_dir = home::home_dir().unwrap();
            let path = format!("{}/.weather-cli/{}/config.json", home_dir.to_string_lossy(), "weatherapi");
        
            let config_exists = std::path::Path::new(&path).exists();
            if !config_exists {
                panic!("Need to configure weatherapi provider before use.")
            }
        
            let mut file = std::fs::File::open(path.clone())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let current_config: WeatherapiConfig = match serde_json::from_str(&contents) {
                Ok(config) => config,
                Err(e) => {
                    _ = e;
                    WeatherapiConfig{api_key: String::from("")}
                }
            };

            let mut wa = weatherapi_service::WeatherapiService{api_key: current_config.api_key, date: String::from("now")};
            
            if date != "now" {
                match wa.set_date(date.to_string()) {
                    Ok(_) => (),
                    Err(err) => {
                        panic!("Error setting date: {}", err);
                    }
                }
            }

            wa.get(location).await?
        },
        &_ => todo!(),
    };

    println!("temperature: {}, description: {}, found: {}", response.temperature, response.description, response.found);
    Ok(())
}


pub async fn configure_weatherapi(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(path.clone())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let current_config: WeatherapiConfig = match serde_json::from_str(&contents) {
                Ok(config) => config,
                Err(e) => {
                    _ = e;
                    WeatherapiConfig{api_key: String::from("")} 
                }
            };

            println!("Current API key: {}", current_config.api_key);
            
            println!("Enter your API key:");
            let mut api_key = String::new();
            std::io::stdin().read_line(&mut api_key).expect("Failed to read line");
        
            let config = WeatherapiConfig { api_key: api_key.trim().to_string() };
        
            println!("API key set to: {}", config.api_key);

            let json_string = serde_json::to_string(&config)?;
            let mut file = std::fs::File::create(path)?;
            file.write_all(json_string.as_bytes())?;

            println!("Saved");
            Ok(())
}

pub async fn configure_openweather(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(path.clone())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let current_config: OpenweatherConfig = match serde_json::from_str(&contents) {
                Ok(config) => config,
                Err(e) => {
                    _ = e;
                    OpenweatherConfig{grpc_url: String::from("")} 
                }
            };

            println!("Current openweather grpc service address: {}", current_config.grpc_url);
            
            println!("Enter new openweather grpc service address:");
            let mut grpc_url = String::new();
            std::io::stdin().read_line(&mut grpc_url).expect("Failed to read line");
        
            let config = OpenweatherConfig { grpc_url: grpc_url.trim().to_string() };
        
            println!("Openweather grpc service address set to: {}", config.grpc_url);

            let json_string = serde_json::to_string(&config)?;
            let mut file = std::fs::File::create(path)?;
            file.write_all(json_string.as_bytes())?;

            println!("Saved");
            Ok(())
}

pub async fn configure_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {

    let provider = match matches.value_of("provider") {
        None => "".to_string(),
        Some(str) => str.to_string(),
    };
    
    let home_dir = home::home_dir().unwrap();
    let path = format!("{}/.weather-cli/{}/config.json", home_dir.to_string_lossy(), provider);

    let config_exists = std::path::Path::new(&path).exists();
    if !config_exists {
        let path = std::path::Path::new(&path);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        std::fs::File::create(path)?;
    }

    match provider.as_str() {
        "weatherapi" => {
            configure_weatherapi(path).await?;
        },
        "openweather" => {
            configure_openweather(path).await?;
        },
        _ => todo!(),
    }

    Ok(())
}
