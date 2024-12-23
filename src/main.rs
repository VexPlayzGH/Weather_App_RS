use serde::Deserialize; // For JSON serialization
use colored::*; // Importing colored crate for text coloring
use std::process::Command;

// Struct to deserialize the JSON response from OpenWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct AirResponse {
    data: AirData,
}

#[derive(Deserialize, Debug)]
struct AirData {
    city: AirCity,
    iaqi: IAqi,
}

#[derive(Deserialize, Debug)]
struct List {
    list: Vec<WeatherForecastResponse>,
    city: City,
}

#[derive(Deserialize, Debug)]
struct City {
    name: String,
}

#[derive(Deserialize, Debug)]
struct AirCity {
    name: String,
}

#[derive(Deserialize, Debug)]
struct IAqi {
    co: Val,
    h: Val,
    no2: Val,
    o3: Val,
    p: Val,
    pm10: Val,
    pm25: Val,
    so2: Val,
    t: Val,
    w: Val,
}

#[derive(Deserialize, Debug)]
struct Val {
    v: f64,
}

#[derive(Deserialize, Debug)]
struct WeatherForecastResponse {    
    weather: Vec<WeatherForecast>,    
    main: MainForecast,
    wind: WindForecast,
    dt_txt: String,    
}

// Struct to represent weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct WeatherForecast {
    description: String,
}


// Struct to represent main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct MainForecast {
    temp: f64,
    feels_like: f64,    
    humidity: f64,
    pressure: f64,
}

// Struct to represent wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

#[derive(Deserialize, Debug)]
struct WindForecast {
    speed: f64,
}

// Function to get weather information from OpenWeatherMap API
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    // Constructing the URL for API request
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    // Sending a blocking GET request to the API endpoint
    let response = reqwest::blocking::get(&url)?;
    // Parsing the JSON response into WeatherResponse struct
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json) // Returning the deserialized response
}

// Function to display weather information
fn display_weather_info(response: &WeatherResponse) {
    // Extracting weather information from the response
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let feels_like = response.main.feels_like;
    let temp_min = response.main.temp_min;
    let temp_max = response.main.temp_max;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    // Formatting weather information into a string
    let weather_text = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C,
        > Feels like: {:.1}°C,
        > Today's maximum temperature: {:.1}°C,
        > Today's minimum temperature: {:.1}°C, 
        > Humidity: {:.1}%, 
        > Pressure: {:.1} hPa, 
        > Wind Speed: {:.1} mps",
        response.name,
        description,
        get_temperature_emoji(temperature, description),
        temperature,
        feels_like,
        temp_max,
        temp_min,
        humidity,
        pressure,
        wind_speed,
    );

    // Coloring the weather text based on weather conditions
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "light rain" | "moderate rain" | "heavy intensity rain" | "thunderstorm" | "snow" | "light snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    // Printing the colored weather information
    println!("{}", weather_text_colored);
}

fn get_weather_forecast_info(city: &str, country_code: &str, api_key: &str) -> Result<List, reqwest::Error> {
    // Constructing the URL for API request
    let url_fc = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={},{}&appid={}",
        city, country_code, api_key
    );

    // Sending a blocking GET request to the API endpoint    
    let response_fc_list = reqwest::blocking::get(&url_fc)?;    
    // Parsing the JSON response into WeatherResponse struct
    let response_fc_json = response_fc_list.json::<List>()?;
    // Returning the deserialized response
    Ok(response_fc_json)
}

fn display_weather_forecast_info(list: &List) {
    // Extracting weather information from the response
    for i in 0..=39 {
        let dt_text = &list.list[i].dt_txt;
        let name = &list.city.name;
        let description_fc = &list.list[i].weather[0].description;
        let temperature_fc = list.list[i].main.temp - 273.15;
        let feels_like_fc = list.list[i].main.feels_like - 273.15;
        let humidity_fc = list.list[i].main.humidity;
        let pressure_fc = list.list[i].main.pressure;
        let wind_speed_fc = list.list[i].wind.speed;       
        // Formatting weather information into a string
        let weather_text_fc = format!(
            "-- {:?} --
            Weather in {}: {} {}
            > Temperature: {:.1}°C,
            > Feels like: {:.1}°C, 
            > Humidity: {:.1}%, 
            > Pressure: {:.1} hPa, 
            > Wind Speed: {:.1} mps\n",
            dt_text,
            name,
            description_fc,
            get_temperature_emoji(temperature_fc, description_fc),
            temperature_fc,
            feels_like_fc,
            humidity_fc,
            pressure_fc,
            wind_speed_fc,
        );        

        // Coloring the weather text based on weather conditions
        let weather_text_colored = match description_fc.as_str() {
            "clear sky" => weather_text_fc.bright_yellow(),
            "few clouds" | "scattered clouds" | "broken clouds" => weather_text_fc.bright_blue(),
            "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text_fc.dimmed(),
            "shower rain" | "rain" | "light rain" | "moderate rain" | "heavy intensity rain" | "thunderstorm" | "snow" | "light snow" => weather_text_fc.bright_cyan(),
            _ => weather_text_fc.normal(),
        };

        // Printing the colored weather information
        println!("{}", weather_text_colored);
    }

}

