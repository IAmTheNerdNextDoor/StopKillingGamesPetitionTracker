use reqwest::Error;
use serde::Deserialize;
use tokio::time::{sleep, Duration};

#[derive(Deserialize, Debug)]
struct EciProgress {
    signatureCount: u64,
    goal: u64,
}

#[derive(Deserialize, Debug)]
struct UkProgress {
    signature_count: u64,
}

async fn check_eci_progress() -> Result<EciProgress, Error> {
    let url = "https://eci.ec.europa.eu/045/public/api/report/progression";
    let res = reqwest::get(url).await?;
    let progress: EciProgress = res.json().await?;
    Ok(progress)
}

async fn check_uk_progress() -> Result<UkProgress, Error> {
    let url = "https://petition.parliament.uk/petitions/702074//count.json";
    let res = reqwest::get(url).await?;
    let progress: UkProgress = res.json().await?;
    Ok(progress)
}

async fn monitor_progress() -> Result<(), Error> {
    let eci_goal = 1_000_000u64;
    let uk_goal = 100_000u64;

    let mut eci_reached = false;
    let mut uk_reached = false;

    loop {
        let eci = check_eci_progress().await?;
        println!(
            "ECI Signatures: {} / Goal: {}", eci.signatureCount, eci.goal);

        let uk = check_uk_progress().await?;
        println!("UK Petition signatures: {} / Goal: {}", uk.signature_count, uk_goal);

        if !eci_reached && eci.signatureCount >= eci_goal {
            println!("ECI at 1,000,000+ signatures!");
            eci_reached = true;
        }

        if !uk_reached && uk.signature_count >= uk_goal {
            println!("UK at 100,000+ signatures!");
            uk_reached = true;
        }

        sleep(Duration::from_secs(5)).await;
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = monitor_progress().await {
        eprintln!("Oops, something went wrong: {:?}", e);
    }
}

