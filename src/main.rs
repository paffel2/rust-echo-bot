//mod lib;
use std::env;

use rust_echo_bot::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let token = &args[1];
    let check = get_me(token);
    let mut update_id: u64 = 0;
    match check {
        Ok(TgResponse {
            ok: true,
            error_code: _,
            description: _,
            result:
                TgGetMeResult {
                    id: _,
                    is_bot: true,
                    first_name: _,
                    username: _,
                },
        }) => loop {
            let update_result = get_updates(token, &update_id);

            match update_result {
                Ok(something) => {
                    for i in something.result {
                        let msg = i.message;
                        let m_id = msg.message_id;
                        let c_id = msg.from.id;
                        send_echo(token, m_id, c_id);
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
