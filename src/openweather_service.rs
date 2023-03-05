use crate::providers::openweather;
use crate::weather_response;

pub struct OpenweatherService {
    pub service_url: &'static str,
}

impl OpenweatherService {
    pub async fn get(self: &OpenweatherService, location: String) -> Result<weather_response::WeatherResponse,Box<dyn std::error::Error>> { 
        let channel = tonic::transport::Channel::from_static(self.service_url)
        .connect()
        .await?;

        let mut client = openweather::weather_client::WeatherClient::new(channel);
        let request = tonic::Request::new(
            openweather::WeatherRequest { location },
        );
        let response: openweather::WeatherResponse = client.current_conditions(request).await?.into_inner();

        Ok(weather_response::WeatherResponse { temperature: response.temperature as f32, description: response.description, found: response.found })
    }
}