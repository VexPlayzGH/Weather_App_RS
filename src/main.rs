use serde::Deserialize; // For JSON serialization
use colored::*; // Importing colored crate for text coloring

// Struct to deserialize the JSON response from OpenWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to represent main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct to represent wind information
#[derive(Deserialize, Debug)]
struct Wind {
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

// Function to get emoji based on temperature
fn get_temperature_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
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

        // Calling the function to fetch weather information
        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => display_weather_info(&response),
            Err(err) => println!("Error: {}", err),
        }

        println!("{}", "Do you want to search for weather in another city? (yes/no):".bright_green()); // Prompting user to continue or exit
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input"); // Reading user input for continuation
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break; // Exiting the loop if user doesn't want to continue
        }
    }
}
