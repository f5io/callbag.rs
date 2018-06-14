pub fn from_iter<I: 'static, A: 'static>(iter: I) -> Source<A>
where
    I: IntoIterator<Item=A> + Send + Sync + Clone,
{
    Box::new(move |message|
        match message {
            Message::Start(sink) => {
                let ended = Arc::new(AtomicBool::new(false));
                let end = ended.clone();
                let iter = iter.clone();
                sink(Message::Start(Box::new(move |msg|
                    match msg {
                        Message::Stop => { (*ended).store(true, Ordering::SeqCst) }
                        _ => {}
                    }
                )));
                for x in iter.into_iter() {
                    if (*end).load(Ordering::Relaxed) == true { break };
                    sink(Message::Data(x));
                }
            }
            _ => {}
        }
    )
}

