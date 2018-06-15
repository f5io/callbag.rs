pub fn take<A: 'static>(count: usize) -> Through<A, A> {
    Box::new(move |source| {
        Box::new(move |message|
            match message {
                Message::Start(sink) => {
                    let count = Arc::new(RwLock::new(count));
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
                    source(Message::Start(Box::new(move |msg| {
                        match msg {
                            Message::Start(src) => {
                                let count = count.clone();
                                let end = end.clone();
                                thread::spawn(move || {
                                    loop {
                                        if *count.read().unwrap() <= 0
                                        || *end.read().unwrap() == true { break }
                                    }
                                    src(Message::Stop);
                                });
                            }
                            Message::Data(x) => {
                                let mut c = count.write().unwrap();
                                if *c != 0 {
                                    *c -= 1;
                                    sink(Message::Data(x));
                                }
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
