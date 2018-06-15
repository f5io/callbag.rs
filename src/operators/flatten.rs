pub fn flatten<A: 'static>(source: Source<Source<A>>) -> Handler<A> {
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
                let s = Arc::new(move |x| sink(Message::Data(x)));
                source(Message::Start(Box::new(move |msg| {
                    match msg {
                        Message::Start(src) => {
                            let end = end.clone();
                            thread::spawn(move || {
                                loop { if *end.read().unwrap() == true { break } } 
                                src(Message::Stop);
                            });    
                        }
                        Message::Data(sc) => {
                            let s = s.clone();
                            let end = end.clone();
                            sc(Message::Start(Box::new(move |msg| {
                                match msg {
                                    Message::Start(src) => {
                                        let end = end.clone();
                                        thread::spawn(move || {
                                            loop { if *end.read().unwrap() == true { break } } 
                                            src(Message::Stop); 
                                        });
                                    }
                                    Message::Data(x) => { s(x) } 
                                    _ => {}
                                } 
                            })))
                        }
                        _ => {}
                    } 
                })))
            }
            _ => {}
        }
    })
}
