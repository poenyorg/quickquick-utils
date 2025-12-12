use crossbeam::channel::{Receiver, SendError, Sender, unbounded};
use once_cell::sync::{Lazy, OnceCell};
use std::env;
use teloxide::prelude::*;

pub static NOTIFIER: Lazy<Notifier> = Lazy::new(|| Notifier {
    send: OnceCell::new(),
    recv: OnceCell::new(),
});

pub struct NotifyRecord {
    message: String,
}
pub struct Notifier {
    send: OnceCell<Sender<NotifyRecord>>,
    recv: OnceCell<Receiver<NotifyRecord>>,
}

impl Notifier {
    pub fn logs(&self, mess: &str) -> Result<(), SendError<NotifyRecord>> {
        let record = NotifyRecord {
            message: mess.to_string(),
        };
        if let Some(send) = NOTIFIER.send.get() {
            send.send(record)
        } else {
            Err(SendError(record))
        }
    }
}

pub fn init() {
    let (s, r) = unbounded();
    NOTIFIER.send.set(s).unwrap();
    NOTIFIER.recv.set(r).unwrap();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let receiver = NOTIFIER.recv.get().unwrap();
            let bot_token = env::var("TELEGRAM_BOT_KEY").unwrap();

            // Create proxy
            // let proxy_url = format!(
            //     "socks5://{}:{}@{}:{}",
            //     env::var("PROXY_USERNAME").unwrap(),
            //     env::var("PROXY_PASSWORD").unwrap(),
            //     env::var("PROXY_URL").unwrap(),
            //     env::var("PROXY_PORT").unwrap()
            // );
            // tracing::info!("proxy url {}", proxy_url);
            // let proxy = Proxy::all(proxy_url).expect("Failed to create proxy");

            // Create custom HTTP client with proxy
            // let reqwest_client = reqwest::ClientBuilder::new()
            //     .proxy(proxy)
            //     .build()
            //     .expect("Failed to build reqwest client");

            // Create bot with custom HTTP client
            let bot = Bot::new(bot_token);
            let telegram_chat_id = env::var("TELEGRAM_CHAT_ID").unwrap();

            loop {
                let log_record = receiver.recv();
                match log_record {
                    Ok(log) => {
                        if bot
                            .send_message(telegram_chat_id.clone(), log.message)
                            .await
                            .is_err()
                        {
                            tracing::error!("Fail to send telegram message");
                        }
                    }
                    Err(_) => {
                        tracing::error!("Fail to print")
                    }
                }
            }
        });
    });
}

pub fn send(message: &str) -> Result<(), SendError<NotifyRecord>> {
    NOTIFIER.logs(message)
}
