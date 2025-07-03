use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Progress {
    signatureCount: u64,
    goal: u64,
}

async fn check_progress() -> Result<(), Error> {
    let url = "https://eci.ec.europa.eu/045/public/api/report/progression";

    loop {
        let res = reqwest::get(url).await?;
        let progress: Progress = res.json().await?;

        println!(
            "Current ECI signatures: {} / Goal: {}",
            progress.signatureCount, progress.goal
        );

        if progress.signatureCount >= progress.goal {
            println!("1 MILLION SIGNATURES!!!");
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = check_progress().await {
        eprintln!("Oops, something went wrong: {:?}", e);
    }
}

