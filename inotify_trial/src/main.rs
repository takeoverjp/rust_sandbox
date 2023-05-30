use std::{fs::File, io, io::Read, io::Write, path::Path, thread, time::Duration, time::Instant};

use futures::Stream;
use futures_util::StreamExt;
use inotify::{EventStream, Inotify, WatchMask};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::{Waker};

struct Subscriber {
    buffer: [u8; 1024],
}

#[derive(Debug)]
struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<&'static str>
    {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Get a handle to the waker for the current task
            let waker = cx.waker().clone();
            let when = self.when;

            // Spawn a timer thread.
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });

            Poll::Pending
        }
    }
}
#[derive(Debug)]
struct Interval {
    rem: usize,
    delay: Delay,
}

impl Interval {
    fn new() -> Self {
        Self {
            rem: 3,
            delay: Delay { when: Instant::now() }
        }
    }
}

impl Stream for Interval {
    type Item = i32;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Option<i32>>
    {
        if self.rem == 0 {
            // これ以上 delay しない
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + Duration::from_millis(1000);
                self.delay = Delay { when };
                self.rem -= 1;
                Poll::Ready(Some(1))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Subscriber {
    pub fn new() -> Self {
        Self { buffer: [0; 1024] }
    }
}
fn subscribe(
    buffer: &mut [u8; 1024],
) -> Result<futures_util::stream::Take<EventStream<&mut [u8; 1024]>>, io::Error> {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    let dir = Path::new("/tmp/inotify_trial/");
    inotify.add_watch(dir, WatchMask::CREATE | WatchMask::MODIFY)?;
    let stream = inotify.event_stream(buffer)?;
    Ok(stream.take(3))
}

use std::path::PathBuf;

struct InotifyAdapter<'a> {
    event_stream: inotify::EventStream<&'a mut [u8; 1024]>,
    count: i32,
}

impl<'a> InotifyAdapter<'a> {
    fn new(path: PathBuf, buffer: &'a mut [u8; 1024]) -> Result<Self, std::io::Error> {
        let mut inotify = Inotify::init()?;
        inotify.add_watch(&path, WatchMask::CREATE | WatchMask::MODIFY)?;

        let event_stream = inotify.event_stream(buffer).unwrap();

        Ok(InotifyAdapter { event_stream, count: 0 })
    }
}

impl<'a> Stream for InotifyAdapter<'a> {
    type Item = i32;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.event_stream).poll_next(cx) {
            Poll::Ready(Some(Ok(event))) => {
                self.count += 1;
                Poll::Ready(Some(self.count))
            },
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(self.count)),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let dir = Path::new("/tmp/inotify_trial/");
    let dir2 = dir.clone();

    thread::spawn::<_, Result<(), io::Error>>(move || {
        let mut count = 0;
        loop {
            let mut file = match File::create(dir.join("file")) {
                Err(why) => panic!("couldn't create file: {}", why),
                Ok(file) => file,
            };
            match file.write_all(format!("count = {}", count).as_bytes()) {
                Err(why) => panic!("couldn't write to file: {}", why),
                Ok(_) => println!("successfully wrote to file"),
            }
            thread::sleep(Duration::from_millis(500));
            count += 1;
        }
    });

    let mut buffer = [0; 1024];
    let mut stream = subscribe(&mut buffer)?.take(2).map(move |ev| {
        println!("hoge");
        ev
    });

    while let Some(event_or_error) = stream.next().await {
        // println!("event: {:?}", event_or_error?);
        if let Some(name) = event_or_error.unwrap().name {
            let name = name.into_string().unwrap();
            let mut file = match File::open(dir2.join(&name)) {
                Err(why) => panic!("couldn't open {}: {}", name, why),
                Ok(file) => file,
            };
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("couldn't read {}: {}", name, why),
                Ok(_) => println!("{} contains:\n---\n{}\n---", name, s),
            }
        }
    }

    let mut buf = [0; 1024];
    let mut my_inotify_stream = InotifyAdapter::new(PathBuf::from("/tmp/inotify_trial/"), &mut buf).unwrap();
    while let event = my_inotify_stream.next().await {
        println!("my_inotify_stream : {:?}", event);
        // TODO: busy loop
    }

    let mut interval_stream = Interval::new().take(4);
    while let event = interval_stream.next().await {
        // println!("event : {:?}", event);
        // TODO: busy loop
    }

    Ok(())
}
