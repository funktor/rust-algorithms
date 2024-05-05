use std::collections::{HashMap, HashSet};
use std::cmp::max;

#[derive(Clone)]
struct BlockBitVector {
    num_data: usize,
    num_blocks: usize,
    block_vector: Vec<usize>,
    block_size: usize,
}

impl BlockBitVector {
    fn new(num_data: usize) -> Self {
        let block_size: usize = 32;
        let num_blocks = num_data/block_size + 1;

        Self {
            num_data,
            num_blocks,
            block_vector: vec![0;num_blocks],
            block_size,
        }
    }
}

impl BlockBitVector {
    fn set_ith_bit(&mut self, i:usize) {
        if !self.check_ith_bit_set(i) {
            let block = (self.num_blocks*self.block_size - i - 1) / self.block_size;
            let block_pos = i % self.block_size;
            self.block_vector[block] = self.block_vector[block] | (1 << block_pos);
        }
    }
}

impl BlockBitVector {
    fn unset_ith_bit(&mut self, i:usize) {
        if self.check_ith_bit_set(i) {
            let block = (self.num_blocks*self.block_size - i - 1) / self.block_size;
            let block_pos = i % self.block_size;
            self.block_vector[block] = self.block_vector[block] & !(1 << block_pos);
        }
    }
}

impl BlockBitVector {
    fn check_ith_bit_set(&self, i: usize) -> bool {
        let block = (self.num_blocks*self.block_size - i - 1) / self.block_size;
        let block_pos = i % self.block_size;

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
        
        for (_i, c) in value.chars().enumerate() {
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
                    self.block_vector[block] = curr_val.parse::<usize>().ok().unwrap();
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
                out.push((self.num_blocks-i-1)*self.block_size + j);
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
    let mut c: BlockBitVector = BlockBitVector::new(a.num_data);

    for i in 0..a.num_blocks {
        c.block_vector[i] = a.block_vector[i] | b.block_vector[i];
    }

    return c;
}

fn do_bitwise_and(a: &BlockBitVector, b: &BlockBitVector) -> BlockBitVector {
    let mut c: BlockBitVector = BlockBitVector::new(a.num_data);

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

    // Store in cache only if number of families in fams in greater than existing for the same hw sku set
    if !cache.contains_key(&hws_val) || length > cache.get(&hws_val).unwrap().0 {
        cache.insert(hws_val, (length, fams.clone()));
    }

    // Recursively get all the (fams, hws) combinations
    for i in 0..n_families {
        let hw = family_to_hw[i].clone();

        let x = do_bitwise_and(&hws, &hw);
        let y = fams.check_ith_bit_set(i);

        if !x.is_zero() && !y {
            fams.set_ith_bit(i);
            let new_hws = do_bitwise_or(&hws, &hw);
            let new_hws_val = new_hws.ser();

            if !visited.contains(&fams.ser()) && (!cache.contains_key(&new_hws_val) || length+1 > cache.get(&new_hws_val).unwrap().0) {
                generate_constraints_dfs(fams, &new_hws, length+1, family_to_hw, n_families, visited, cache, n_hws);
            }

            fams.unset_ith_bit(i);
        }
    }
}

fn generate_constraints(family_to_hw: &Vec<BlockBitVector>, n_families:usize, n_hws: usize) -> Vec<(Vec<usize>, Vec<usize>)> {
    // Track which family set has been visited
    let mut visited:HashSet<String> = HashSet::new();
    
    // Track which hw sku set has been visited, store only maximum length family set corresponding to each hw sku set
    let mut cache:HashMap<String, (usize, BlockBitVector)> = HashMap::new();

    // Run depth first search starting from each family and store results in cache
    for i in 0..n_families {
        let mut fams = BlockBitVector::new(n_families);
        fams.set_ith_bit(i);
        generate_constraints_dfs(&mut fams, &family_to_hw[i], 1, family_to_hw, n_families, &mut visited, &mut cache, n_hws);
    }

    // Extract results from cache
    let mut out: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();

    for (key, value) in cache {
        let x = value.1.get_set_bits();

        let mut bbv = BlockBitVector::new(n_hws);
        bbv.deser(key);

        let y = bbv.get_set_bits();

        out.push((x, y));
    }

    return out;
}

fn create_hw_groups(family_to_hw: &Vec<BlockBitVector>, n_families:usize, n_hws: usize) -> (Vec<BlockBitVector>, Vec<BlockBitVector>) {
    let mut hw_map: HashMap<String, BlockBitVector> = HashMap::new();

    // Create map from hw sku bit representation to family bit representations
    for i in 0..n_families {
        let val = family_to_hw[i].ser();
        let mut x:BlockBitVector;

        if hw_map.contains_key(&val) {
            x = hw_map.get(&val).unwrap().clone();
        }
        else {
            x = BlockBitVector::new(n_families);
        }

        x.set_ith_bit(i);
        hw_map.insert(val, x.clone());
    }

    // Create updated merged family to hw sku bit representation
    let mut grouped_family_hw_map: Vec<BlockBitVector> = Vec::new();
    let mut grouped_families: Vec<BlockBitVector> = Vec::new();

    for (k, v) in hw_map {
        let mut bbv = BlockBitVector::new(n_hws);
        bbv.deser(k);

        grouped_family_hw_map.push(bbv);
        grouped_families.push(v);
    }

    return (grouped_family_hw_map, grouped_families);

}

fn family_hwsku_constraints(inp: &Vec<Vec<usize>>) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut n_families = inp.len();
    let mut n_hws: usize = 0;

    // Get number of hardware skus
    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            n_hws = max(n_hws, inp[i][j]+1);
        }
    }

    // Encode hardware skus into bit representations
    let mut bit_inp:Vec<BlockBitVector> = 
        vec![BlockBitVector::new(n_hws);n_families];

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            bit_inp[i].set_ith_bit(inp[i][j]);
        }
    }

    // Merge all families that runs on the same set of hardwares into a single group
    // 0 - updated family to list of hw skus with family = merged family
    // 1 - merged family to individual families
    let hw_grps = create_hw_groups(&bit_inp, n_families, n_hws);
    n_families = hw_grps.0.len();

    // Generate constraints, 1st term = list of merged families, 2nd term = list of hardware skus
    let out = generate_constraints(&hw_grps.0, n_families, n_hws);
    
    // Project back from merged families to individual families
    let mut result: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
    let grp_families = hw_grps.1;

    for x in out {
        let mut p: Vec<usize> = Vec::new();
        for y in x.0 {
            let mut f = grp_families[y].get_set_bits();
            p.append(&mut f);
        }
        result.push((p, x.1));
    }

    return result;
}

