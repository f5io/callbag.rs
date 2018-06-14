pub fn take<A: 'static>(count: usize) -> Through<A, A> {
    Box::new(move |source| {
        Box::new(move |message|
            match message {
                Message::Start(sink) => {
                    let c = Arc::new(AtomicUsize::new(count));
                    source(Message::Start(Box::new(move |msg| {
                        match msg {
                            Message::Start(src) => {
                                let c = c.clone();
                                thread::spawn(move || {
                                    loop { if (*c).load(Ordering::Relaxed) == 0 { break } }
                                    src(Message::Stop);
                                });
                            }
                            Message::Data(x) => {
                                sink(Message::Data(x));
                                (*c).fetch_sub(1, Ordering::Relaxed);
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
