pub fn combine<A: 'static>(a: Source<A>, b: Source<A>) -> Source<A> {
    let a = Arc::new(a);
    let b = Arc::new(b);
    Box::new(move |message| {
        let a = a.clone();
        let b = b.clone();
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
                let s = move |x| sink(Message::Data(x));
                let sk = Arc::new(s);
                for sc in vec![a, b].iter() {
                    let s = sk.clone();  
                    let end = end.clone();
                    sc(Message::Start(Box::new(move |msg| {
                        match msg {
                            Message::Start(src) => {
                                let end = end.clone();
                                thread::spawn(move || {
                                    loop { if (*end).load(Ordering::Relaxed) == true { break } }
                                    src(Message::Stop);
                                });
                            }
                            Message::Data(x) => { s(x) }
                            _ => {}
                        } 
                    })))
                }
            }
            _ => {}
        }
    })
}
