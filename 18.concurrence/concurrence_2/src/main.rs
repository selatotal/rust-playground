use std::thread;

fn main() {

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        print!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
