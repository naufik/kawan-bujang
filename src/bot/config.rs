use std::env;

pub struct TelegramBotConfig {
  pub bot_token: String,
  pub chat_id: String,
}

impl TelegramBotConfig {
  pub fn load_from_env() -> TelegramBotConfig {
    let bot_token = env::var_os("TELEGRAM_BOT_TOKEN");
    let chat_id = env::var_os("TELEGRAM_GROUP_CHAT_ID");

    if bot_token.is_none() || chat_id.is_none() {
      println!(
        "Incomplete parameters. Check that both TELEGRAM_BOT_TOKEN and TELEGRAM_GROUP_CHAT_ID is set."
      );
      std::process::exit(1);
    };

    TelegramBotConfig{
      bot_token: bot_token.unwrap().to_str().unwrap().to_string(),
      chat_id: chat_id.unwrap().to_str().unwrap().to_string()
    }
  }
}
