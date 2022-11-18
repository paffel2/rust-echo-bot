mod lib;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let token = &args[1];
    let check = lib::get_me(token);
    let mut update_id: u64 = 0;
    if check.is_ok() {
        loop {
            let update_result = lib::get_updates(token, &update_id);

            match update_result {
                Ok(something) => {
                    for i in something.result {
                        let msg = i.message.unwrap();
                        let m_id = msg.message_id;
                        let c_id = msg.from.unwrap().id;
                        lib::send_echo(token, m_id, c_id);
                        update_id = i.update_id + 1;
                    }
                }

                _ => println!("no updates"),
            }
        }
    } else {
        println!("bad token")
    }
}
