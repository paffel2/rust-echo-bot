use std::env;

use rust_echo_bot::*;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let token = &args[1];
    let check = get_me(token);
    let mut update_id: u64 = 0;
    let mut repeats:HashMap<u64,u32> = HashMap::new();
    match check {
        Ok(TgResponse {
            ok: true,
            error_code: _,
            description: _,
            result:
                Some(TgGetMeResult {
                    id: _,
                    is_bot: true,
                    first_name: _,
                    username: _,
                }),
        }) => loop {
            let update_result = get_updates(token, &update_id);

            match update_result {
                Ok(something) => {
                    for i in something.result.unwrap() {
                        let cl_data = i.callback_query;
                        match cl_data {
                            Some(cl_data1) => {
                                let c_id = cl_data1.from.id;
                                let repeat:u32 = cl_data1.data.parse::<u32>().unwrap();
                                repeats.insert(c_id,repeat);
                                send_help(token, c_id, "repeats updated");
                                println!("repeats updated");
                            }
                            None => {
                                let msg = i.message.unwrap();
                                let m_id = msg.message_id;
                                let c_id = msg.from.unwrap().id;
                                let message_type = to_message_type(msg.text.unwrap_or("".to_string()));
                                let repeats = repeats.get(&c_id).unwrap_or(&1);

                                match message_type {
                                    MessageType::Help => {
                                        send_help(token, c_id, "some help message");
                                    }
                                    MessageType::Repeat => {
                                        send_keyboard(token, c_id);
                                    }
                                    _ => {
                                        for _num in 0..*repeats {
                                        send_echo(token, m_id, c_id);}
                                    }
                                }
                            }
                        }
                        update_id = i.update_id + 1;
                    }
                }

                _ => println!("bad updates"),
            }
        },

        Err(err_message) => {
            println!("{}", err_message)
        }
        _ => {
            println!("something wrong")
        }
    }
}
