use std::collections::HashMap;

use rand::{seq::SliceRandom, Rng};


/*
k: i32, v: i32の結果
M3 macでは、要素数10kから100kの間にvecとhashmapの速度のブレークポイントがあるようだ。

--- loop: 1000, contents: 1 ---
dualation: 'create vec' took 0.21us
dualation: 'shuffle vec' took 2.92us
dualation: 'create hashmap' took 2.83us
dualation: 'insert hashmap' took 1.71us
dualation: 'vec search' took 1.33us
dualation: 'hashmap search' took 2.92us
--- loop: 1000, contents: 10 ---
dualation: 'create vec' took 0.17us
dualation: 'shuffle vec' took 1.58us
dualation: 'create hashmap' took 1.50us
dualation: 'insert hashmap' took 2.38us
dualation: 'vec search' took 1.92us
dualation: 'hashmap search' took 2.96us
--- loop: 1000, contents: 100 ---
dualation: 'create vec' took 0.17us
dualation: 'shuffle vec' took 3.21us
dualation: 'create hashmap' took 2.25us
dualation: 'insert hashmap' took 4.00us
dualation: 'vec search' took 1.50us
dualation: 'hashmap search' took 3.12us
--- loop: 1000, contents: 1000 ---
dualation: 'create vec' took 0.50us
dualation: 'shuffle vec' took 20.88us
dualation: 'create hashmap' took 3.88us
dualation: 'insert hashmap' took 17.79us
dualation: 'vec search' took 1.58us
dualation: 'hashmap search' took 2.83us
--- loop: 1000, contents: 10000 ---
dualation: 'create vec' took 16.54us
dualation: 'shuffle vec' took 198.21us
dualation: 'create hashmap' took 6.67us
dualation: 'insert hashmap' took 180.83us
dualation: 'vec search' took 2.75us
dualation: 'hashmap search' took 3.25us
--- loop: 1000, contents: 100000 ---
dualation: 'create vec' took 106.88us
dualation: 'shuffle vec' took 1987.00us
dualation: 'create hashmap' took 19.67us
dualation: 'insert hashmap' took 2051.88us
dualation: 'vec search' took 2.92us
dualation: 'hashmap search' took 2.54us
--- loop: 1000, contents: 1000000 ---
dualation: 'create vec' took 885.04us
dualation: 'shuffle vec' took 15215.58us
dualation: 'create hashmap' took 247.12us
dualation: 'insert hashmap' took 15334.92us
dualation: 'vec search' took 15.92us
dualation: 'hashmap search' took 2.29us
--- loop: 1000, contents: 10000000 ---
dualation: 'create vec' took 6887.75us
dualation: 'shuffle vec' took 162905.71us
dualation: 'create hashmap' took 1333.79us
dualation: 'insert hashmap' took 237717.38us
dualation: 'vec search' took 14.54us
dualation: 'hashmap search' took 2.12us
--- loop: 1000, contents: 100000000 ---
dualation: 'create vec' took 113763.92us
dualation: 'shuffle vec' took 1624629.17us
dualation: 'create hashmap' took 18889.58us
dualation: 'insert hashmap' took 4279178.88us
dualation: 'vec search' took 26.75us
dualation: 'hashmap search' took 2.38us
*/


enum TimeUnit {
    Ms,
    Us,
}
struct Timer {
    start: std::time::Instant,
    unit: TimeUnit,
}
impl Timer {
    pub fn new(unit: TimeUnit) -> Self {
        Self {
            start: std::time::Instant::now(),
            unit,
        }
    }
    pub fn reset_interval(&mut self) {
        self.start = std::time::Instant::now();
    }
    pub fn print_interval(&mut self, name: &str) {
        let now = std::time::Instant::now();
        match self.unit {
            TimeUnit::Ms => {
                let elapsed = (now - self.start).as_secs_f64()*1000.0;
                println!("dualation: '{}' took {:.2?}ms", name, elapsed);
            },
            TimeUnit::Us => {
                let elapsed = (now - self.start).as_secs_f64()*1000_000.0;
                println!("dualation: '{}' took {:.2?}us", name, elapsed);
            }
        }
        self.start = now;
    }   
}

fn main() {
    // 計測平均化のためのループ回数
    let loop_n = 1000;
    // 一回の要素数
    let digit = 9;
    
    
    let mut rng = rand::thread_rng();
    let mut timer = Timer::new(TimeUnit::Us);

    
    for d in 0..digit {
        let n = 10.0_f32.powi(d) as i32;
        let search_target = rng.gen_range(0..n);


        println!("--- loop: {}, contents: {} ---", loop_n, n);

        timer.reset_interval();
        let mut vec: Vec<(i32, i32)> = (0..n).into_iter().map(|i| (i, i)).collect();
        timer.print_interval("create vec");
        vec.shuffle(&mut rng);
        timer.print_interval("shuffle vec");
        let mut hashmap: HashMap<i32, i32> = HashMap::with_capacity(n as usize);
        timer.print_interval("create hashmap");
        for (k, v) in vec.iter() {
            hashmap.insert(*k, *v);
        }
        timer.print_interval("insert hashmap");

        
        for _ in 0..loop_n {
            // search vec
            for (k, _) in vec.iter() {
                if *k == search_target {
                    break;
                }
            }
        }
        timer.print_interval("vec search");

        for _ in 0..loop_n {
            // search hashmap
            hashmap.get(&search_target).unwrap();
        }
        timer.print_interval("hashmap search");
    }
}
