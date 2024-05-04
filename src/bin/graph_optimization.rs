use std::{collections::{HashMap, HashSet, VecDeque}, default, process::Output};

#[derive(Clone)]
struct BlockBitVector {
    num_blocks: usize,
    block_vector: Vec<u32>,
}

impl BlockBitVector {
    fn new(num_blocks: usize) -> Self {
        Self {
            num_blocks,
            block_vector: vec![0;num_blocks],
        }
    }
}

impl BlockBitVector {
    fn set_ith_bit(&mut self, i:usize) {
        if !self.check_ith_bit_set(i) {
            let block = (self.num_blocks*32 - i - 1) / 32;
            let block_pos = i % 32;
            self.block_vector[block] = self.block_vector[block] | (1 << block_pos);
        }
    }
}

impl BlockBitVector {
    fn unset_ith_bit(&mut self, i:usize) {
        if self.check_ith_bit_set(i) {
            let block = (self.num_blocks*32 - i - 1) / 32;
            let block_pos = i % 32;
            self.block_vector[block] = self.block_vector[block] & !(1 << block_pos);
        }
    }
}

impl BlockBitVector {
    fn check_ith_bit_set(&self, i: usize) -> bool {
        let block = (self.num_blocks*32 - i - 1) / 32;
        let block_pos = i % 32;

        let x = self.block_vector[block] & (1 << block_pos);
        return x != 0;
    }
}

impl BlockBitVector {
    fn ser(&self) -> String {
        let mut out:String = String::from("");
        
        for i in 0..self.num_blocks {
            let x = self.block_vector[i].to_string();
            let y = x.len();
            if y < 10 {
                out.push_str("0");
                out.push_str(&y.to_string());
            }
            else {
                out.push_str(&y.to_string());
            }
            out.push_str(&x);
        }
        
        return out;
    }
}

impl BlockBitVector {
    fn deser(&mut self, value:String) {
        let mut is_len:u8 = 2;
        let mut is_val:u8 = 0;
        let mut curr_len:String = String::from("");
        let mut curr_val:String = String::from("");
        let mut block:usize = 0;
        
        for (i, c) in value.chars().enumerate() {
            if is_len > 0 {
                if is_len < 2 || c != '0' {
                    curr_len.push(c);
                }
                is_len -= 1;
            }
            else {
                if is_val == 0 {
                    is_val = curr_len.parse::<u8>().ok().unwrap();
                    curr_len.clear();
                }
                
                curr_val.push(c);
                is_val -= 1;
                
                if is_val == 0 {
                    is_len = 2;
                    self.block_vector[block] = curr_val.parse::<u32>().ok().unwrap();
                    curr_val.clear();
                    block += 1;
                }
            }
        }
    }
}

impl BlockBitVector {
    fn get_set_bits(&self) -> Vec<usize> {
        let mut out: Vec<usize> = Vec::new();
        
        for i in 0..self.num_blocks {
            let x = get_set_bits(self.block_vector[i] as usize);
            for j in x {
                out.push((self.num_blocks-i-1)*32 + j);
            }
        }

        return out;
    }
}

impl BlockBitVector {
    fn is_zero(&self) -> bool {
        return self.get_set_bits().len() == 0;
    }
}

fn do_bitwise_or(a: &BlockBitVector, b: &BlockBitVector) -> BlockBitVector {
    let mut c: BlockBitVector = BlockBitVector::new(a.num_blocks);

    for i in 0..a.num_blocks {
        c.block_vector[i] = a.block_vector[i] | b.block_vector[i];
    }

    return c;
}

fn do_bitwise_and(a: &BlockBitVector, b: &BlockBitVector) -> BlockBitVector {
    let mut c: BlockBitVector = BlockBitVector::new(a.num_blocks);

    for i in 0..a.num_blocks {
        c.block_vector[i] = a.block_vector[i] & b.block_vector[i];
    }

    return c;
}

fn get_set_bits(mut num:usize) -> Vec<usize> {
    let mut positions:Vec<usize> = Vec::new();
    let mut i:usize = 0;

    while num > 0 {
        let m = num % 2;
        if m == 1 {
            positions.push(i);
        }
        num = num/2;
        i += 1;
    }

    return positions
}

