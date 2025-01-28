use chrono::{Datelike, Duration, Utc, TimeZone, Timelike};
use serenity::all::Event;
use serenity::model::channel::Message;
use serenity::{async_trait, model::gateway::Ready, prelude::*};
use serenity::Result;
use tokio_cron_scheduler::{Job, JobScheduler};
use std::collections::HashMap;
use std::sync::Arc;

struct Handler;

impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILDS;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .intents(intents)
        .await
        .expect("Err creating client");

    let scheduler = JobScheduler::new().await.unwrap();

    scheduler
        .add(
            Job::new_async("0 8 1 * * *", |_uuid, _lock| {
                Box::pin(async move {
                    println!("Running scheduled job");

                    // 前月の開始と終了日時を計算
                    let now = Utc::now();
                    let start_of_last_month = Utc
                        .with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0)
                        .unwrap()
                        .checked_sub_months(chrono::Months::new(1))
                        .unwrap()
                        .with_day(1)
                        .expect("ひとつ前の月の開始日時を計算する際にエラーが発生しました。");

                    let end_of_last_month = start_of_last_month
                        .checked_add_months(chrono::Months::new(1))
                        .unwrap()
                        .checked_sub_signed(Duration::days(1))
                        .expect("ひとつ前の月の終了日時を計算する際にエラーが発生しました。")
                        .with_hour(23)
                        .and_then(|dt| dt.with_minute(59))
                        .and_then(|dt| dt.with_second(59))
                        .expect("時刻を設定する際にエラーが発生しました。");

                    println!("先月の開始日時: {}", start_of_last_month);
                    println!("先月の終了日時: {}", end_of_last_month);
                })
            })
            .unwrap(),
        )
        .await
        .unwrap();

    scheduler
        .start()
        .await
        .expect("Error starting scheduler");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}

#[test]
fn test_start_of_last_month() {

    let now = Utc::now();
    let start_of_last_month = Utc
        .with_ymd_and_hms(now.year(), now.month() , 1, 0, 0, 0)
        .unwrap()
        .checked_sub_months(chrono::Months::new(1))
        .unwrap()
        .with_day(1).expect("ひとつ前の月の開始日時を計算する際にエラーが発生しました。");

    eprintln!("{start_of_last_month}");
}

#[test]
fn test_end_of_last_month() {

    let now = Utc::now();
        // 先月の初日を計算
    let start_of_last_month = Utc
        .with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0)
        .unwrap()
        .checked_sub_months(chrono::Months::new(1))
        .unwrap()
        .with_day(1)
        .expect("ひとつ前の月の開始日時を計算する際にエラーが発生しました。");

    let end_of_last_month = start_of_last_month
        .checked_add_months(chrono::Months::new(1))
        .unwrap()
        .checked_sub_signed(Duration::days(1))
        .expect("ひとつ前の月の終了日時を計算する際にエラーが発生しました。")
        .with_hour(23)
        .and_then(|dt| dt.with_minute(59))
        .and_then(|dt| dt.with_second(59))
        .expect("時刻を設定する際にエラーが発生しました。");    // 次の月の初日を計算してから1日引く

    eprintln!("先月の初日: {}", start_of_last_month);
    eprintln!("先月の最終日: {}", end_of_last_month);

}