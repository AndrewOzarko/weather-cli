pub struct WeatherRequest {
    pub location: String,
    pub date: String,
}
#[derive(Debug)]
pub struct WeatherResponse {
    pub temperature: f32,
    pub description: String,
}

pub mod weather_client {
    use serde_derive::Deserialize;
    use serde_derive::Serialize;
    pub struct WeatherClient {
        pub api_key: String,
    }

    #[derive(Debug, Deserialize)]
    struct WeatherapiErrorResponse {
        error: WeatherapiError,
    }
    
    #[derive(Debug, Deserialize)]
    struct WeatherapiError {
        message: String,
    }

    impl WeatherClient {
        pub async fn query(
            &self,
            req: super::WeatherRequest,
        ) -> Result<super::WeatherResponse, Box<dyn std::error::Error>> {
            let client = reqwest::Client::new();
            let mut endpoint = "current.json";
            if req.date != "now" {
                endpoint = "history.json"
            }
        
            let mut url = format!(
                "http://api.weatherapi.com/v1/{}?key={}&q={}",
                endpoint, self.api_key, req.location
            );
        
            if req.date != "now" {
                url += &format!("&dt={}", req.date);
            }
        
            let response = client.get(&url).send().await?;
            if response.status().is_success() {
                let wr: WeatherapiResponse = response.json().await?;
                Ok(super::WeatherResponse {
                    temperature: wr.current.temp_c as f32,
                    description: wr.current.condition.text,
                })
            } else {
                let error_response: WeatherapiErrorResponse = response.json().await?;
                let error_message = error_response.error.message;
                Err(error_message.into())
            }
        }
    }

    pub fn new(api_key: String) -> WeatherClient {
        WeatherClient { api_key }
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WeatherapiResponse {
        pub location: Location,
        pub current: Current,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Location {
        pub name: String,
        pub region: String,
        pub country: String,
        pub lat: f64,
        pub lon: f64,
        #[serde(rename = "tz_id")]
        pub tz_id: String,
        #[serde(rename = "localtime_epoch")]
        pub localtime_epoch: i64,
        pub localtime: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Current {
        #[serde(rename = "last_updated_epoch")]
        pub last_updated_epoch: i64,
        #[serde(rename = "last_updated")]
        pub last_updated: String,
        #[serde(rename = "temp_c")]
        pub temp_c: f64,
        #[serde(rename = "temp_f")]
        pub temp_f: f64,
        #[serde(rename = "is_day")]
        pub is_day: i64,
        pub condition: Condition,
        #[serde(rename = "wind_mph")]
        pub wind_mph: f64,
        #[serde(rename = "wind_kph")]
        pub wind_kph: f64,
        #[serde(rename = "wind_degree")]
        pub wind_degree: i64,
        #[serde(rename = "wind_dir")]
        pub wind_dir: String,
        #[serde(rename = "pressure_mb")]
        pub pressure_mb: f64,
        #[serde(rename = "pressure_in")]
        pub pressure_in: f64,
        #[serde(rename = "precip_mm")]
        pub precip_mm: f64,
        #[serde(rename = "precip_in")]
        pub precip_in: f64,
        pub humidity: i64,
        pub cloud: i64,
        #[serde(rename = "feelslike_c")]
        pub feelslike_c: f64,
        #[serde(rename = "feelslike_f")]
        pub feelslike_f: f64,
        #[serde(rename = "vis_km")]
        pub vis_km: f64,
        #[serde(rename = "vis_miles")]
        pub vis_miles: f64,
        pub uv: f64,
        #[serde(rename = "gust_mph")]
        pub gust_mph: f64,
        #[serde(rename = "gust_kph")]
        pub gust_kph: f64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Condition {
        pub text: String,
        pub icon: String,
        pub code: i64,
    }

}