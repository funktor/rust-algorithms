use fasthash::murmur3;
use std::cmp::max;
use rand::{distributions::Alphanumeric, Rng};
use rand::seq::SliceRandom;

fn get_trailing_zeros(mut n:u32, num_bits:u8)->u8 {
    let mut j:u8 = 0;

    if n == 0 {
        j = num_bits;
    }
        
    else {
        while n > 0 {
            if (n & 1) == 1 {
                break;
            }
            n = n >> 1;
            j += 1;
        }
    }

    return j;
}

fn hmean(vector:Vec<u32>)->f32 {
    let mut sum:f32 = 0.0;
    let n = vector.len();
    for i in 0..n {
        sum += 1.0/vector[i] as f32;
    }

    return n as f32/sum;
}
    

struct HLL {
    num_bits: u8,
    p_bits: u8,
    m: u32,
    buckets: Vec<i16>,
    alpha: f32,
}

impl HLL {
    fn new(p:u8) -> Self {
        let bsize:u32 = 1<<p;
        let mut alp:f32 = 0.7213/(1.0 + 1.079/bsize as f32);

        if bsize == 16 {
            alp = 0.673;
        }
        else if bsize == 32 {
            alp = 0.697;
        }
        else if bsize == 64 {
            alp = 0.709;
        }

        Self {
            num_bits: 31,
            p_bits: p,
            m: bsize,
            buckets: vec![-1;bsize as usize],
            alpha: alp,
        }
    }
}

impl HLL {
    fn add(&mut self, data:&String) {
        let mut hsh = murmur3::hash32_with_seed(data.as_bytes(), 42);
        hsh = hsh & ((1<<self.num_bits)-1);

        let v = self.num_bits-self.p_bits;
        let q = hsh & ((1<<v)-1);
        let r = hsh>>v;
        let j = get_trailing_zeros(q, v);
        self.buckets[r as usize] = max(self.buckets[r as usize], (j+1).into());
    }
}

impl HLL {
    fn get_size(&self)->u32 {
        let u:u32 = 1 << self.num_bits;
        let mut v:Vec<u32> = Vec::new();

        for x in self.buckets.iter() {
            v.push(u32::pow(2, *x as u32));
        }

        let n:f32 = self.alpha*self.m as f32*hmean(v);

        if n <= 2.5*self.m as f32 {
            let mut z:u32 = 0;
            for i in 0..self.m {
                if self.buckets[i as usize] == 0 {
                    z += 1;
                }
            }

            if z != 0 {
                let q = f32::ln(self.m as f32/z as f32);
                return self.m*q as u32;
            }
        }

        else if n > (1.0/30.0)*u as f32 {
            let q = -f32::ln(1.0-(n/u as f32));
            return u*q as u32;
        }
        
        return n as u32;
    }
}


fn main() {
    let k = rand::thread_rng().gen_range(100..100000);
    let mut data:Vec<String> = Vec::new();

    for _ in 0..k {
        let u = rand::thread_rng().gen_range(1..20);
        let s: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(u)
                    .map(char::from)
                    .collect();
        data.push(s);
    }

    let data_sampled = data.choose_multiple(&mut rand::thread_rng(), 1000000);
    let mut hll = HLL::new(9);

    for s in data_sampled {
        hll.add(s);
    }

    let a = hll.get_size();
    let x:f32 = u32::abs_diff(k, a) as f32/k as f32;
    println!("{:?}, {:?}, {:?}", k, a, x);

}