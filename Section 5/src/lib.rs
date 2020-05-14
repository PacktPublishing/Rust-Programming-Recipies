//#![feature(async_await)]

use futures::io::AsyncRead;
use futures::stream::Stream;
use futures::task::Context;
use futures::Poll;
use std::pin::Pin;

pub mod simple;

pub struct ReadStream<A: AsyncRead + Unpin> {
    reader: A,
    buf: [u8; 100],
}

impl<A: AsyncRead + Unpin> ReadStream<A> {
    pub fn new(reader: A) -> Self {
        ReadStream {
            reader,
            buf: [0; 100],
        }
    }
}

impl<A: AsyncRead + Unpin> Stream for ReadStream<A> {
    type Item = String;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let up = self.get_mut();
        let r = Pin::new(&mut up.reader);
        match r.poll_read(cx, &mut up.buf) {
            Poll::Ready(Ok(len)) => {
                Poll::Ready(Some(String::from_utf8_lossy(&up.buf[..len]).to_string()))
            }
            Poll::Ready(Err(_e)) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
