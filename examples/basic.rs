extern crate rodio;

use std::io::BufReader;
use std::thread;
use std::time::Duration;

fn main() {
    let endpoint = rodio::get_default_endpoint().unwrap();

    let file = std::fs::File::open("examples/beep.wav").unwrap();
    let mut beep1 = rodio::play_once(&endpoint, BufReader::new(file)).unwrap();
    beep1.set_volume(0.2);
    println!("Started beep1");

    thread::sleep(Duration::from_millis(1500));

    let file = std::fs::File::open("examples/beep2.wav").unwrap();
    rodio::play_once(&endpoint, BufReader::new(file))
        .unwrap()
        .detach();
    println!("Started beep2");

    thread::sleep(Duration::from_millis(1500));
    let file = std::fs::File::open("examples/beep3.ogg").unwrap();
    let beep3 = rodio::play_once(&endpoint, file).unwrap();
    println!("Started beep3");

    thread::sleep(Duration::from_millis(1500));
    drop(beep1);
    println!("Stopped beep1");

    thread::sleep(Duration::from_millis(1500));
    drop(beep3);
    println!("Stopped beep3");

    thread::sleep(Duration::from_millis(1500));
}
