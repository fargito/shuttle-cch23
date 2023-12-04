use axum::Json;

/// Day 4
pub async fn reindeer_cheer(Json(body): Json<Vec<ReindeerSimpleInfo>>) -> String {
    body.iter()
        .map(|reindeer| reindeer.strength)
        .sum::<f64>()
        .to_string()
}

/// Day 4
pub async fn reindeer_contest(Json(body): Json<Vec<ReindeerInfo>>) -> String {
    let (mut fastest, mut tallest, mut magician, mut consumer) = (0_f64, 0_f64, 0_f64, 0_f64);

    let mut response = ReindeerContestResponse {
        fastest: "".to_owned(),
        tallest: "".to_owned(),
        magician: "".to_owned(),
        consumer: "".to_owned(),
    };

    for reindeer in body {
        if reindeer.speed > fastest {
            fastest = reindeer.speed;

            response.fastest = format!(
                "Speeding past the finish line with a strength of {} is {}",
                reindeer.strength, reindeer.name
            );
        }

        if reindeer.height > tallest {
            tallest = reindeer.height;

            response.tallest = format!(
                "{} is standing tall with his {} cm wide antlers",
                reindeer.name, reindeer.antler_width
            );
        }

        if reindeer.snow_magic_power > magician {
            magician = reindeer.snow_magic_power;

            response.magician = format!(
                "{} could blast you away with a snow magic power of {}",
                reindeer.name, reindeer.snow_magic_power
            );
        }

        if reindeer.candies_eaten_yesterday > consumer {
            consumer = reindeer.candies_eaten_yesterday;

            response.consumer = format!(
                "{} ate lots of candies, but also some {}",
                reindeer.name, reindeer.favorite_food
            );
        }
    }

    serde_json::to_string(&response).unwrap()
}

#[derive(serde::Deserialize)]
pub struct ReindeerSimpleInfo {
    strength: f64,
}

#[derive(serde::Deserialize)]
pub struct ReindeerInfo {
    name: String,
    strength: f64,
    speed: f64,
    height: f64,
    antler_width: f64,
    snow_magic_power: f64,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: f64,
}

#[derive(serde::Serialize)]
pub struct ReindeerContestResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}
