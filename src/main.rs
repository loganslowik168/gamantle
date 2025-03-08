use std::env;
use dotenv::dotenv;
use reqwest::Error;
use serde::Deserialize;
use std::io::{self, Write};

#[derive(Deserialize, Debug)]
struct Game {
    name: String,
    released: Option<String>,
    rating: f32,
    description: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GamesResponse {
    results: Vec<Game>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let api_key = env::var("RAWG_API_KEY").expect("API_KEY not found in .env file");

    loop {
        print!("Enter a game name (or type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut game_name = String::new();
        io::stdin().read_line(&mut game_name).expect("Failed to read input");
        let game_name = game_name.trim();

        if game_name.eq_ignore_ascii_case("exit") {
            break;
        }

        let url = format!(
            "https://api.rawg.io/api/games?key={}&search={}&page_size=1",
            api_key, game_name
        );

        let response: GamesResponse = match reqwest::get(&url).await?.json().await {
            Ok(res) => res,
            Err(_) => {
                println!("Failed to fetch game information. Try again.");
                continue;
            }
        };

        if let Some(game) = response.results.get(0) {
            println!(
                "\nName: {}\nReleased: {}\nRating: {}\nDescription: {}\n",
                game.name,
                game.released.clone().unwrap_or_else(|| "Unknown".to_string()),
                game.rating,
                game.description.clone().unwrap_or_else(|| "No description available.".to_string())
            );
        } else {
            println!("No game found with the name '{}'.", game_name);
        }
    }

    println!("Program terminated.");
    Ok(())
}
