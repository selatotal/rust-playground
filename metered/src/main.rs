use metered::{metered, Throughput, HitCount};
use serde;
use serde_yaml;
use std::thread;
use rand;
use std::sync::Arc;


#[derive(Default, Debug, serde::Serialize)]
pub struct Biz {
    metrics: BizMetrics,
}

#[metered(registry = BizMetrics)]
impl Biz {
    #[measure([HitCount, Throughput])]
    pub fn biz(&self){
        let delay = std::time::Duration::from_millis(rand::random::<u64>() % 200);
        thread::sleep(delay);
    }
}

fn main() {
    let biz = Arc::new(Biz::default());
    let mut threads = Vec::new();
    for _ in 0..5 {
        let biz = Arc::clone(&biz);
        let t = thread::spawn(move || {
            for _ in 0..200 {
                biz.biz();
            }
        });
        threads.push(t);
    }
    for t in threads {
        t.join().unwrap();
    }
    let serialized = serde_yaml::to_string(&*biz).unwrap();
    println!("{}", serialized);
}
