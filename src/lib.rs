pub use co_proc_macro::coroutine;

pub struct Coroutine {
    step: usize,
    elapsed: f64,
}

#[allow(dead_code)]
impl Coroutine {
    pub const fn new() -> Self {
        Coroutine {
            step: 0,
            elapsed: 0.0,
        }
    }

    pub fn halt(&mut self) {
        self.step += 1;
    }

    pub fn reset(&mut self) {
        self.step = 0;
    }

    pub fn step(&self) -> usize {
        self.step
    }

    pub fn wait(&mut self, dt: f64, wait_secs: f64) {
        self.elapsed += dt;
        if self.elapsed >= wait_secs {
            self.elapsed = 0.0;
            self.step += 1;
        }
    }

    pub fn wait_until<F>(&mut self, f: F)
    where
        F: Fn() -> bool,
    {
        self.step += f() as usize;
    }
}

#[test]
fn test_1() {
    use std::time::Duration;
    let mut co = Coroutine::new();
    let dt_dur = Duration::from_secs_f64(1.0 / 60.0);
    let _dt = dt_dur.as_secs_f64();
    let mut i = 0;
    loop {
        std::thread::sleep(dt_dur);
        coroutine!(
            {
                println!("Testing 0");
                co.halt();
            },
            {
                println!("Testing 1");
                co.halt();
            },
            {
                println!("Testing 2");
                co.halt();
            },
            co.wait(0.016, 3.0),
            {
                println!("Testing 3");
                co.halt();
            },
            co.wait_until(|| i >= 400),
            {
                println!("Testing 4");
                break;
            },
        );
        i += 1;
    }
}
