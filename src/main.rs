mod pool;

use pool::Pool;

fn main() {
    const HEIGHT: usize = 11;
    const WIDTH: usize = 11;
    let mut pool: Pool<HEIGHT, WIDTH> = Default::default();
    pool.randomize();
    for _ in 0..10 {
        pool.step();
        println!("Pool is :\n{}", pool.to_string());
    }
}
