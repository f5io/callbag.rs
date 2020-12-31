use std::sync::{Arc, RwLock};

use crate::types::{Message, Through};

pub fn scan<A: 'static, B: 'static, F: 'static>(f: F, b: B) -> Through<A, B>
where
    F: Fn(B, A) -> B + Send + Sync + Clone,
    B: Send + Sync + Clone,
{
    let b = Arc::new(RwLock::new(Some(b)));
    Box::new(move |source| {
        let f = f.clone();
        let b = b.clone();
        Box::new(move |message| {
            if let Message::Start(sink) = message {
                let f = f.clone();
                let b = b.clone();
                source(Message::Start(Box::new(move |msg| match msg {
                    Message::Start(src) => {
                        sink(Message::Start(Box::new(move |msg| {
                            if let Message::Stop = msg {
                                src(Message::Stop)
                            }
                        })));
                    }
                    Message::Data(x) => {
                        let acc = b.read().unwrap().clone();
                        {
                            let mut b = b.write().unwrap();
                            *b = Some(f(acc.unwrap(), x));
                        }
                        let acc = b.read().unwrap().clone();
                        sink(Message::Data(acc.unwrap()))
                    }
                    _ => {}
                })))
            }
        })
    })
}
