extern crate chrono;
extern crate chrono_tz;

mod weather_bot;
mod bot;

use clokwerk::{Scheduler, TimeUnits};
use std::thread;
use std::time::Duration;

use crate::bot::*;

fn main() {
  println!("[bujang] Starting operation, loading configuration");
  let config: Option<TelegramBotConfig> = TelegramBotConfig::load_from_env();

  let config = config.unwrap_or_else(|| {
    println!("Check if TELEGRAM_BOT_TOKEN and TELEGRAM_GROUP_CHAT_ID are both defined.");
    std::process::exit(1);
  });

  println!("[bot] Configuration loaded. Starting initial loop.");
  init_bot(config);
}

// TODO: move this somewhere else!!!
fn init_bot(config: TelegramBotConfig) {
  let mut scheduler = Scheduler::with_tz(chrono_tz::Asia::Jakarta);

  let bot_token = config.bot_token.clone();
  let chat_id = config.chat_id.clone();

  scheduler
    .every(1.day())
    .at("8:00 am")
    .run(move || weather_bot::bot_op(&bot_token, &chat_id));

  loop {
    scheduler.run_pending();
    thread::sleep(Duration::from_millis(20));
  }
}