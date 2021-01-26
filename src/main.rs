extern crate chrono;
extern crate chrono_tz;

mod weather_bot;
mod bot;

use clokwerk::{Scheduler, TimeUnits};
use std::thread;
use std::time::Duration;

pub use crate::bot::config::*;

fn main() {
  let config: Option<TelegramBotConfig> = TelegramBotConfig::load_from_env();
  
  let config = config.expect("Check if both TELEGRAM_BOT_TOKEN and TELEGRAM_CHAT_ID exists.");

  init_bot(
    config.bot_token, config.chat_id
  )
}

// TODO: move this somewhere else!!!
fn init_bot(_bot_token: String, _chat_id: String) {
  let mut scheduler = Scheduler::with_tz(chrono_tz::Asia::Jakarta);

  let bot_token = _bot_token.clone();
  let chat_id = _chat_id.clone();

  scheduler
    .every(1.day())
    .at("8:00 am")
    .run(move || weather_bot::bot_op(&bot_token, &chat_id));

  loop {
    scheduler.run_pending();
    thread::sleep(Duration::from_millis(20));
  }
}