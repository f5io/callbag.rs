pub fn from_iter<I: 'static, A: 'static>(iter: I) -> Source<A>
where
    I: IntoIterator<Item=A> + Send + Sync + Clone,
{
    Box::new(move |message|
        match message {
            Message::Start(sink) => {
                let running = Arc::new(AtomicBool::new(true));
                let run = running.clone();
                let iter = iter.clone();
                sink(Message::Start(Box::new(move |msg|
                    match msg {
                        Message::Stop => { (*running).store(false, Ordering::Relaxed) }
                        _ => {}
                    }                      
                )));
                for x in iter.into_iter() {
                    if (*run).load(Ordering::Relaxed) != true { break };
                    sink(Message::Data(x));
                } 
            }
            _ => {}
        }       
    )
}

