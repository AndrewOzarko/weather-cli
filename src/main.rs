#![feature(async_fn_in_trait)]

mod providers;
pub mod weather_response;
pub mod openweather_service;
pub mod weatherapi_service;
pub mod arg_parser;
pub mod commands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = arg_parser::make_arg_mathes();
    if let Some(matches) = matches.subcommand_matches("get") {
        commands::get_command(matches).await?
    } else if let Some(matches) = matches.subcommand_matches("configure") {
        commands::configure_command(matches).await?;
    } else {
        println!("Try to use help");
    }
    Ok(())
}

