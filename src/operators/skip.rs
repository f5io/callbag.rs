use async_std::task;
use std::sync::{Arc, RwLock};

use crate::types::{Message, Through};

pub fn skip<A: 'static>(count: usize) -> Through<A, A> {
    Box::new(move |source| {
        Box::new(move |message| {
            if let Message::Start(sink) = message {
                let c = Arc::new(RwLock::new(0));
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
                        let end = end.clone();
                        task::spawn(async move {
                            loop {
                                if *end.read().unwrap() {
                                    break;
                                }
                            }
                            src(Message::Stop);
                        });
                    }
                    Message::Data(x) => {
                        let mut c = c.write().unwrap();
                        if *c != count {
                            *c += 1;
                        } else {
                            sink(Message::Data(x));
                        }
                    }
                    _ => {}
                })))
            }
        })
    })
}
