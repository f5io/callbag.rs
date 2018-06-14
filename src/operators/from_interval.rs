pub fn from_interval(interval: u64) -> Source<usize> {
    Box::new(move |message| {
        match message {
            Message::Start(sink) => {
                let ended = Arc::new(AtomicBool::new(false));
                let end = ended.clone();
                sink(Message::Start(Box::new(move |msg|
                    match msg {
                        Message::Stop => { (*ended).store(true, Ordering::SeqCst) }
                        _ => {}
                    }
                )));
                let _ = thread::spawn(move || for x in 0.. {
                    if (*end).load(Ordering::Relaxed) == true { break };
                    sink(Message::Data(x));
                    thread::sleep(time::Duration::from_millis(interval));
                }).join();
            }
            _ => {}
        }
    })
}
