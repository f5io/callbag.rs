use async_std::task;
use std::sync::{Arc, RwLock};

use crate::types::{Message, Through};

pub fn take_until<A: 'static, F: 'static>(f: F) -> Through<A, A>
where
    F: Fn(&A) -> bool + Send + Sync + Clone,
{
    Box::new(move |source| {
        let f = f.clone();
        Box::new(move |message| {
            if let Message::Start(sink) = message {
                let f = f.clone();
                let taking = Arc::new(RwLock::new(true));
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
                        let mut taking = taking.write().unwrap();
                        if *taking {
                            *taking = !f(&x);
                        }
                        if *taking {
                            sink(Message::Data(x));
                        }
                    }
                    _ => {}
                })))
            }
        })
    })
}
