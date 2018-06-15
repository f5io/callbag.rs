pub fn interval(interval: u64) -> Source<usize> {
    Box::new(move |message| {
        match message {
            Message::Start(sink) => {
                let ended = Arc::new(RwLock::new(false));
                let end = ended.clone();
                sink(Message::Start(Box::new(move |msg|
                    match msg {
                        Message::Stop => {
                            let mut e = ended.write().unwrap();
                            *e = true;
                        }
                        _ => {}
                    }
                )));
                thread::spawn(move || for x in 0.. {
                    if *end.read().unwrap() == true { break };
                    sink(Message::Data(x));
                    thread::sleep(time::Duration::from_millis(interval));
                });
            }
            _ => {}
        }
    })
}
