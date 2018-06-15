pub fn skip<A: 'static>(count: usize) -> Through<A, A> {
    Box::new(move |source| {
        Box::new(move |message|
            match message {
                Message::Start(sink) => {
                    let c = Arc::new(RwLock::new(0));
                    let ended = Arc::new(AtomicBool::new(false));
                    let end = ended.clone();
                    sink(Message::Start(Box::new(move |msg|
                        match msg {
                            Message::Stop => { (*ended).store(true, Ordering::SeqCst) }
                            _ => {}
                        }
                    )));
                    source(Message::Start(Box::new(move |msg| {
                        match msg {
                            Message::Start(src) => {
                                let end = end.clone();
                                thread::spawn(move || {
                                    loop { if (*end).load(Ordering::Relaxed) == true { break } }
                                    src(Message::Stop);
                                });
                            }
                            Message::Data(x) => {
                                let mut c = c.write().unwrap();
                                if *c != count { *c += 1; }
                                else { sink(Message::Data(x)); }
                            }
                            _ => {}
                        }
                    })))
                }
                _ => {}
            }
        )
    })
}
