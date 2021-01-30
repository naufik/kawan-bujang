use std::env;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;

pub fn bot_op(bot_token: &String, chat_id: &String) {
  let weather_api_appid = env::var_os("WEATHER_APP_ID");

  let weather_api_appid = match weather_api_appid {
    Some(ref key) => key.to_str().unwrap(),
    None => return
  };

  let weather_api_url = format!("https://api.openweathermap.org/data/2.5/onecall?units=metric&lang=id&lat=-6.2088&lon=106.8456&exclude=minutely,alerts&appid={}", weather_api_appid);

  let weather_client = Client::new();
  let weather_data = weather_client.get(&weather_api_url).send().unwrap();
  let weather_contents: Value = weather_data.json().unwrap();
  let weather_description: Value =
    weather_contents["daily"][0]["weather"][0]["description"].clone();
  let weather_name: Result<String, serde_json::Error> = serde_json::from_value(weather_description);

  let message_text = format!(
    r#"Selamat Pagi! Prakiraan cuaca untuk kota Jakarta hari ini adalah {}.\n\nSuhu cuaca hari ini:\nPagi: {}°C\nSiang: {}°C\nSore: {}°C\nMalam: {}°C\n\nCurah hujan: {}mm\nKelembaban: {}%"#,
    weather_name.unwrap(),
    weather_contents["daily"][0]["temp"]["morn"],
    weather_contents["daily"][0]["temp"]["day"],
    weather_contents["daily"][0]["temp"]["eve"],
    weather_contents["daily"][0]["temp"]["night"],
    weather_contents["daily"][0]["rain"],
    weather_contents["daily"][0]["humidity"]
  );

  let send_url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

  // TODO: use json instead of this voodoo string bullshit
  let body = format!(
    r#"
    {{
        "chat_id": {},
        "text": "{}"
    }}
    "#,
    chat_id, message_text
  );
  let telegram_send_client = Client::new();

  let result = telegram_send_client
    .post(&send_url)
    .body(body)
    .header(CONTENT_TYPE, "application/json")
    .send()
    .unwrap();
  let contents = result.text();

  if contents.is_ok() {
    println!("[bujang] ran intent: scheduled-weather-update.");
    println!("{}", contents.unwrap());
  }
}