fn main() {
    // Hardware sku indices corresponding to a family i
    let inp:Vec<Vec<usize>> = 
        vec![
            vec![11, 12, 13, 14, 22, 23, 8, 9], 
            vec![12], 
            vec![12], 
            vec![0], 
            vec![14], 
            vec![3], 
            vec![23], 
            vec![11, 12, 13, 14, 22, 23, 8, 9], 
            vec![11, 12, 13, 14, 22, 23, 8, 9], 
            vec![2, 3, 14, 23], 
            vec![11, 12, 13, 14, 22, 23, 8, 9], 
            vec![0, 2, 3, 1, 13, 14, 11, 12, 22, 23, 21], 
            vec![2, 3], 
            vec![0, 2, 3], 
            vec![2, 3], 
            vec![0, 2, 3], 
            vec![6], 
            vec![13, 14, 22, 23], 
            vec![22, 23], 
            vec![13, 14, 22, 23], 
            vec![22, 23], 
            vec![1], 
            vec![2], 
            vec![3], 
            vec![10, 9], 
            vec![12], 
            vec![12], 
            vec![14], 
            vec![21], 
            vec![22], 
            vec![23], 
            vec![22, 23], 
            vec![24, 22, 23], 
            vec![4], 
            vec![4], 
            vec![4], 
            vec![4], 
            vec![11, 12, 13, 14, 22, 23, 8, 9], 
            vec![0, 2, 3, 1, 13, 14, 11, 12, 22, 23, 21], 
            vec![11, 12, 13, 14, 22, 23, 10, 9], 
            vec![11, 12, 13, 14, 22, 23, 10, 9], 
            vec![11, 12, 13, 14, 22, 23, 10, 9], 
            vec![13, 14, 24, 22, 23], 
            vec![24, 22, 23], 
            vec![11, 12, 13, 14, 22, 23, 8, 9], 
            vec![11, 12, 13, 14, 22, 23, 8, 9], 
            vec![11, 12, 13, 14, 22, 23, 8, 9], 
            vec![13, 14, 24, 22, 23], 
            vec![24, 22, 23], 
            vec![0, 2, 3, 1, 13, 14, 11, 12, 22, 23, 21], 
            vec![3, 1], 
            vec![0, 3, 1], 
            vec![3, 1], 
            vec![0, 3, 1], 
            vec![23, 21], 
            vec![24, 23, 21], 
            vec![14, 13, 23, 21], 
            vec![23, 21], 
            vec![14, 13, 23, 21], 
            vec![23, 21], 
            vec![3, 1], 
            vec![0, 3, 1], 
            vec![3, 1], 
            vec![14, 13, 23, 21], 
            vec![23, 21], 
            vec![23, 21], 
            vec![24, 23, 21], 
            vec![24, 23, 21], 
            vec![4], 
            vec![4], 
            vec![0, 2, 3, 1, 13, 14, 11, 12, 22, 23, 21], 
            vec![12, 14, 11, 13, 23, 21], 
            vec![14, 13, 24, 23, 21], 
            vec![24, 23, 21], 
            vec![12, 14, 11, 13, 23, 21], 
            vec![14, 13, 24, 23, 21], 
            vec![24, 23, 21], 
            vec![11, 12, 13, 14, 22, 23, 8, 9], 
            vec![0, 2, 3, 1, 13, 14, 11, 12, 22, 23, 21], 
            vec![11, 12, 13, 14, 22, 23, 10, 9], 
            vec![0, 2, 3, 1, 13, 14, 11, 12, 22, 23, 21], 
            vec![7, 13, 14, 22, 23], 
            vec![5], 
            vec![5], 
            vec![18], 
            vec![18], 
            vec![19], 
            vec![20], 
            vec![1], 
            vec![21], 
            vec![5], 
            vec![21], 
            vec![25], 
            vec![26, 27], 
            vec![27], 
            vec![27], 
            vec![27], 
            vec![28], 
            vec![28], 
            vec![29], 
            vec![30], 
            vec![15], 
            vec![31], 
            vec![16], 
            vec![17], 
            vec![17], 
            vec![32], 
            vec![32], 
            vec![33], 
            vec![33], 
            vec![34], 
            vec![12, 14], 
            vec![1, 21, 3, 23]
        ];

    let results = 
        family_hwsku_constraints(&inp);
    
    for (fams, hws) in results.iter() {
        println!("{:?}, {:?}", fams, hws);
    }
}