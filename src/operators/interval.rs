use async_std::task;
use std::{
    sync::{Arc, RwLock},
    time,
};

use crate::types::{Message, Source};

pub fn interval(interval: u64) -> Source<usize> {
    Box::new(move |message| {
        if let Message::Start(sink) = message {
            let ended = Arc::new(RwLock::new(false));
            let end = ended.clone();
            sink(Message::Start(Box::new(move |msg| {
                if let Message::Stop = msg {
                    let mut e = ended.write().unwrap();
                    *e = true;
                }
            })));
            task::spawn(async move {
                for x in 0.. {
                    if *end.read().unwrap() {
                        break;
                    };
                    sink(Message::Data(x));
                    task::sleep(time::Duration::from_millis(interval)).await;
                }
            });
        }
    })
}
