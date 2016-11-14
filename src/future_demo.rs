use futures::{Future, Map, Poll};

struct WutDemo {
    foo: u32
}

enum WutError {
    NotReady,
    BadError
}

impl Future for WutDemo {
    type Item = u32;
    type Error = WutError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Err(WutError::BadError)
    }
}
