use std::usize::MAX;
use std::fmt::Debug;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

pub trait MyTrait: PartialOrd + Debug + Copy {}

impl MyTrait for i8 {}
impl MyTrait for i16 {}
impl MyTrait for i32 {}
impl MyTrait for i64 {}
impl MyTrait for i128 {}
impl MyTrait for isize {}

impl MyTrait for u8 {}
impl MyTrait for u16 {}
impl MyTrait for u32 {}
impl MyTrait for u64 {}
impl MyTrait for u128 {}
impl MyTrait for usize {}

impl MyTrait for f32 {}
impl MyTrait for f64 {}

impl MyTrait for &str {}

#[derive(Clone)]
struct WeightedSample<T> {
    samples: Vec<T>,
    cumulative_sums: Vec<f64>
}

impl<T: MyTrait> WeightedSample<T> {
    fn new(samples:Vec<T>, weights:Vec<f64>) -> Self {
        let n = samples.len();
        let mut cum_sum:Vec<f64> = vec![0.0;n];

        for i in 0..n {
            if i == 0 {
                cum_sum[i] = weights[i];
            }
            else {
                cum_sum[i] = cum_sum[i-1] + weights[i];
            }
        }

        Self {
            samples,
            cumulative_sums: cum_sum,
        }
    }
}

impl<T: MyTrait> WeightedSample<T> {
    fn sample(&self) -> T {
        let u = rand::thread_rng().gen_range(0.0..1.0);
        let mut index:usize = self.samples.len();

        let mut lt:usize = 0;
        let mut rt:usize = self.samples.len()-1;

        while lt <= rt {
            let mid = (lt + rt)/2;
            if self.cumulative_sums[mid] >= u {
                index = mid;
                if mid == 0 {
                    break;
                }
                rt = mid-1;
            }
            else{
                lt = mid+1;
            }
        }

        return self.samples[index];
    }
}

#[derive(Clone)]
struct Node<S> {
    id: usize,
    val: S,
    level: usize,
    next_pointers: Vec<usize>
}

impl<S: MyTrait> Node<S> {
    fn new(id:usize, &val:&S, level:usize) -> Self {
        Self {
            id,
            val,
            level,
            next_pointers: vec![MAX;30],
        }
    }
}

struct SkipList<S> {
    num_levels: usize,
    head_id: usize,
    sample_obj: WeightedSample<usize>,
    curr_id: usize,
    node_vec: Vec<Option<Node<S>>>,
}

impl<S: MyTrait> SkipList<S> {
    fn new(max_size:usize, default_value:S) -> Self {
        let mut samples:Vec<usize> = Vec::new();
        let mut weights:Vec<f64> = Vec::new();

        let num_levels:usize = 1 + f64::log2(max_size as f64) as usize;
        let mut wt:f64 = 1.0;

        for level in 1..num_levels+1 {
            samples.push(level);
            weights.push(wt);
            wt *= 0.5;
        }

        let mut wt_sum = 0.0;
        for wt in weights.iter() {
            wt_sum += *wt;
        }

        for wt in weights.iter_mut() {
            if wt_sum == 0.0 {
                *wt = 0.0;
            }
            else {
                *wt = *wt/wt_sum;
            }
        }

        let sample = WeightedSample::new(samples, weights);  
        let head_node = Node::new(0,&default_value, num_levels);  
        let mut node_vector:Vec<Option<Node<S>>> = vec![None;max_size];
        node_vector[0] = Some(head_node);

        Self {
            num_levels,
            head_id: 0,
            sample_obj: sample,
            curr_id: 0,
            node_vec: node_vector,
        }
    }
}

impl<S: MyTrait> SkipList<S> {
    fn insert(&mut self, val: S) {
        let level = self.sample_obj.sample();
        self.curr_id += 1;

        let mut new_node = Node::new(self.curr_id, &val, level);
        let mut prev_and_next_node_ids:Vec<(usize, usize)> = vec![(MAX, MAX);30];

        let mut curr_node_id:usize = 0;
        let mut curr_level = self.num_levels-1;

        loop {
            let curr_node = &self.node_vec[curr_node_id];

            match curr_node {
                Some(node) => {
                    let next_node_id = node.next_pointers[curr_level];

                    if next_node_id != MAX {
                        let next_node = &self.node_vec[next_node_id];
                        
                        match next_node {
                            Some(nxt) => {
                                if nxt.val >=  val {
                                    if curr_level < level {
                                        prev_and_next_node_ids[curr_level] = (curr_node_id, next_node_id);
                                    }
                                    
                                    if curr_level == 0 {
                                        break;
                                    }

                                    curr_level -= 1;
                                }
                                else {
                                    curr_node_id = next_node_id;
                                }
                            }
                            None => {}
                        }
                    }
                    else {
                        if curr_level < level {
                            prev_and_next_node_ids[curr_level] = (curr_node_id, MAX);
                        }
                                    
                        if curr_level == 0 {
                            break;
                        }

                        curr_level -= 1;
                    }
                }
                None => {}
            }
        }

        for i in 0..level {
            let p_n_id = prev_and_next_node_ids[i];

            let p_id = p_n_id.0;
            let n_id = p_n_id.1;

            if p_id < self.node_vec.len() {
                let prev_node = &mut self.node_vec[p_id];
                match prev_node {
                    Some(node) => {
                        node.next_pointers[i] = self.curr_id;
                    }
                    None => {}
                }
            }

            if n_id < self.node_vec.len() {
                new_node.next_pointers[i] = n_id;
            }
        }

        self.node_vec[self.curr_id] = Some(new_node);
    }
}

