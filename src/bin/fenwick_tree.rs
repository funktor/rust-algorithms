struct FenwickTree {
    bits:Vec<isize>
}

impl FenwickTree {
    fn new(vec:&Vec<isize>) -> Self {
        let n = vec.len();
        let mut mybits:Vec<isize> = vec![0;n];

        for i in 0..n {
            mybits[i] += vec[i];
            let r = i | (i+1);
            if r < n {
                mybits[r] += mybits[i];
            }
        }
        
        Self {
            bits: mybits,
        }
    }
}

impl FenwickTree {
    fn get_sum(&self, mut idx:usize) -> isize {
        let mut sum:isize = 0;

        loop {
            sum += self.bits[idx];
            idx = idx & (idx + 1);
            if idx == 0 {
                break;
            }
            idx = idx-1;
        }

        return sum;
    }
}

impl FenwickTree {
    fn get_sum_range(&self, lt:usize, rt:usize) -> Option<isize> {
        let lt_sum:isize;
        let rt_sum:isize;

        if rt >= lt {
            if lt > 0 {
                lt_sum = self.get_sum(lt-1);
            }
            else {
                lt_sum = 0;
            }
            rt_sum = self.get_sum(rt);
            return Some(rt_sum-lt_sum);
        }
        
        return None;
    }
}

impl FenwickTree {
    fn update(&mut self, mut idx:usize, delta:isize) {
        while idx < self.bits.len() {
            self.bits[idx] += delta;
            idx = idx | (idx + 1);
        }
    }
}

fn main() {
    let inp:Vec<isize> = vec![1,2,3,4,5,6,7,8];
    let mut fw = FenwickTree::new(&inp);
    println!("{:?}", fw.get_sum_range(1, 5));
    fw.update(2, 10);
    println!("{:?}", fw.get_sum_range(1, 5));
}