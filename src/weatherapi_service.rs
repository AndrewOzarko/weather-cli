use crate::providers::weatherapi;
use crate::weather_response;

pub struct WeatherapiService {
    pub api_key: String,
    pub date: String,
}

impl WeatherapiService {
    pub async fn get(self: &WeatherapiService, location: String) -> Result<weather_response::WeatherResponse,Box<dyn std::error::Error>> { 
        let client = weatherapi::weather_client::new(self.api_key.to_string());
        let request = weatherapi::WeatherRequest{ location, date: self.date.to_string() };
        let response = client.query(request).await?;
        
        Ok(weather_response::WeatherResponse { temperature: response.temperature, description: response.description, found: true })
    }

    pub fn set_date(&mut self, date: String) -> Result<(), Box<dyn std::error::Error>> {
        // if date not formatted by regex yyyy-mm-dd
        let regex = regex::Regex::new(r"^(\d{4})-(0[1-9]|1[0-2]|[1-9])-([1-9]|0[1-9]|[1-2]\d|3[0-1])$").unwrap();
        
        if regex.is_match(&date) {
            self.date = date;
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid date format",
            )))
        }
    }
}