// Function to get emoji based on temperature
fn get_temperature_emoji(temperature: f64, description_fc: &String) -> &'static str {
    if description_fc.as_str() == "shower rain" || description_fc.as_str() == "rain" || description_fc.as_str() == "light rain" || description_fc.as_str() == "moderate rain" || description_fc.as_str() == "heavy intensity rain" {
        "🌧️"
    }
    else if description_fc.as_str() == "thunderstorm" {
        "⛈️"
    }
    else if description_fc.as_str() == "snow" || description_fc.as_str() == "light snow" {
        "❄️"
    }
    else {
        if temperature < 10.0 {
            "☁️"
        } else if temperature < 20.0 {
            "⛅"
        } else if temperature < 30.0 {
            "🌤️"
        } else if temperature < 37.0 {
            "☀️"
        } else {
            "🔥"
        }    
    }    
}

fn get_air_info(city: &str, api_key_air: &str) -> Result<AirResponse, reqwest::Error> {
    // Constructing the URL for API request
    let url_air = format!(
        "https://api.waqi.info/feed/{}/?token={}",
        city, api_key_air
    );

    // Sending a blocking GET request to the API endpoint
    let response_air = reqwest::blocking::get(&url_air)?;
    // Parsing the JSON response into WeatherResponse struct
    let response_air_json = response_air.json::<AirResponse>()?;
    Ok(response_air_json) // Returning the deserialized response
}

fn display_air_info(response_air: &AirResponse) {
    // Extracting air pollution information from the response
    let name = &response_air.data.city.name;
    let co = response_air.data.iaqi.co.v;
    let h = response_air.data.iaqi.h.v;
    let no2 = response_air.data.iaqi.no2.v;
    let o3 = response_air.data.iaqi.o3.v;
    let p = response_air.data.iaqi.p.v;
    let pm10 = response_air.data.iaqi.pm10.v;
    let pm25 = response_air.data.iaqi.pm25.v;
    let so2 = response_air.data.iaqi.so2.v;
    let t = response_air.data.iaqi.t.v;
    let w = response_air.data.iaqi.w.v;

    let air_text = format!(
        "Air Pollution in {}:
        > CO₂: {:.1}μg/m³,
        > H₂: {:.1}μg/m³,
        > NO₂: {:.1}μg/m³,
        > O₃: {:.1}μg/m³,
        > P₄: {:.1}μg/m³,
        > PM1: {:.1}µm,
        > PM2.5: {:.1}µm,
        > SO₂: {:.1}μg/m³,
        > T: {:.1}°K,
        > W: {:.1}μg/m³,",
        name,
        co,
        h,
        no2,
        o3,
        p,
        pm10,
        pm25,
        so2,
        t,
        w,
    );
    let air_text_colored = air_text.bright_yellow();
    // Printing the colored air pollution information
    println!("{}", air_text_colored);
}
fn main() {    
    println!("{}", "Welcome to Weather Station!".bright_yellow());
    loop {
        println!("Please enter the name of the city:");
        let mut city: String = String::new();
        std::io::stdin().read_line(&mut city).expect("Input failed");
        let city: &str = city.trim();

        println!("Please enter the name of the country code (e.g. US for United States):");
        let mut country_code: String = String::new();
        std::io::stdin().read_line(&mut country_code).expect("Input failed");
        let country_code: &str = country_code.trim();

        // Get API keys
        let api_key: &str = "591ca4363aa734036342ecd0969b9466";
        let api_key_air: &str = "2a39e0b3437b635b380c94272b99ff5dd00d2b5d";

        let mut action_selection: String = String::new();
        println!("Do you want to see the current weather, the 3-hour weather forecast,\nor the air pollution levels? (1/2/3)");
        std::io::stdin().read_line(&mut action_selection).expect("Input failed");

        match action_selection.trim() {
            "1" => {
                // Calling the function to fetch weather information
                match get_weather_info(&city, &country_code, api_key) {
                    Ok(response) => display_weather_info(&response),
                    Err(err) => eprintln!("Error: {}", err),
                }
            },
            "2" => {
                match get_weather_forecast_info(&city, &country_code, api_key) {
                    Ok(response_fc) => display_weather_forecast_info(&response_fc),
                    Err(err) => eprintln!("Error: {}", err),
                }
            },
            "3" => {
                match get_air_info(&city, api_key_air) {
                    Ok(response_air) => display_air_info(&response_air),
                    Err(err) => eprintln!("Error: {}", err),
                }
            },
            _ => println!("Invalid input"),
        }

        println!("{}", "Do you want to search for weather in another city? (yes/no):".bright_green()); // Prompting user to continue or exit
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input"); // Reading user input for continuation
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break;
        }
    }
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
