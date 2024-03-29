use std::{cmp::{max, min}, usize::{MAX, MIN}};

#[derive(Clone)]
struct Veb {
    size: usize,
    sq: usize,
    val: Option<Vec<u8>>,
    cluster: Vec<Veb>,
    summary: Option<Box<Veb>>,
    min: usize,
    max: usize,
}

impl Veb {
    fn new(size:usize) -> Self {
        let sq = (size as f32).sqrt().round() as usize;
        Self {
            size,
            sq,
            val: None,
            cluster: Vec::new(),
            summary: None,
            min: MAX,
            max: MIN,
        }
    }
}

impl Veb {
    fn initialize(&mut self) {
        if self.size == 1 {
            self.val = Some(vec![0]);
        }
        else if self.size == 2 {
            self.val = Some(vec![0;2]);
        }
        else {
            let mut cluster:Veb;

            while (self.cluster.len() + 1)*self.sq < self.size {
                cluster = Veb::new(self.sq);
                cluster.initialize();
                self.cluster.push(cluster);
            }

            if self.cluster.len()*self.sq < self.size {
                cluster = Veb::new(self.size - self.cluster.len()*self.sq);
                cluster.initialize();
                self.cluster.push(cluster);
            }
    
            let mut summary = Veb::new(self.cluster.len());
            summary.initialize();
            self.summary = Some(Box::new(summary));
        }
    }
}

impl Veb {
    fn insert(&mut self, val: usize) {
        if self.size == 1 {
            self.min = 0;
            self.max = 0;
            self.val = Some(vec![1]);
        }

        else if self.size == 2 {
            if self.val.is_none() {
                self.val = Some(vec![0;2]);
            }

            match &mut self.val {
                Some(v) => {
                    if val == 0 {
                        v[0] = 1;
                    }

                    else if val == 1 {
                        v[1] = 1;
                    }

                    if v[0] == 1 {
                        self.min = min(self.min, 0);
                        self.max = max(self.max, 0);
                    }

                    if v[1] == 1 {
                        self.min = min(self.min, 1);
                        self.max = max(self.max, 1);
                    }
                }
                None => {}
            }
        }

        else {
            let i = (val/self.sq) as usize;
            let j = val % self.sq as usize;

            let cluster = &mut self.cluster[i];
            let prev_min = cluster.min;

            cluster.insert(j);

            let a = i*self.sq + cluster.min;
            let b = i*self.sq + cluster.max;

            self.min = min(self.min, a);
            self.max = max(self.max, b);

            if prev_min == MAX {
                let summary = &mut self.summary;
                match summary {
                    Some(sm) => {
                        sm.insert(i);
                    }
                    None => {}
                }
            }
        }
    }
}

impl Veb {
    fn successor(&self, val: usize) -> Option<usize> {
        if self.size == 1 {
            return None;
        }

        else if self.size == 2 {
            if (val == 0) && (self.max == 1) {
                return Some(1);
            }
            return None;
        }

        else {
            let i = (val/self.sq) as usize;
            let j = val % self.sq as usize;

            let cluster = &self.cluster[i];

            if cluster.max > j {
                let out = cluster.successor(j);
                match out {
                    Some(x) => {
                        return Some(i*self.sq + x);
                    }
                    None => {}
                }
            }

            let summary = &self.summary;

            match summary {
                Some(y) => {
                    let k = y.successor(i);
                    match k {
                        Some(z) => {
                            let succ_cluster = &self.cluster[z];
                            return Some(z*self.sq + succ_cluster.min);
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }

        return None;
    }
}


impl Veb {
    fn predecessor(&self, val: usize) -> Option<usize> {
        if self.size == 1 {
            return None;
        }

        else if self.size == 2 {
            if (val == 1) && (self.min == 0) {
                return Some(0);
            }
            return None;
        }

        else {
            let i = (val/self.sq) as usize;
            let j = val % self.sq as usize;

            let cluster = &self.cluster[i];

            if cluster.min < j {
                let out = cluster.predecessor(j);
                match out {
                    Some(x) => {
                        return Some(i*self.sq + x);
                    }
                    None => {}
                }
            }

            let summary = &self.summary;

            match summary {
                Some(y) => {
                    let k = y.predecessor(i);
                    match k {
                        Some(z) => {
                            let pred_cluster = &self.cluster[z];
                            return Some(z*self.sq + pred_cluster.max);
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }

        return None;
    }
}

impl Veb {
    fn printveb(&self) {
        if self.size == 1 {
            match &self.val {
                Some(x) => {
                    print!("{:?} ", x[0]);
                }
                None => {}
            }
        }
        else if self.size == 2 {
            match &self.val {
                Some(x) => {
                    print!("{:?} {:?} ", x[0], x[1]);
                }
                None => {}
            }
        }

        else {
            for i in 0..self.cluster.len() {
                self.cluster[i].printveb();
            }
        }
    }
    
}
fn main() {
    let mut veb = Veb::new(1000000);
    veb.initialize();
    veb.insert(2);
    veb.insert(5);
    veb.insert(7);

    veb.printveb();
    println!();
    match &veb.summary {
        Some(x) => {
            x.printveb();
        }
        None => {}
    }
    println!();

    println!("{:?}", veb.successor(3));
    println!("{:?}", veb.predecessor(8));
}