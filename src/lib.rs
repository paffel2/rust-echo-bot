use std::str::FromStr;

use serde::de::DeserializeOwned;
use serde_json::ser::to_string;
mod structures;
pub use structures::*;
use ureq;

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

pub fn send_keyboard(token: &str, chat_id: u64) -> u64 {
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
    match send_help_message {
        Ok(resp) => {
            let returned_message: Result<TgResponse<TgMessage>, std::io::Error> = resp.into_json();
            match returned_message {
                Ok(message) => return message.result.unwrap().message_id,
                _ => {
                    println!("message not parsed");
                    0
                }
            }
        }
        _ => {
            println!("message not parsed");
            0
        }
    }

    /*    println!("keyboard send")
    } else {
        println!("keyboard not send")
    }-}*/
}

pub fn unwrap_repeats(status: Status) -> u8 {
    match status {
        Status::CurrentNumber(a) => a,
        _ => 0,
    }
}

pub fn unwrap_status(status: Status) -> u64 {
    match status {
        Status::CurrentNumber(_) => 0,
        Status::WaitNumber(a) => a,
    }
}

pub fn delete_message(token: &str, chat_id: u64, msg_id: u64) -> () {
    let requset_string: &str = &(format!(
        "{0}{1}{2}",
        "https://api.telegram.org/bot", token, "/deleteMessage"
    ));
    let chat_id_str: &str = &(format!("{}", chat_id));
    let msg_id_str: &str = &(format!("{}", msg_id));
    let delete_message = ureq::get(requset_string)
        .send_form(&[("chat_id", chat_id_str), ("message_id", msg_id_str)]);
    if delete_message.is_ok() {
        println!("service message deleted")
    } else {
        println!("service message not deleted")
    }
}
