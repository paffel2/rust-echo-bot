mod lib;


fn main() {

    let check = lib::get_me("token");
    if check.is_ok()
        {
            let update_result = lib::get_updates("token");

            match update_result
            {
                Ok(something) => for i in something.result
                                                    {   
                                                        let msg = i.message.unwrap();
                                                        let m_id = msg.message_id;
                                                        let c_id = msg.from.unwrap().id;
                                                        lib::send_echo("token",m_id,c_id);

                                                    },

                _ => println!("no updates")
            }
        } else {println!("bad token")}

}
