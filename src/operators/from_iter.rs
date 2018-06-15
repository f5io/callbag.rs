pub fn from_iter<I: 'static, A: 'static>(iter: I) -> Source<A>
where
    I: IntoIterator<Item=A> + Send + Sync + Clone,
{
    Box::new(move |message|
        match message {
            Message::Start(sink) => {
                let ended = Arc::new(RwLock::new(false));
                let end = ended.clone();
                let iter = iter.clone();
                sink(Message::Start(Box::new(move |msg|
                    match msg {
                        Message::Stop => {
                            let mut e = ended.write().unwrap();
                            *e = true;
                        }
                        _ => {}
                    }
                )));
                for x in iter.into_iter() {
                    if *end.read().unwrap() == true { break };
                    sink(Message::Data(x));
                }
            }
            _ => {}
        }
    )
}

