use std::thread;
use std::time::Duration;

fn main() {
    let list = [8, 1, 4, 9, 2, 3, 7, 6, 5];
    let mut handles = vec![];

    for number in list {
        let handle = std::thread::spawn(move || {
            thread::sleep(Duration::from_secs(number));
            println!("{}", number);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
