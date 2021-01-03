use crate::types::{Message, Through};

pub fn filter<A: 'static, F: 'static>(f: F) -> Through<A, A>
where
    F: Fn(&A) -> bool + Send + Sync + Clone,
{
    Box::new(move |source| {
        let f = f.clone();
        Box::new(move |message| {
            if let Message::Start(sink) = message {
                let f = f.clone();
                source(Message::Start(Box::new(move |msg| match msg {
                    Message::Start(src) => {
                        sink(Message::Start(Box::new(move |msg| {
                            if let Message::Stop = msg {
                                src(Message::Stop)
                            }
                        })));
                    }
                    Message::Data(x) => {
                        if f(&x) {
                            sink(Message::Data(x))
                        }
                    }
                    _ => {}
                })))
            }
        })
    })
}
