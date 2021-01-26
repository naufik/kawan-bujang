use std::env;

pub struct TelegramBotConfig {
  pub bot_token: String,
  pub chat_id: String,
}

impl TelegramBotConfig {
  pub fn load_from_env() -> Option<TelegramBotConfig> {
    let bot_token = env::var_os("TELEGRAM_BOT_TOKEN");
    let chat_id = env::var_os("TELEGRAM_GROUP_CHAT_ID");

    let config = TelegramBotConfig{
      bot_token: bot_token?.to_str()?.to_string(),
      chat_id: chat_id?.to_str()?.to_string()
    };
  
    Some(config)
  }
}