impl<S: MyTrait> SkipList<S> {
    fn search(&self, val: S) -> bool {
        let mut curr_node_id = self.head_id;
        let mut curr_level = self.num_levels-1;

        loop {
            let curr_node = &self.node_vec[curr_node_id];
            match curr_node {
                Some(node) => { 
                    let next_node_id = node.next_pointers[curr_level];

                    if next_node_id != MAX {
                        let next_node = &self.node_vec[next_node_id];

                        match next_node {
                            Some(nxt) => {
                                if nxt.val == val {
                                    return true;
                                }
                                else if nxt.val < val {
                                    curr_node_id = next_node_id;
                                }
                                else {
                                    if curr_level == 0 {
                                        return false;
                                    }
                                    curr_level -= 1;
                                }
                            }
                            None => {}
                        }
                    }
                    else {
                        if curr_level == 0 {
                            return false;
                        }
                        curr_level -= 1;
                    }
                }
                None => {}
            }
        }

    }
}


impl<S: MyTrait> SkipList<S> {
    fn delete(&mut self, val: S) {
        let mut curr_node_id = self.head_id;
        let mut curr_level = self.num_levels-1;

        let mut deleted_node_id = 0;
        let mut prev_and_next_node_ids:Vec<(usize, usize)> = vec![(MAX, MAX);self.num_levels];

        loop {
            let curr_node = &self.node_vec[curr_node_id];
            match curr_node {
                Some(node) => { 
                    let next_node_id = node.next_pointers[curr_level];

                    if next_node_id != MAX {
                        let next_node = &self.node_vec[next_node_id];

                        match next_node {
                            Some(nxt) => {
                                if nxt.val == val {
                                    deleted_node_id = next_node_id;
                                    let next_next_node_id = nxt.next_pointers[curr_level];

                                    if next_next_node_id != MAX {
                                        prev_and_next_node_ids[curr_level] = (curr_node_id, next_next_node_id);
                                    }
                                    else {
                                        prev_and_next_node_ids[curr_level] = (curr_node_id, MAX);
                                    }

                                    if curr_level == 0 {
                                        break;
                                    }

                                    curr_level -= 1;
                                }
                                else if nxt.val < val {
                                    curr_node_id = next_node_id;
                                }
                                else {
                                    if curr_level == 0 {
                                        break;
                                    }
                                    curr_level -= 1;
                                }
                            }
                            None => {}
                        }
                    }
                    else {
                        if curr_level == 0 {
                            break;
                        }
                        curr_level -= 1;
                    }
                }
                None => {}
            }
        }

        for i in 0..self.num_levels {
            let p_n_id = prev_and_next_node_ids[i];

            let p_id = p_n_id.0;
            let n_id = p_n_id.1;

            let n = self.node_vec.len();

            if (p_id < n) && (n_id < n) {
                let prev_node = &mut self.node_vec[p_id];

                match prev_node {
                    Some(node) => {
                        node.next_pointers[i] = n_id;
                    }
                    None => {}
                }
            }

            else if p_id < n {
                let prev_node = &mut self.node_vec[p_id];
                match prev_node {
                    Some(node) => {
                        let n = node.next_pointers.len();
                        node.next_pointers[n-1] = MAX;
                    }
                    None => {}
                }
            }
        }

        if deleted_node_id != 0 {
            self.node_vec[deleted_node_id] = None;
        }
        
    }
}


impl<S: MyTrait> SkipList<S> {
    fn print_sl(&self) {
        for i in (0..self.num_levels).rev() {
            let mut curr_node_id = self.head_id;

            loop {
                let curr_node = &self.node_vec[curr_node_id];
                match curr_node {
                    Some(node) => {
                        print!("{:?} -> ", node.val);
                        curr_node_id = node.next_pointers[i];
                        if curr_node_id == MAX {
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }

            println!();
        }
    }
}

fn main() {
    let mut skip_list:SkipList<usize> = SkipList::new(32, 0);
    let mut vec: Vec<usize> = (1..32).collect();
    vec.shuffle(&mut thread_rng());

    for v in vec.iter() {
        skip_list.insert(*v);
    }
    skip_list.print_sl();
    println!();

    skip_list.search(10);
    skip_list.delete(12);

    skip_list.print_sl();
}