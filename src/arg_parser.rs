use clap::{App, Arg, SubCommand};
use clap::ArgMatches;

pub fn make_arg_mathes() -> ArgMatches {
    App::new("Weather app")
    .version("0.1.0")
    .author("Your Name andrewozarko aka sweetjew")
    .about("Get weather information for a location")
    .subcommand(
        SubCommand::with_name("get")
            .about("Get the weather for a location")
            .arg(
                Arg::with_name("location")
                    .required(true)
                    .index(1)
                    .help("The location to get the weather for"),
            )
            .arg(
                Arg::with_name("provider")
                    .short('p')
                    .long("provider")
                    .required(false)
                    .takes_value(true)
                    .possible_values(["openweather", "weatherapi"])
                    .help("The weather provider to use"),
            )
            .arg(
                Arg::with_name("date")
                .short('d')
                .long("date")
                .required(false)
                .takes_value(true)
                .default_value("now")
                .help("The date in format YYYY-MM-DD"),
            ),
    )
    .subcommand(
        SubCommand::with_name("configure")
            .about("Configure the weather app")
            .arg(
                Arg::with_name("provider")
                    .index(1)
                    .required(true)
                    .takes_value(true)
                    .possible_values(["openweather", "weatherapi"])
                    .help("The weather provider to configure"),
            )
    )
    .get_matches()
}
