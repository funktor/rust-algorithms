use rand::thread_rng;
use rand::seq::SliceRandom;

fn update_vector(myvec:&mut Box<Vec<usize>>) {
    for x in myvec.iter_mut() {
        *x += 1;
    }
}
fn main() {
    let mut vec: Vec<usize> = (1..1000000).collect();
    vec.shuffle(&mut thread_rng());

    update_vector(&mut Box::new(vec));
}