use std::{fs::File, io, io::Read, io::Write, path::Path, thread, time::Duration};

use futures_util::StreamExt;
use inotify::{Inotify, WatchMask};
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    let dir = Path::new("/tmp/inotify_trial/");
    let dir2 = dir.clone();

    inotify.add_watch(dir, WatchMask::CREATE | WatchMask::MODIFY)?;

    thread::spawn::<_, Result<(), io::Error>>(move || loop {
        let mut file = match File::create(dir.join("file")) {
            Err(why) => panic!("couldn't create file: {}", why),
            Ok(file) => file,
        };
        match file.write_all("hoge".as_bytes()) {
            Err(why) => panic!("couldn't write to file: {}", why),
            Ok(_) => println!("successfully wrote to file"),
        }
        thread::sleep(Duration::from_millis(500));
    });

    let mut buffer = [0; 1024];
    let mut stream = inotify.event_stream(&mut buffer)?;

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

    Ok(())
}
