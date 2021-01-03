use crate::types::{Message, Sink};

pub fn for_each<A: 'static, F: 'static>(f: F) -> Sink<A>
where
    F: Fn(A) + Send + Sync + Clone,
{
    Box::new(move |source| {
        let f = f.clone();
        source(Message::Start(Box::new(move |message| {
            if let Message::Data(x) = message {
                f(x)
            }
        })))
    })
}
