use dotenv::dotenv;
use reqwest::Error;
use serde::Deserialize;
use std::env;
use std::io::{self, Write};
use std::collections::HashSet;

#[derive(Deserialize, Debug)]
struct Genre {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Tag {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Platform {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Game {
    name: String,
    released: Option<String>,
    rating: f32,
    description: Option<String>,
    genres: Vec<Genre>,
    tags: Vec<Tag>,
    platforms: Option<Vec<PlatformWrapper>>,
}

#[derive(Deserialize, Debug)]
struct PlatformWrapper {
    platform: Platform,
}

#[derive(Deserialize, Debug)]
struct GamesResponse {
    results: Vec<Game>,
}

// Function to calculate Jaccard Index (similarity) between two sets of tags
fn calculate_jaccard_index(tags1: &Vec<String>, tags2: &Vec<String>) -> f64 {
    let set1: HashSet<_> = tags1.iter().cloned().collect();
    let set2: HashSet<_> = tags2.iter().cloned().collect();

    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    let union: HashSet<_> = set1.union(&set2).collect();

    intersection.len() as f64 / union.len() as f64
}

// Enhanced similarity calculation
fn enhanced_similarity(tags1: &Vec<String>, tags2: &Vec<String>, genres1: &Vec<String>, genres2: &Vec<String>, platforms1: &Vec<String>, platforms2: &Vec<String>) -> f64 {
    let tag_similarity = calculate_jaccard_index(tags1, tags2);
    let genre_similarity = calculate_jaccard_index(genres1, genres2);
    let platform_similarity = calculate_jaccard_index(platforms1, platforms2);

    // Assign weights to the factors based on importance
    let total_similarity = (tag_similarity * 0.6 + genre_similarity * 0.35 + platform_similarity * 0.05);
    total_similarity * 100.0 // Scale to 1-100
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let api_key = env::var("RAWG_API_KEY").expect("API_KEY not found in .env file");

    loop {
        print!("Enter a game name (or type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut game_name1 = String::new();
        io::stdin().read_line(&mut game_name1).expect("Failed to read input");
        let game_name1 = game_name1.trim();

        if game_name1.eq_ignore_ascii_case("exit") {
            break;
        }

        let url1 = format!(
            "https://api.rawg.io/api/games?key={}&search={}&page_size=1",
            api_key, game_name1
        );

        let response1: GamesResponse = match reqwest::get(&url1).await?.json().await {
            Ok(res) => res,
            Err(_) => {
                println!("Failed to fetch game information. Try again.");
                continue;
            }
        };

        if let Some(game1) = response1.results.get(0) {
            println!("\nName: {}\nReleased: {}\nRating: {}\nDescription: {}",
                game1.name,
                game1.released.clone().unwrap_or_else(|| "Unknown".to_string()),
                game1.rating,
                game1.description.clone().unwrap_or_else(|| "No description available.".to_string())
            );

            let genres1: Vec<String> = game1.genres.iter().map(|g| g.name.clone()).collect();
            let tags1: Vec<String> = game1.tags.iter().map(|t| t.name.clone()).collect();
            let platforms1: Vec<String> = game1.platforms
                .as_ref()
                .map_or(vec![], |platforms| platforms.iter().map(|p| p.platform.name.clone()).collect());
            
            println!("Genres: {}", genres1.join(", "));
            println!("Tags: {}", tags1.join(", "));
            println!("Platforms: {}", platforms1.join(", "));

            // Prompt for the second game
            print!("\nEnter the second game name (or type 'exit' to quit): ");
            io::stdout().flush().unwrap();

            let mut game_name2 = String::new();
            io::stdin().read_line(&mut game_name2).expect("Failed to read input");
            let game_name2 = game_name2.trim();

            if game_name2.eq_ignore_ascii_case("exit") {
                break;
            }

            let url2 = format!(
                "https://api.rawg.io/api/games?key={}&search={}&page_size=1",
                api_key, game_name2
            );

            let response2: GamesResponse = match reqwest::get(&url2).await?.json().await {
                Ok(res) => res,
                Err(_) => {
                    println!("Failed to fetch game information. Try again.");
                    continue;
                }
            };

            if let Some(game2) = response2.results.get(0) {
                println!("\nName: {}\nReleased: {}\nRating: {}\nDescription: {}",
                    game2.name,
                    game2.released.clone().unwrap_or_else(|| "Unknown".to_string()),
                    game2.rating,
                    game2.description.clone().unwrap_or_else(|| "No description available.".to_string())
                );

                let genres2: Vec<String> = game2.genres.iter().map(|g| g.name.clone()).collect();
                let tags2: Vec<String> = game2.tags.iter().map(|t| t.name.clone()).collect();
                let platforms2: Vec<String> = game2.platforms
                    .as_ref()
                    .map_or(vec![], |platforms| platforms.iter().map(|p| p.platform.name.clone()).collect());

                println!("Genres: {}", genres2.join(", "));
                println!("Tags: {}", tags2.join(", "));
                println!("Platforms: {}", platforms2.join(", "));

                // Calculate the enhanced similarity
                let similarity = enhanced_similarity(&tags1, &tags2, &genres1, &genres2, &platforms1, &platforms2);
                println!("\nSimilarity between '{}' and '{}' based on tags, genres, and platforms: {:.2}", game1.name, game2.name, similarity);
            } else {
                println!("No game found with the name '{}'.", game_name2);
            }
        } else {
            println!("No game found with the name '{}'.", game_name1);
        }
    }

    println!("Program terminated.");
    Ok(())
}
