pub enum Message<T> {
    Start(Handler<T>),
    Data(T),
    Stop,
}

pub type Handler<T> = Box<Fn(Message<T>) + Send + Sync>;
pub type Transform<A, B> = Box<(Fn(A) -> B) + Send + Sync>;
pub type Source<T> = Handler<T>;
pub type Through<A, B> = Transform<Source<A>, Handler<B>>;
pub type Sink<T> = Box<Fn(Source<T>)>;

