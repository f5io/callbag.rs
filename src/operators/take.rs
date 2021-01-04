use async_std::task;
use std::sync::{Arc, RwLock};

use crate::types::{Message, Through};

pub fn take<A: 'static>(count: usize) -> Through<A, A> {
    Box::new(move |source| {
        Box::new(move |message| {
            if let Message::Start(sink) = message {
                let count = Arc::new(RwLock::new(count));
                let ended = Arc::new(RwLock::new(false));
                let end = ended.clone();
                sink(Message::Start(Box::new(move |msg| {
                    if let Message::Stop = msg {
                        let mut e = ended.write().unwrap();
                        *e = true;
                    }
                })));
                source(Message::Start(Box::new(move |msg| match msg {
                    Message::Start(src) => {
                        let count = count.clone();
                        let end = end.clone();
                        task::spawn(async move {
                            loop {
                                if *count.read().unwrap() == 0 || *end.read().unwrap() {
                                    break;
                                }
                            }
                            src(Message::Stop);
                        });
                    }
                    Message::Data(x) => {
                        let mut c = count.write().unwrap();
                        if *c != 0 {
                            *c -= 1;
                            sink(Message::Data(x));
                        }
                    }
                    _ => {}
                })))
            }
        })
    })
}
