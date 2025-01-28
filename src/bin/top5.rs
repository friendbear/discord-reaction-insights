use serenity::{
    async_trait,
    model::{
        channel::{Message, ReactionType},
        gateway::Ready,
        id::ChannelId,
    },
    prelude::*,
};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use dotenv::dotenv;
use std::{collections::HashMap, env};

struct Handler {
    target_channel: ChannelId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return; // ボットのメッセージは無視
        }

        if msg.content.starts_with("!top5") {
            let args: Vec<&str> = msg.content.split_whitespace().collect();

            // 日付範囲の取得
            if args.len() < 3 {
                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        "利用方法: !top5 <start_date> <end_date>\n例: !top5 2025-01-01 2025-02-02",
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
                return;
            }

            let naive_start_date = NaiveDateTime::parse_from_str(
                &format!("{} 00:00:00", args[1]),
                "%Y-%m-%d %H:%M:%S",
            );
            let naive_end_date = NaiveDateTime::parse_from_str(
                &format!("{} 00:00:00", args[2]),
                "%Y-%m-%d %H:%M:%S",
            );

            let start_date = match naive_start_date {
                Ok(dt) => TimeZone::from_utc_datetime(&Utc, &dt),
                Err(why) => {
                    if let Err(why) = msg
                        .channel_id
                        .say(&ctx.http, "日付のフォーマットが正しくありません。{why}")
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    }
                    return;
                }
            };

            // Use the start_date variable
            println!("Start date: {:?}", start_date);
        }
    }
}

fn main() {
    dotenv().ok();

    let token = env::var(
        "
        DISCORD_TOKEN
        ",
    )
    .expect("Expected a token in the environment");

    todo!();
}
