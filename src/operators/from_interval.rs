pub fn from_interval(interval: u64) -> Source<usize> {
    Box::new(move |message| {
        match message {
            Message::Start(sink) => {
                let running = Arc::new(AtomicBool::new(true));
                let run = running.clone();
                sink(Message::Start(Box::new(move |msg|
                    match msg {
                        Message::Stop => { (*running).store(false, Ordering::Relaxed) }
                        _ => {}
                    }
                )));
                let _ = thread::spawn(move || for x in 0.. {
                    if (*run).load(Ordering::Relaxed) != true { break };
                    sink(Message::Data(x));
                    thread::sleep(time::Duration::from_millis(interval));
                }).join();
            }
            _ => {}
        }
    })
}
