#[derive(Debug)]
pub struct WeatherResponse {
    pub temperature: f32,
    pub description: String,
    pub found: bool,
}
