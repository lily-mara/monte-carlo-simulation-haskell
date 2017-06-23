extern crate rand;

use rand::{Rng, SeedableRng};

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn random<T>(gen: &mut T) -> Self
        where T: Rng
    {
        Point {
            x: gen.gen(),
            y: gen.gen(),
        }
    }

    fn weight(&self) -> i64 {
        if (self.x * self.x + self.y * self.y).sqrt() <= 1.0 {
            1
        } else {
            0
        }
    }
}

fn main() {
    let limit = std::env::args()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    println!("{}", pi(limit));
}

fn pi(limit: usize) -> f64 {
    let mut points = Vec::with_capacity(limit);
    let mut gen = get_rng();

    for _ in 0..limit {
        points.push(Point::random(&mut gen));
    }

    let total_weight = points.iter().fold(0, |x, y| x + y.weight()) as f64;

    total_weight / points.len() as f64 * 4.0
}

fn get_rng() -> rand::XorShiftRng {
    let mut default_rng = rand::thread_rng();

    let buf = [default_rng.next_u32(),
               default_rng.next_u32(),
               default_rng.next_u32(),
               default_rng.next_u32()];

    rand::XorShiftRng::from_seed(buf)
}
