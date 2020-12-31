use std::sync::{Arc, RwLock};

use crate::types::{Message, Source};

pub fn from_iter<I: 'static, A: 'static>(iter: I) -> Source<A>
where
    I: IntoIterator<Item = A> + Send + Sync + Clone,
{
    Box::new(move |message| {
        if let Message::Start(sink) = message {
            let ended = Arc::new(RwLock::new(false));
            let end = ended.clone();
            let iter = iter.clone();
            sink(Message::Start(Box::new(move |msg| {
                if let Message::Stop = msg {
                    let mut e = ended.write().unwrap();
                    *e = true;
                }
            })));
            for x in iter.into_iter() {
                if *end.read().unwrap() {
                    break;
                };
                sink(Message::Data(x));
            }
        }
    })
}
