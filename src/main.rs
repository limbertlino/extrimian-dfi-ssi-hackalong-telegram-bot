use dotenv::dotenv;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

mod handlers;
mod models;
mod utils;

use crate::handlers::{receive_category, receive_name, start};
use crate::models::State;

async fn run_bot() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        dptree::entry()
            .branch(
                Update::filter_message()
                    .enter_dialogue::<Message, InMemStorage<State>, State>()
                    .branch(dptree::case![State::Start].endpoint(start))
                    .branch(dptree::case![State::ReceiveName].endpoint(receive_name)),
            )
            .branch(
                Update::filter_callback_query()
                    .enter_dialogue::<CallbackQuery, InMemStorage<State>, State>()
                    .branch(
                        dptree::case![State::ReceiveCategory { name }].endpoint(receive_category),
                    ),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    run_bot().await;
}
