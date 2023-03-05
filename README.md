# Weather-cli

The app allows getting weather from a few sources

![Alt Text](https://media.giphy.com/media/9t3XdrwxLrkm3HKvWE/giphy.gif)


# app

It could be used as an example. Need refactoring.


# install

Native (Linux)

1. git clone https://github.com/andrewozarko/weather-cli
2. cd weather-cli
3. cargo install
4. cargo build --release
5. cp ./target/release/weather-cli ./weather-cli

# commands

1. ```./weather-cli help``` - show all commands
2. ```./weather-cli configure <provider>``` - impelmented providers: openweather, weatherapi
3. ```./weather-cli get <city_name> --date YYYY-MM-DD --provider openweather``` - an example Lviv, Kiev... Option date is not required, only weatherapi support option date. Option provider isn't required, default provider "weatherapi"

# details

1. How to setup to grpc service ?

```
docker run --rm -p 9000:9000 \
  -e OPEN_WEATHER_MAP_API_KEY="56b55e8977a22969fd9da0023e8bcdc2" \
  -e WEATHER_UNDERGROUND_API_KEY="6256985d57ff42bf96985d57ff72bf1f" \
  --name weather_service caiofilipini/grpc-weather:master
```
2. configure openweather provider: http://localhost:9000 (./weather-cli configure openweather)

3. default path to configs /home/your_username/.weather-cli/