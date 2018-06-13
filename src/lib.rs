#[cfg(test)]
mod tests {
    use std::{ thread, time };
    use std::sync::{ Arc };
    use std::sync::atomic::{ AtomicBool, AtomicUsize, Ordering };

    enum Message<T> {
        Start(Handler<T>),
        Data(T),
        Stop,
    }

    type Handler<T> = Box<Fn(Message<T>) + Send + Sync>;
    type Transform<A, B> = Box<(Fn(A) -> B) + Send + Sync>;
    type Source<T> = Handler<T>;
    type Through<A, B> = Transform<Source<A>, Handler<B>>;
    type Sink<T> = Box<Fn(Source<T>)>;

    fn from_interval(interval: u64) -> Source<i32> {
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

    fn map<A: 'static, B: 'static, F: 'static>(f: F) -> Through<A, B>
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

    fn filter<A: 'static, F: 'static>(f: F) -> Through<A, A>
    where
        F: Fn(&A) -> bool + Send + Sync + Clone,
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
                                Message::Data(x) => {
                                    if f(&x) == true {
                                        sink(Message::Data(x))
                                    } 
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

    fn take_until_interval<A: 'static>(interval: u64) -> Through<A, A> {
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

    fn take<A: 'static>(count: usize) -> Through<A, A> {
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

    fn for_each<A: 'static, F: 'static>(f: F) -> Sink<A>
    where 
        F: Fn(A) -> () + Send + Sync + Clone,
    {
        Box::new(move |source| {
            let f = f.clone();
            source(Message::Start(Box::new(move |message| {
                match message {
                    Message::Data(x) => { f(x) }
                    _ => {}
                }
            })))
        }) 
    }

    macro_rules! pipe {
        ($a:expr, $b:expr) => ($b($a));
        ($a:expr, $b:expr, $($rest:expr),*) => {
            pipe!($b($a), $($rest),*)
        };
    }

    #[test]
    fn it_works() {
        pipe!(
            from_interval(1000),
            map(|x| x * 3),
            filter(|x| x % 2 == 0),
            map(|x| format!("The number is {}", x)),
            take(30),
            for_each(|x| println!("{}", x))
        );

        assert_eq!(2 + 2, 4);
    }
}
