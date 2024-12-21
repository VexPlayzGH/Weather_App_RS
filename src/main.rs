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
struct List {
    list: Vec<WeatherForecastResponse>,
    city: City,
}

#[derive(Deserialize, Debug)]
struct City {
    name: String,
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
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct MainForecast {
    temp: f64,
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
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    // Formatting weather information into a string
    let weather_text = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C, 
        > Humidity: {:.1}%, 
        > Pressure: {:.1} hPa, 
        > Wind Speed: {:.1} mps",
        response.name,
        description,
        get_temperature_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    // Coloring the weather text based on weather conditions
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
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
        let humidity_fc = list.list[i].main.humidity;
        let pressure_fc = list.list[i].main.pressure;
        let wind_speed_fc = list.list[i].wind.speed;       
        // Formatting weather information into a string
        let weather_text_fc = format!(
            "-- {:?} --
            Weather in {}: {} {}
            > Temperature: {:.1}°C, 
            > Humidity: {:.1}%, 
            > Pressure: {:.1} hPa, 
            > Wind Speed: {:.1} mps\n",
            dt_text,
            name,
            description_fc,
            get_temperature_emoji(temperature_fc),
            temperature_fc,
            humidity_fc,
            pressure_fc,
            wind_speed_fc,
        );        

        // Coloring the weather text based on weather conditions
        let weather_text_colored = match description_fc.as_str() {
            "clear sky" => weather_text_fc.bright_yellow(),
            "few clouds" | "scattered clouds" | "broken clouds" => weather_text_fc.bright_blue(),
            "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text_fc.dimmed(),
            "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text_fc.bright_cyan(),
            _ => weather_text_fc.normal(),
        };

        // Printing the colored weather information
        println!("{}", weather_text_colored);
    }

}

// Function to get emoji based on temperature
fn get_temperature_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature < 10.0 {
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

        // Get API key
        let api_key: &str = "591ca4363aa734036342ecd0969b9466";

        let mut action_selection: String = String::new();
        println!("Do you want to see the current weather, or the hourly weather forecast?");
        std::io::stdin().read_line(&mut action_selection).expect("Input failed");

        match action_selection.trim() {
            "1" => {
                // Calling the function to fetch weather information
                match get_weather_info(&city, &country_code, api_key) {
                    Ok(response) => display_weather_info(&response),
                    Err(err) => println!("Error: {}", err),
                }
            },
            "2" => {
                match get_weather_forecast_info(&city, &country_code, api_key) {
                    Ok(response_fc) => display_weather_forecast_info(&response_fc),
                    Err(err) => println!("Error: {:?}", err),
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
