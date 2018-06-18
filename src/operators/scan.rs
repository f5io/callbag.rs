pub fn scan<A: 'static, B: 'static, F: 'static>(f: F, b: B) -> Through<A, B>
where
    F: Fn(B, A) -> B + Send + Sync + Clone,
    B: Send + Sync + Clone,
{
    let b = Arc::new(RwLock::new(Some(b)));
    Box::new(move |source| {
        let f = f.clone();
        let b = b.clone();
        Box::new(move |message|
            match message {
                Message::Start(sink) => {
                    let f = f.clone();
                    let b = b.clone();
                    source(Message::Start(Box::new(move |msg|
                        match msg {
                            Message::Start(src) => {
                                sink(Message::Start(Box::new(move |msg|
                                    match msg {
                                        Message::Stop => { src(Message::Stop) }
                                        _ => {}
                                    }
                                )));
                            }
                            Message::Data(x) => {
                                let acc = b.read().unwrap().clone();
                                {
                                    let mut b = b.write().unwrap();
                                    *b = Some(f(acc.unwrap(), x));
                                }
                                let acc = b.read().unwrap().clone();
                                sink(Message::Data(acc.unwrap()))
                            }
                            _ => {}
                        }
                    )))
                }
                _ => {}
            }
        )
    })
}
