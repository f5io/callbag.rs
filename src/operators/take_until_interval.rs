pub fn take_until_interval<A: 'static>(interval: u64) -> Through<A, A> {
    Box::new(move |source| {
        Box::new(move |message|
            match message {
                Message::Start(sink) => {
                    source(Message::Start(Box::new(move |msg|
                        match msg {
                            Message::Start(src) => {
                                thread::spawn(move || {
                                    thread::sleep(time::Duration::from_millis(interval));
                                    src(Message::Stop);
                                });
                            }
                            Message::Data(x) => { sink(Message::Data(x)) }
                            _ => {}
                        }
                    )))
                }
                _ => {}
            }
        )
    })
}


