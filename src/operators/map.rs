pub fn map<A: 'static, B: 'static, F: 'static>(f: F) -> Through<A, B>
where
    F: Fn(A) -> B + Send + Sync + Clone,
{
    Box::new(move |source| {
        let f = f.clone();
        Box::new(move |message|
            match message {
                Message::Start(sink) => {
                    let f = f.clone();
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
                            Message::Data(x) => { sink(Message::Data(f(x))) }
                            _ => {}
                        }          
                    ))) 
                }
                _ => {}
            } 
        )
    }) 
}

