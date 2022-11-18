use ureq;
use serde::{Deserialize};
use std::fmt;


#[derive(Deserialize)]
pub struct TgGetMeResult {
    id: u64,
    is_bot: bool,
    first_name: String,
    username: String
}

impl fmt::Display for TgGetMeResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {0},is_bot: {1}; first_name: {2}; username:{3};\n", self.id, self.is_bot, self.first_name, self.username)
            }
        }

#[derive(Deserialize)]
pub struct TgResponse<T> {
    ok: bool,
    pub result: T
}

impl<T: fmt::Display> fmt::Display for TgResponse<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ok: {0},\nresult: {1}", self.ok, self.result)
            }
        }

#[derive(Deserialize)]
pub struct TgUser{
    pub id: u64
    }

impl fmt::Display for TgUser {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "user_id: {0}", self.id)
                }
            }

#[derive(Deserialize)]
pub struct TgMessage{
    pub message_id: u64,
    pub from: Option<TgUser>
        }


impl fmt::Display for TgMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match &self.from {
                Some(user) => write!(f, "message_id: {0};message_from: {1}", self.message_id, user),
                _ => write!(f, "message_id: {0};", self.message_id)
                   }
                }}
    
#[derive(Deserialize)]
pub struct TgChat{
    pub id: u64
}
                
impl fmt::Display for TgChat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "chat_id: {0}", self.id)
                                }
                            }

#[derive(Deserialize)]
pub struct TgUpdate{
    pub update_id: u64,
    pub message: Option<TgMessage>//,
}

impl fmt::Display for TgUpdate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match &self.message {
                    Some(message) => write!(f, "update_Id: {0}; message: {1};", self.update_id, message),
                    _ => write!(f, "update_id: {0};", self.update_id)
                        }
                    }}
pub fn get_me(token: &str) -> Result<TgResponse<TgGetMeResult>,ureq::Error> {
    let requset_string: &str = &(format!("{0}{1}{2}", "https://api.telegram.org/bot", token, "/getMe"));
    let message: TgResponse<TgGetMeResult> = ureq::get(requset_string)
                        .call()?
                        .into_json()?;
    Ok(message)
}


pub fn get_updates(token: &str, update_id: &u64) -> Result<TgResponse<Vec<TgUpdate>>,ureq::Error> {
    let requset_string: &str = &(format!("{0}{1}{2}", "https://api.telegram.org/bot", token, "/getUpdates"));
    let update_id:&str = &format!("{}",update_id);
    let messages = ureq::get(requset_string)
                                .send_form(&[("offset",update_id),("timeout","10")])?
                                .into_json()?;
    Ok(messages)
}

pub fn send_echo(token: &str, message_id: u64, chat_id: u64) -> (){
    let requset_string:&str = &(format!("{0}{1}{2}", "https://api.telegram.org/bot", token, "/copyMessage"));
    let message_id_str:&str = &(format!("{}",message_id));
    let chat_id_str:&str = &(format!("{}",chat_id));
    let send_copy = ureq::get(requset_string)
            .send_form(&[("chat_id",chat_id_str),("from_chat_id",chat_id_str),("message_id",message_id_str)]);
    if send_copy.is_ok()
        {
            println!("message send")
        }
        else {println!("message not send")}

}