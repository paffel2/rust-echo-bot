use std::str::FromStr;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::ser::to_string;
use ureq;

#[derive(Deserialize)]
pub struct TgGetMeResult {
    pub id: u64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: String,
}

/*impl fmt::Display for TgGetMeResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {0},is_bot: {1}; first_name: {2}; username:{3};\n",
            self.id, self.is_bot, self.first_name, self.username
        )
    }
}*/

#[derive(Deserialize)]
pub struct TgResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub error_code: Option<u64>,
    pub description: Option<String>,
}

/*impl<T: fmt::Display> fmt::Display for TgResponse<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ok: {0},\nresult: {1}", self.ok, self.result)
    }
}*/

#[derive(Deserialize)]
pub struct TgUser {
    pub id: u64,
}

/*impl fmt::Display for TgUser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "user_id: {0}", self.id)
    }
}*/

#[derive(Deserialize)]
pub struct TgMessage {
    pub message_id: u64,
    pub from: Option<TgUser>,
    pub text: Option<String>,
}

/*impl fmt::Display for TgMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.from {
            Some(user) => write!(
                f,
                "message_id: {0};message_from: {1}",
                self.message_id, user
            ),
            _ => write!(f, "message_id: {0};", self.message_id),
        }
    }
}*/

#[derive(Deserialize)]
pub struct TgChat {
    pub id: u64,
}

/*impl fmt::Display for TgChat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "chat_id: {0}", self.id)
    }
}
*/

#[derive(Deserialize)]
pub struct TgCallbackData {
    pub data: String,
    pub from: TgUser//,
}

#[derive(Deserialize)]
pub struct TgUpdate {
    pub update_id: u64,
    pub message: Option<TgMessage>,
    pub callback_query: Option<TgCallbackData>, //,
}

/*impl fmt::Display for TgUpdate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.message {
            Some(message) => write!(f, "update_Id: {0}; message: {1};", self.update_id, message),
            _ => write!(f, "update_id: {0};", self.update_id),
        }
    }
}*/
pub fn get_me(token: &str) -> Result<TgResponse<TgGetMeResult>, String> {
    let requset_string: &str =
        &(format!("{0}{1}{2}", "https://api.telegram.org/bot", token, "/getMe"));
    let message = ureq::get(requset_string).call();
    match message {
        Ok(response) => match response.into_json() {
            Ok(someresult) => Ok(someresult),
            _ => Err(String::from("bad parse json")),
        },
        Err(something) => parse_error(something),
    }
}

pub fn parse_error<T: DeserializeOwned>(err: ureq::Error) -> Result<TgResponse<T>, String> {
    let err_response = err.into_response();
    match err_response {
        Some(err_res) => {
            let something: Result<TgResponse<T>, std::io::Error> = err_res.into_json();
            match something {
                Ok(some_result) => {
                    if some_result.ok {
                        Err(String::from("unexpected response"))
                    } else {
                        Err(some_result
                            .description
                            .unwrap_or(String::from("unexpected response")))
                    }
                }
                _ => Err(String::from("unexpected response")),
            }
        }
        None => Err(String::from("unexpected response")),
    }
}

pub fn get_updates(token: &str, update_id: &u64) -> Result<TgResponse<Vec<TgUpdate>>, ureq::Error> {
    let requset_string: &str = &(format!(
        "{0}{1}{2}",
        "https://api.telegram.org/bot", token, "/getUpdates"
    ));
    let update_id: &str = &format!("{}", update_id);
    let messages = ureq::get(requset_string)
        .send_form(&[("offset", update_id), ("timeout", "10")])?
        .into_json()?;
    Ok(messages)
}

pub fn send_echo(token: &str, message_id: u64, chat_id: u64) -> () {
    let requset_string: &str = &(format!(
        "{0}{1}{2}",
        "https://api.telegram.org/bot", token, "/copyMessage"
    ));
    let message_id_str: &str = &(format!("{}", message_id));
    let chat_id_str: &str = &(format!("{}", chat_id));
    let send_copy = ureq::get(requset_string).send_form(&[
        ("chat_id", chat_id_str),
        ("from_chat_id", chat_id_str),
        ("message_id", message_id_str),
    ]);
    if send_copy.is_ok() {
        println!("message send")
    } else {
        println!("message not send")
    }
}



pub enum MessageType {
    Help,
    Repeat,
    Simple,
}

pub fn to_message_type(text: String) -> MessageType {
    if text == String::from("/help") {
        MessageType::Help
    } else if text == String::from("/repeat") {
        MessageType::Repeat
    } else {
        MessageType::Simple
    }
}

pub fn send_help(token: &str, chat_id: u64, text: &str) -> () {
    let requset_string: &str = &(format!(
        "{0}{1}{2}",
        "https://api.telegram.org/bot", token, "/sendMessage"
    ));
    let chat_id_str: &str = &(format!("{}", chat_id));
    let send_help_message =
        ureq::get(requset_string).send_form(&[("chat_id", chat_id_str), ("text", text)]);
    if send_help_message.is_ok() {
        println!("help message send")
    } else {
        println!("help message not send")
    }
}


//"{\"inline_keyboard\":[[{\"text\":\"1\",\"callback_data\":\"1\"}],[{\"text\":\"2\",\"callback_data\":\"2\"}],[{\"text\":\"3\",\"callback_data\":\"3\"}],[{\"text\":\"4\",\"callback_data\":\"4\"}],[{\"text\":\"5\",\"callback_data\":\"5\"}]]}"
#[derive(Deserialize, Serialize)]
struct Button {
    text: String,
    callback_data: String,
}
#[derive(Deserialize, Serialize)]
struct Keyboard {
    inline_keyboard: Vec<Vec<Button>>,
}

fn get_keyboard() -> Keyboard {
    let b1 = String::from_str("1").unwrap();
    let b2 = String::from_str("2").unwrap();
    let b3 = String::from_str("3").unwrap();
    let b4 = String::from_str("4").unwrap();
    let b5 = String::from_str("5").unwrap();

    let bn1: Vec<Button> = vec![Button {
        text: b1.clone(),
        callback_data: b1.clone(),
    }];
    let bn2: Vec<Button> = vec![Button {
        text: b2.clone(),
        callback_data: b2.clone(),
    }];
    let bn3: Vec<Button> = vec![Button {
        text: b3.clone(),
        callback_data: b3.clone(),
    }];
    let bn4: Vec<Button> = vec![Button {
        text: b4.clone(),
        callback_data: b4.clone(),
    }];
    let bn5: Vec<Button> = vec![Button {
        text: b5.clone(),
        callback_data: b5.clone(),
    }];

    Keyboard {
        inline_keyboard: vec![bn1, bn2, bn3, bn4, bn5],
    }
}

pub fn send_keyboard(token: &str, chat_id: u64) -> () {
    let requset_string: &str = &(format!(
        "{0}{1}{2}",
        "https://api.telegram.org/bot", token, "/sendMessage"
    ));
    let chat_id_str: &str = &(format!("{}", chat_id));
    let kb = get_keyboard();
    let kboard = to_string(&kb).unwrap();
    let send_help_message = ureq::get(requset_string).send_form(&[
        ("chat_id", chat_id_str),
        ("text", "choose number"),
        ("reply_markup", &kboard),
        ("one_time_keyboard", "true"),
    ]);
    if send_help_message.is_ok() {
        println!("keyboard send")
    } else {
        println!("keyboard not send")
    }
}
