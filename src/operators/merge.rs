use std::{
    sync::{Arc, RwLock},
    thread,
};

use crate::types::{Message, Source};

pub fn merge<A: 'static>(a: Source<A>, b: Source<A>) -> Source<A> {
    let a = Arc::new(a);
    let b = Arc::new(b);
    Box::new(move |message| {
        let a = a.clone();
        let b = b.clone();
        if let Message::Start(sink) = message {
            let ended = Arc::new(RwLock::new(false));
            let end = ended.clone();
            sink(Message::Start(Box::new(move |msg| {
                if let Message::Stop = msg {
                    let mut e = ended.write().unwrap();
                    *e = true;
                }
            })));
            let s = move |x| sink(Message::Data(x));
            let sk = Arc::new(s);
            for sc in vec![a, b].iter() {
                let s = sk.clone();
                let end = end.clone();
                sc(Message::Start(Box::new(move |msg| match msg {
                    Message::Start(src) => {
                        let end = end.clone();
                        thread::spawn(move || {
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
        }
    })
}