fn generate_constraints_dfs(fams: &mut BlockBitVector, hws: &BlockBitVector, length: usize, family_to_hw: &Vec<BlockBitVector>, n_families: usize, visited: &mut HashSet<String>, cache: &mut HashMap<String, (usize, BlockBitVector)>, n_hws: usize) {
    visited.insert(fams.ser());
    let hws_val = hws.ser();

    if !cache.contains_key(&hws_val) || length > cache.get(&hws_val).unwrap().0 {
        cache.insert(hws_val, (length, fams.clone()));
    }

    for i in 0..n_families {
        let hw = family_to_hw[i].clone();

        let x = do_bitwise_and(&hws, &hw);
        let y = fams.check_ith_bit_set(i);

        if !x.is_zero() && !y {
            fams.set_ith_bit(i);
            let new_hws = do_bitwise_or(&hws, &hw);
            if !visited.contains(&fams.ser()) {
                generate_constraints_dfs(fams, &new_hws, length+1, family_to_hw, n_families, visited, cache, n_hws);
            }
            fams.unset_ith_bit(i);
        }
    }
}

fn generate_constraints_2(family_to_hw: &Vec<BlockBitVector>, n_hws: usize) -> Vec<(Vec<usize>, Vec<usize>)> {
    let n_families = family_to_hw.len();

    let mut visited:HashSet<String> = HashSet::new();
    let mut cache:HashMap<String, (usize, BlockBitVector)> = HashMap::new();

    for i in 0..n_families {
        let mut fams = BlockBitVector::new(n_families/32 + 1);
        fams.set_ith_bit(i);
        generate_constraints_dfs(&mut fams, &family_to_hw[i], 1, family_to_hw, n_families, &mut visited, &mut cache, n_hws);
    }

    let mut out: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();

    for (key, value) in cache {
        let x = value.1.get_set_bits();

        let mut bbv = BlockBitVector::new(n_hws/32 + 1);
        bbv.deser(key);

        let y = bbv.get_set_bits();

        out.push((x, y));
    }

    return out;
}

fn generate_constraints(family_to_hw:&Vec<usize>) -> Vec<(Vec<usize>, Vec<usize>)> {
    let n_families = family_to_hw.len();

    let mut queue:VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited:HashSet<usize> = HashSet::new();
    let mut output:Vec<(usize, usize)> = Vec::new();

    for i in 0..n_families {
        queue.push_back((0 | (1 << i), family_to_hw[i]));
        visited.insert(0 | (1 << i));
    }

    while queue.len() > 0 {
        let q = queue.pop_front().unwrap();
        output.push(q);

        let fams = q.0;
        let hws = q.1;

        for i in 0..n_families {
            let hw = family_to_hw[i];
            let new_fams = fams | (1 << i);
            let new_hws = hws | hw;

            if (hws & hw) != 0 && (fams & (1 << i)) == 0 && !visited.contains(&new_fams) {
                queue.push_back((new_fams, new_hws));
                visited.insert(new_fams);
            }
        }
    }

    let mut out: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();

    for i in 0..output.len() {
        let q = output[i];
        let x = get_set_bits(q.0);
        let y = get_set_bits(q.1);
        out.push((x, y));
    }

    return out;

}

fn create_hw_groups(family_to_hw: &Vec<BlockBitVector>, n_hws: usize) -> (Vec<BlockBitVector>, Vec<BlockBitVector>) {
    let n_families = family_to_hw.len();
    let mut hw_map: HashMap<String, BlockBitVector> = HashMap::new();

    for i in 0..n_families {
        let val = family_to_hw[i].ser();
        let mut x:BlockBitVector;

        if hw_map.contains_key(&val) {
            x = hw_map.get(&val).unwrap().clone();
        }
        else {
            x = BlockBitVector::new(n_families/32 + 1);
        }

        x.set_ith_bit(i);
        hw_map.insert(val, x.clone());
    }

    let mut grouped_family_hw_map: Vec<BlockBitVector> = Vec::new();
    let mut grouped_families: Vec<BlockBitVector> = Vec::new();

    for (k, v) in hw_map {
        let mut bbv = BlockBitVector::new(n_hws/32 + 1);
        bbv.deser(k);

        grouped_family_hw_map.push(bbv);
        grouped_families.push(v);
    }

    return (grouped_family_hw_map, grouped_families);
}


fn main() {
    let inp:Vec<Vec<usize>> = vec![vec![0], vec![0,1,2], vec![1,4], vec![2,3], vec![3,4,5,6], vec![5], vec![6]];
    let mut bit_inp:Vec<BlockBitVector> = vec![BlockBitVector::new(1);inp.len()];

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            bit_inp[i].set_ith_bit(inp[i][j]);
        }
    }

    // let mut h = create_hw_groups(&bit_inp, 7);
    let out = generate_constraints_2(&bit_inp, 7);
    for i in 0..out.len() {
        println!("{:?}, {:?}", out[i].0, out[i].1);
    }
    // let grp_families = h.1;

    // for x in out {
    //     let mut p: Vec<usize> = Vec::new();
    //     for y in x.0 {
    //         let mut f = grp_families[y].get_set_bits();
    //         p.append(&mut f);
    //     }
    //     println!("{:?}, {:?}", p, x.1);
    // }
}