use std::env;

use rust_echo_bot::*;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let token = &args[1];
    let check = get_me(token);
    let mut update_id: u64 = 0;
    let mut repeats: HashMap<u64, Status> = HashMap::new();
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
                                let repeat: u8 = cl_data1.data.parse::<u8>().unwrap();
                                let service_status = repeats.get(&c_id).unwrap();
                                delete_message(token, c_id, unwrap_status(*service_status));

                                repeats.insert(c_id, Status::CurrentNumber(repeat));

                                send_help(token, c_id, "repeats updated");
                                println!("repeats updated");
                            }
                            None => {
                                let msg = i.message.unwrap();
                                let m_id = msg.message_id;
                                let c_id = msg.from.unwrap().id;
                                let message_type =
                                    to_message_type(msg.text.unwrap_or("".to_string()));
                                let cur_repeat =
                                    repeats.get(&c_id).unwrap_or(&Status::CurrentNumber(1));

                                match message_type {
                                    MessageType::Help => {
                                        send_help(token, c_id, "some help message");
                                    }
                                    MessageType::Repeat => {
                                        let service_id = send_keyboard(token, c_id);
                                        repeats.insert(c_id, Status::WaitNumber(service_id));
                                    }
                                    _ => {
                                        // TODO - checks enums of repeats
                                        for _num in 0..unwrap_repeats(*cur_repeat) {
                                            send_echo(token, m_id, c_id);
                                        }
                                    }
                                }
                            }
                        }
                        update_id = i.update_id + 1;
                        //println!("{:#?}", repeats);
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
