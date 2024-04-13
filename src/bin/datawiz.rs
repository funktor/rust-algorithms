use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;
use std::vec;
use std::{cmp::max, fmt::Debug};
use std::usize::MAX;
use rand::Rng;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataWizDataTypes {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
    Bool(bool),
    Text(String)
}

pub trait MyTrait: PartialOrd + Debug + Clone {}

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

impl MyTrait for String {}
impl MyTrait for DataWizDataTypes {}

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

        return self.samples[index].clone();
    }
}

#[derive(Clone)]
struct Node<S> {
    id: usize,
    val: S,
    index: usize,
    level: usize,
    next_pointers: Vec<usize>
}

impl<S: MyTrait> Node<S> {
    fn new(id:usize, val:&S, index:usize, level:usize) -> Self {
        Self {
            id,
            val:val.clone(),
            index,
            level,
            next_pointers: Vec::new(),
        }
    }
}

pub struct SkipList<S> {
    num_levels: usize,
    head_id: usize,
    sample_obj: WeightedSample<usize>,
    curr_id: usize,
    node_vec: Vec<Option<Node<S>>>,
}

impl<S: MyTrait> SkipList<S> {
    pub fn new(max_size:usize, default_value:S) -> Self {
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
        let head_node = Node::new(0, &default_value, MAX, num_levels);  
        let mut node_vector:Vec<Option<Node<S>>> = Vec::new();
        node_vector.push(Some(head_node));

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
    pub fn insert(&mut self, val: &S, index:usize) {
        let level = self.sample_obj.sample();
        self.curr_id += 1;

        let mut new_node = Node::new(self.curr_id, val, index, level);
        let mut prev_and_next_node_ids:Vec<(usize, usize)> = Vec::new();

        for _i in 0..level {
            prev_and_next_node_ids.push((MAX, MAX));
        }

        let mut curr_node_id:usize = self.head_id;
        let mut curr_level = self.num_levels-1;

        loop {
            if curr_node_id >= self.node_vec.len() {
                break;
            }

            let curr_node = self.node_vec[curr_node_id].as_ref().unwrap();

            if curr_level < curr_node.next_pointers.len() {
                let next_node_id = curr_node.next_pointers[curr_level];
                let next_node = self.node_vec[next_node_id].as_ref().unwrap();

                if next_node.val >=  val.clone() {
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

        for i in 0..level {
            let p_n_id = prev_and_next_node_ids[i];

            let p_id = p_n_id.0;
            let n_id = p_n_id.1;

            if p_id < self.node_vec.len() {
                let prev_node = self.node_vec[p_id].as_mut().unwrap();
                if i < prev_node.next_pointers.len() {
                    prev_node.next_pointers[i] = self.curr_id;
                }
                else {
                    prev_node.next_pointers.push(self.curr_id);
                }
                
            }

            if n_id < self.node_vec.len() {
                if i < new_node.next_pointers.len() {
                    new_node.next_pointers[i] = n_id;
                }
                else {
                    new_node.next_pointers.push(n_id);
                }
            }
        }

        self.node_vec.push(Some(new_node));
    }
}

impl<S: MyTrait> SkipList<S> {
    pub fn search(&self, val: S) -> Vec<usize> {
        let mut curr_node_id = self.head_id;
        let mut curr_level = self.num_levels-1;

        let mut start:Option<Node<S>> = None;

        loop {
            let curr_node = self.node_vec[curr_node_id].as_ref().unwrap();

            if curr_level < curr_node.next_pointers.len() {
                let next_node_id = curr_node.next_pointers[curr_level];
                let next_node = self.node_vec[next_node_id].as_ref().unwrap();

                if next_node.val == val {
                    start = Some(next_node.clone());
                    if curr_level == 0 {
                        break;
                    }
                    curr_level -= 1;
                }
                else {
                    curr_node_id = next_node_id;
                }
            }
            else {
                if curr_level == 0 {
                    break;
                }
                curr_level -= 1;
            }
        }

        let mut output:Vec<usize> = Vec::new();
        
        if !start.is_none() {
            let mut curr_node = start.as_ref().unwrap();
            loop {
                if curr_node.val > val {
                    break;
                }

                output.push(curr_node.index);
                
                if curr_node.next_pointers.len() > 0 {
                    let next_node_id = curr_node.next_pointers[0];
                    curr_node = self.node_vec[next_node_id].as_ref().unwrap();
                }
                else {
                    break;
                }
            }
        }

        return output;
    }
}

impl<S: MyTrait> SkipList<S> {
    pub fn search_lt(&self, val: S) -> Vec<usize> {
        let mut output:Vec<usize> = Vec::new();

        if self.head_id+1 < self.node_vec.len() {
            let mut start_node = self.node_vec[self.head_id+1].as_ref().unwrap();
            loop {
                if start_node.val >= val {
                    break;
                }
                output.push(start_node.index);

                if start_node.next_pointers.len() > 0 {
                    let next_node_id = start_node.next_pointers[0];
                    start_node = self.node_vec[next_node_id].as_ref().unwrap();
                }
                else {
                    break;
                }
            }
        }

        return output;
    }
}

impl<S: MyTrait> SkipList<S> {
    pub fn search_lte(&self, val: S) -> Vec<usize> {
        let mut output:Vec<usize> = Vec::new();

        if self.head_id+1 < self.node_vec.len() {
            let mut start_node = self.node_vec[self.head_id+1].as_ref().unwrap();
            loop {
                if start_node.val > val {
                    break;
                }
                output.push(start_node.index);

                if start_node.next_pointers.len() > 0 {
                    let next_node_id = start_node.next_pointers[0];
                    start_node = self.node_vec[next_node_id].as_ref().unwrap();
                }
                else {
                    break;
                }
            }
        }

        return output;
    }
}

impl<S: MyTrait> SkipList<S> {
    pub fn search_gt(&self, val: S) -> Vec<usize> {
        let mut curr_node_id = self.head_id;
        let mut curr_level = self.num_levels-1;

        let mut start:Option<Node<S>> = None;

        loop {
            let curr_node = self.node_vec[curr_node_id].as_ref().unwrap();

            if curr_level < curr_node.next_pointers.len() {
                let next_node_id = curr_node.next_pointers[curr_level];
                let next_node = self.node_vec[next_node_id].as_ref().unwrap();

                if next_node.val > val {
                    start = Some(next_node.clone());
                    if curr_level == 0 {
                        break;
                    }
                    curr_level -= 1;
                }
                else {
                    curr_node_id = next_node_id;
                }
            }
            else {
                if curr_level == 0 {
                    break;
                }
                curr_level -= 1;
            }
        }

        let mut output:Vec<usize> = Vec::new();
        
        if !start.is_none() {
            let mut curr_node = start.as_ref().unwrap();
            loop {
                output.push(curr_node.index);
                
                if curr_node.next_pointers.len() > 0 {
                    let next_node_id = curr_node.next_pointers[0];
                    curr_node = self.node_vec[next_node_id].as_ref().unwrap();
                }
                else {
                    break;
                }
            }
        }

        return output;
    }
}

impl<S: MyTrait> SkipList<S> {
    pub fn search_gte(&self, val: S) -> Vec<usize> {
        let mut curr_node_id = self.head_id;
        let mut curr_level = self.num_levels-1;

        let mut start:Option<Node<S>> = None;

        loop {
            let curr_node = self.node_vec[curr_node_id].as_ref().unwrap();

            if curr_level < curr_node.next_pointers.len() {
                let next_node_id = curr_node.next_pointers[curr_level];
                let next_node = self.node_vec[next_node_id].as_ref().unwrap();

                if next_node.val >= val {
                    start = Some(next_node.clone());
                    if curr_level == 0 {
                        break;
                    }
                    curr_level -= 1;
                }
                else {
                    curr_node_id = next_node_id;
                }
            }
            else {
                if curr_level == 0 {
                    break;
                }
                curr_level -= 1;
            }
        }

        let mut output:Vec<usize> = Vec::new();
        
        if !start.is_none() {
            let mut curr_node = start.as_ref().unwrap();
            loop {
                output.push(curr_node.index);
                
                if curr_node.next_pointers.len() > 0 {
                    let next_node_id = curr_node.next_pointers[0];
                    curr_node = self.node_vec[next_node_id].as_ref().unwrap();
                }
                else {
                    break;
                }
            }
        }
        
        return output;
    }
}

impl<S: MyTrait> SkipList<S> {
    pub fn delete(&mut self, val: S) {
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
    pub fn print_sl(&self) {
        for i in (0..self.num_levels).rev() {
            let mut curr_node_id = self.head_id;

            loop {
                let curr_node = &self.node_vec[curr_node_id];
                match curr_node {
                    Some(node) => {
                        print!("{:?} -> ", node.val);
                        if i < node.next_pointers.len() {
                            curr_node_id = node.next_pointers[i];
                        }
                        else {
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

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(&mut self, buffer: &'buf mut String) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}

fn parse_line(line: String, vector:&mut Vec<Vec<String>>, num_cols: usize) -> bool {
    let mut curr:String = String::from("");
    let mut quotes:usize = 0;
    let mut index:usize = 0;

    for (_i, c) in line.chars().enumerate() {
        if c == '"' {
            quotes = (quotes + 1) % 2;
            curr.clear();
        }

        else if c == ',' {
            if quotes == 0 {
                if num_cols == 0 {
                    vector.push(vec![curr.clone()]);
                    index += 1;
                }

                else if index < num_cols {
                    vector[index].push(curr.clone());
                    index += 1;
                }

                else {
                    return false;
                }
            }
            curr.clear();
        }
        else {
            curr.push(c);
        }
    }

    if curr.len() > 0 {
        if num_cols == 0 {
            vector.push(vec![curr.clone()]);
            index += 1;
        }

        else if index < num_cols {
            vector[index].push(curr.clone());
            index += 1;
        }

        else {
            return false;
        }
    }

    return true;
}

fn read_file(file_path: String) -> Option<Vec<Vec<String>>> {
    let mut columnar_data:Vec<Vec<String>> = Vec::new();
    let mut reader = my_reader::BufReader::open(file_path).unwrap();
    let mut buffer = String::new();
    let mut num_cols:usize = 0;

    while let Some(line) = reader.read_line(&mut buffer) {
        let curr_line = line.unwrap().trim();
        let res = parse_line(curr_line.to_string(), &mut columnar_data, num_cols);
        if !res {
            println!("Misformatted file !!!");
            return None;
        }

        if num_cols == 0 {
            num_cols = columnar_data.len();
        }
    }

    return Some(columnar_data);
}

fn get_column_names(vector:&mut Vec<Vec<String>>) -> Vec<String> {
    let mut colnames:Vec<String> = vec![String::from("");vector.len()];

    for i in 0..vector.len() {
        colnames[i] = vector[i][0].clone();
        vector[i].drain(0..1);
    }

    return colnames;
}

fn infer_data_types(vector:&mut Vec<Vec<String>>) -> Vec<String> {
    let mut data_types:Vec<String> = vec![String::from("String");vector.len()];
    
    let mut prefs1:Vec<usize> = vec![0;vector.len()];
    let mut prefs2:Vec<usize> = vec![0;vector.len()];
    let mut prefs3:Vec<usize> = vec![0;vector.len()];

    for i in 0..vector.len() {
        let mut down_cast:bool = false;
        
        for j in 0..vector[i].len() {
            if vector[i][j].parse::<f64>().is_ok() && (prefs1[i] == 0 || prefs1[i] == 1) {
                prefs1[i] = 1;
            }
            else if vector[i][j].parse::<bool>().is_ok() && (prefs1[i] == 0 || prefs1[i] == 2) {
                prefs1[i] = 2;
            }
            else {
                prefs1[i] = 3;
                data_types[i] = String::from("String");
                break;
            }
        }
        
        if prefs1[i] == 2 {
            data_types[i] = String::from("bool");
        }
        
        else if prefs1[i] == 1 {
            for j in 0..vector[i].len() {
                if vector[i][j].parse::<usize>().is_ok() && prefs2[i] <= 1 {
                    prefs2[i] = 1;
                }
                else if vector[i][j].parse::<isize>().is_ok() && prefs2[i] <= 2 {
                    prefs2[i] = 2;
                }
                else {
                    let f = vector[i][j].parse::<f64>().ok().unwrap();
                    let u = f as usize;
                    let v = f as isize;
                    
                    if f - (u as f64) == 0.0 && prefs2[i] <= 1 {
                        prefs2[i] = 1;
                        down_cast = true;
                    }
                    else if f - (v as f64) == 0.0 && prefs2[i] <= 2 {
                        prefs2[i] = 2;
                        down_cast = true;
                    }
                    else {
                        prefs2[i] = 3;
                        break;
                    }
                }
            }
            
            if prefs2[i] == 3 {
                for j in 0..vector[i].len() {
                    if vector[i][j].parse::<f32>().is_ok() && prefs3[i] <= 1 {
                        prefs3[i] = 1;
                    }
                    else {
                        prefs3[i] = 2;
                        break;
                    }
                }
                
                if prefs3[i] == 2 {
                    data_types[i] = String::from("f64");
                }
                else {
                    data_types[i] = String::from("f32");
                }
            }
            
            else if prefs2[i] == 1 {
                for j in 0..vector[i].len() {
                    if down_cast {
                        vector[i][j] = (vector[i][j].parse::<f64>().ok().unwrap() as usize).to_string();
                    }
                    
                    if vector[i][j].parse::<u8>().is_ok() && prefs3[i] <= 1 {
                        prefs3[i] = 1;
                    }
                    else if vector[i][j].parse::<u16>().is_ok() && prefs3[i] <= 2 {
                        prefs3[i] = 2;
                    }
                    else if vector[i][j].parse::<u32>().is_ok() && prefs3[i] <= 3 {
                        prefs3[i] = 3;
                    }
                    else if vector[i][j].parse::<u64>().is_ok() && prefs3[i] <= 4 {
                        prefs3[i] = 4;
                    }
                    else {
                        prefs3[i] = 5;
                        break;
                    }
                }
                
                if prefs3[i] == 1 {
                    data_types[i] = String::from("u8");
                }
                else if prefs3[i] == 2 {
                    data_types[i] = String::from("u16");
                }
                else if prefs3[i] == 3 {
                    data_types[i] = String::from("u32");
                }
                else if prefs3[i] == 4 {
                    data_types[i] = String::from("u64");
                }
                else {
                    data_types[i] = String::from("u128");
                }
            }
            
            else {
                for j in 0..vector[i].len() {
                    if down_cast {
                        vector[i][j] = (vector[i][j].parse::<f64>().ok().unwrap() as isize).to_string();
                    }
                    
                    if vector[i][j].parse::<i8>().is_ok() && prefs3[i] <= 1 {
                        prefs3[i] = 1;
                    }
                    else if vector[i][j].parse::<i16>().is_ok() && prefs3[i] <= 2 {
                        prefs3[i] = 2;
                    }
                    else if vector[i][j].parse::<i32>().is_ok() && prefs3[i] <= 3 {
                        prefs3[i] = 3;
                    }
                    else if vector[i][j].parse::<i64>().is_ok() && prefs3[i] <= 4 {
                        prefs3[i] = 4;
                    }
                    else {
                        prefs3[i] = 5;
                        break;
                    }
                }
                
                if prefs3[i] == 1 {
                    data_types[i] = String::from("i8");
                }
                else if prefs3[i] == 2 {
                    data_types[i] = String::from("i16");
                }
                else if prefs3[i] == 3 {
                    data_types[i] = String::from("i32");
                }
                else if prefs3[i] == 4 {
                    data_types[i] = String::from("i64");
                }
                else {
                    data_types[i] = String::from("i128");
                }
            }
        }
    } 

    return data_types;
}

fn update_data_type(vector:&mut Vec<Vec<String>>, dtypes: &Vec<String>) -> Vec<Vec<Option<DataWizDataTypes>>> {
    let mut new_data:Vec<Vec<Option<DataWizDataTypes>>> = Vec::new();

    for i in 0..vector.len() {
        let mut coldata:Vec<Option<DataWizDataTypes>> = Vec::new();
        for j in 0..vector[i].len() {
            if dtypes[i] == "u8" {
                if vector[i][j].parse::<u8>().is_ok() {
                    let val = vector[i][j].parse::<u8>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::U8(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else if dtypes[i] == "u16" {
                if vector[i][j].parse::<u16>().is_ok() {
                    let val = vector[i][j].parse::<u16>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::U16(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else if dtypes[i] == "u32" {
                if vector[i][j].parse::<u32>().is_ok() {
                    let val = vector[i][j].parse::<u32>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::U32(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else if dtypes[i] == "u64" {
                if vector[i][j].parse::<u64>().is_ok() {
                    let val = vector[i][j].parse::<u64>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::U64(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else if dtypes[i] == "u128" {
                if vector[i][j].parse::<u128>().is_ok() {
                    let val = vector[i][j].parse::<u128>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::U128(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else if dtypes[i] == "i8" {
                if vector[i][j].parse::<i8>().is_ok() {
                    let val = vector[i][j].parse::<i8>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::I8(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else if dtypes[i] == "i16" {
                if vector[i][j].parse::<i16>().is_ok() {
                    let val = vector[i][j].parse::<i16>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::I16(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else if dtypes[i] == "i32" {
                if vector[i][j].parse::<i32>().is_ok() {
                    let val = vector[i][j].parse::<i32>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::I32(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else if dtypes[i] == "i64" {
                if vector[i][j].parse::<i64>().is_ok() {
                    let val = vector[i][j].parse::<i64>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::I64(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else if dtypes[i] == "i128" {
                if vector[i][j].parse::<i128>().is_ok() {
                    let val = vector[i][j].parse::<i128>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::I128(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else if dtypes[i] == "f32" {
                if vector[i][j].parse::<f32>().is_ok() {
                    let val = vector[i][j].parse::<f32>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::F32(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else if dtypes[i] == "f64" {
                if vector[i][j].parse::<f64>().is_ok() {
                    let val = vector[i][j].parse::<f64>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::F64(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else if dtypes[i] == "bool" {
                if vector[i][j].parse::<bool>().is_ok() {
                    let val = vector[i][j].parse::<bool>().ok().unwrap();
                    coldata.push(Some(DataWizDataTypes::Bool(val)));
                }
                else {
                    coldata.push(None);
                }
            }
            else {
                coldata.push(Some(DataWizDataTypes::Text(vector[i][j].clone())));
            }
        }
        new_data.push(coldata);
    }

    return new_data;
}

fn create_index(data:&Vec<Vec<Option<DataWizDataTypes>>>, dtypes: &Vec<String>) -> Vec<SkipList<DataWizDataTypes>> {
    let mut index:Vec<SkipList<DataWizDataTypes>> = Vec::new();

    for i in 0..data.len() {
        let mut sl:SkipList<DataWizDataTypes>;
        let n = data[i].len();

        if dtypes[i] == "u8" {
            sl = SkipList::new(n, DataWizDataTypes::U8(std::u8::MAX));
        }
        else if dtypes[i] == "u16" {
            sl = SkipList::new(n, DataWizDataTypes::U16(std::u16::MAX));
        }
        else if dtypes[i] == "u32" {
            sl = SkipList::new(n, DataWizDataTypes::U32(std::u32::MAX));
        }
        else if dtypes[i] == "u64" {
            sl = SkipList::new(n, DataWizDataTypes::U64(std::u64::MAX));
        }
        else if dtypes[i] == "u128" {
            sl = SkipList::new(n, DataWizDataTypes::U128(std::u128::MAX));
        }
        else if dtypes[i] == "i8" {
            sl = SkipList::new(n, DataWizDataTypes::I8(std::i8::MAX));
        }
        else if dtypes[i] == "i16" {
            sl = SkipList::new(n, DataWizDataTypes::I16(std::i16::MAX));
        }
        else if dtypes[i] == "i32" {
            sl = SkipList::new(n, DataWizDataTypes::I32(std::i32::MAX));
        }
        else if dtypes[i] == "i64" {
            sl = SkipList::new(n, DataWizDataTypes::I64(std::i64::MAX));
        }
        else if dtypes[i] == "f32" {
            sl = SkipList::new(n, DataWizDataTypes::F32(std::f32::MAX));
        }
        else if dtypes[i] == "f64" {
            sl = SkipList::new(n, DataWizDataTypes::F64(std::f64::MAX));
        }
        else {
            sl = SkipList::new(n, DataWizDataTypes::Text(String::from("")));
        }

        for j in 0..n {
            sl.insert(data[i][j].as_ref().unwrap(), j);
        }

        index.push(sl);
    }

    return index;
}

fn main() {
    let path: String = String::from("/mnt/c/Users/amondal/Downloads/price_params_violations.csv");
    let mut data = read_file(path);
    match &mut data {
        Some(x) => {
            println!("{:?}", x);
            let colnames = get_column_names(x);
            println!("{:?}", colnames);
            println!("{:?}", x);
            let dtypes = infer_data_types(x);
            println!("{:?}", dtypes);

            let new_data = update_data_type(x, &dtypes);
            println!("{:?}", new_data);
            x.clear();

            let index = create_index(&new_data, &dtypes);
            println!();
            println!();
            index[11].print_sl();

            let filtered = index[0].search(DataWizDataTypes::Text(String::from("za west|ddv4|linux")));
            println!();
            println!("{:?}", filtered.len());
        }
        None => {}
    }
}