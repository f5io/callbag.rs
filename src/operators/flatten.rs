use async_std::task;
use std::sync::{Arc, RwLock};

use crate::types::{Handler, Message, Source};

pub fn flatten<A: 'static>(source: Source<Source<A>>) -> Handler<A> {
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
            let s = Arc::new(move |x| sink(Message::Data(x)));
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
                Message::Data(sc) => {
                    let s = s.clone();
                    let end = end.clone();
                    sc(Message::Start(Box::new(move |msg| match msg {
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
                        Message::Data(x) => s(x),
                        _ => {}
                    })))
                }
                _ => {}
            })))
        }
    })
}
