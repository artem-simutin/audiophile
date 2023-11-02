use serenity::{model::prelude::Message, Error};

pub fn check_msg_err(res: Result<Message, Error>) -> () {
    if let Err(why) = res {
        println!("❌ ============");
        println!(
            "❌ Something went wrong sending message due DISCORD issue: {:?}",
            why
        );
        println!("❌ ============");
    }
}
