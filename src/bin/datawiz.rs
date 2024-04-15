use std::time::SystemTime;

pub mod data_type {
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
}

pub mod data_object {
    use crate::data_type::DataWizDataTypes;
    use crate::indexer::SkipList;

    #[derive(Debug, Clone)]
    pub struct DataFrame {
        data: Vec<Vec<DataWizDataTypes>>,
        colnames: Vec<String>,
        dtypes: Vec<String>,
        indexes: Vec<SkipList<DataWizDataTypes>>,
    }

    impl DataFrame {
        fn new(data:Vec<Vec<DataWizDataTypes>>) -> Self {
            Self {
                data,
                colnames: Vec::new(),
                dtypes: Vec::new(),
                indexes: Vec::new(),
            }
        }
    }
}


pub mod indexer {
    use std::fmt::Debug;
    use std::usize::MAX;
    use rand::Rng;
    use crate::data_type::DataWizDataTypes;

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

    #[derive(Debug, Clone)]
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

    #[derive(Debug, Clone)]
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

    #[derive(Debug, Clone)]
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
        pub fn insert_initial(&mut self, vector: &Vec<S>) {
            let mut prev_node_id_at_levels:Vec<usize> = vec![self.head_id;self.num_levels];

            for i in 0..vector.len() {
                let level = self.sample_obj.sample();
                self.curr_id += 1;

                let new_node = Node::new(self.curr_id, &vector[i], i, level);
                                
                for l in 0..self.num_levels {
                    if l <= level {
                        let node_id = prev_node_id_at_levels[l];
                        let node = self.node_vec[node_id].as_mut().unwrap();
                        
                        if l < node.next_pointers.len() {
                            node.next_pointers[l] = self.curr_id;
                        }
                        else {
                            node.next_pointers.push(self.curr_id);
                        }
                        
                        prev_node_id_at_levels[l] = self.curr_id;
                    }
                }

                self.node_vec.push(Some(new_node));
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
}


pub mod my_reader {
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

pub mod data_processor {
    use crate::data_type::DataWizDataTypes;
    use crate::indexer::SkipList;
    use crate::data_object::DataFrame;
    use std::fs::File;
    use std::vec;
    use std::path::Path;
    use std::fs;
    use std::io;
    use std::io::prelude::*;

    fn parse_line(line:&str, columnar_data:&mut Vec<String>, row_num:usize, num_cols: usize) -> usize {
        let mut curr:String = String::from("");
        let mut quotes:usize = 0;
        let mut index:usize = 0;
    
        for c in line.chars() {
            if c == '"' {
                quotes = (quotes + 1) % 2;
                curr.clear();
            }
    
            else if c == ',' {
                if quotes == 0 {
                    columnar_data[row_num*num_cols + index] = curr.clone();
                    index += 1;
                }
                curr.clear();
            }
            else {
                curr.push(c);
            }
        }
    
        if curr.len() > 0 {
            columnar_data[row_num*num_cols + index] = curr.clone();
            index += 1;
        }

        if num_cols == 0 || index == num_cols {
            return index;
        }

        return 0;
    }
    
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    
    pub fn read_file(file_path: String, newline: String, header:bool) -> (Vec<String>, Vec<String>, usize, usize) {
        let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
        let lines = contents.split(&newline);
        let lines_vec = lines.collect::<Vec<&str>>();
        let num_rows = lines_vec.len()-1;
        let mut num_cols:usize = 0;

        let mut columnar_data:Vec<String> = vec![String::from("");100];
        let mut colnames:Vec<String> = vec![String::from("");100];

        let mut i:usize = 0;

        for line in lines_vec {
            let curr_line = line.trim();

            if curr_line.len() > 0 {
                let ncols = parse_line(curr_line, &mut columnar_data, i, num_cols);
                if ncols == 0 {
                    println!("Misformatted CSV File !!!");
                    return (Vec::new(), Vec::new(), 0, 0);
                }
                if i == 0 {
                    num_cols = ncols;

                    columnar_data.resize(num_rows*num_cols, String::from(""));
                    colnames.resize(num_cols, String::from(""));

                    if header {
                        for j in 0..num_cols {
                            colnames[j] = columnar_data[j].clone();
                        }
                    }
                    else {
                        for j in 0..num_cols {
                            colnames[j] = j.to_string();
                        }
                    }
                }
            }

            i += 1;
        }

        println!("{:?}, {:?}", num_rows, num_cols);
    
        return (columnar_data, colnames, num_rows, num_cols);
    }
    
    pub fn infer_data_types(vector:&mut Vec<String>, num_rows:usize, num_cols:usize, header:bool) -> Vec<String> {
        let mut data_types:Vec<String> = vec![String::from("String");num_cols];
        
        let mut prefs1:Vec<usize> = vec![0;num_cols];
        let mut prefs2:Vec<usize> = vec![0;num_cols];
        let mut prefs3:Vec<usize> = vec![0;num_cols];

        let mut start:usize = 0;
        if header {
            start = 1;
        }

        for j in start..num_rows {
            for i in 0..num_cols {
                let index = j*num_cols + i;

                if vector[index].parse::<f64>().is_ok() && (prefs1[i] == 0 || prefs1[i] == 1) {
                    prefs1[i] = 1;
                }
                else if vector[index].parse::<bool>().is_ok() && (prefs1[i] == 0 || prefs1[i] == 2) {
                    prefs1[i] = 2;
                }
                else {
                    prefs1[i] = 3;
                }
            }
        }

        for i in 0..num_cols {
            if prefs1[i] == 2 {
                data_types[i] = String::from("bool");
            }
            else if prefs1[i] == 3 {
                data_types[i] = String::from("String");
            }
        }

        let mut down_cast:Vec<bool> = vec![false;num_cols];

        for j in start..num_rows {
            for i in 0..num_cols {
                let index = j*num_cols + i;

                if prefs1[i] == 1 {
                    if vector[index].parse::<usize>().is_ok() && prefs2[i] <= 1 {
                        prefs2[i] = 1;
                    }
                    else if vector[index].parse::<isize>().is_ok() && prefs2[i] <= 2 {
                        prefs2[i] = 2;
                    }
                    else {
                        let f = vector[index].parse::<f64>().ok().unwrap();
                        let u = f as usize;
                        let v = f as isize;
                        
                        if f - (u as f64) == 0.0 && prefs2[i] <= 1 {
                            prefs2[i] = 1;
                            down_cast[i] = true;
                        }
                        else if f - (v as f64) == 0.0 && prefs2[i] <= 2 {
                            prefs2[i] = 2;
                            down_cast[i] = true;
                        }
                        else {
                            prefs2[i] = 3;
                        }
                    }
                }
            }
        }

        for j in start..num_rows {
            for i in 0..num_cols {
                let index = j*num_cols + i;

                if prefs2[i] == 3 {
                    if vector[index].parse::<f32>().is_ok() && prefs3[i] <= 1 {
                        prefs3[i] = 1;
                    }
                    else {
                        prefs3[i] = 2;
                    }
                }
            }
        }

        for i in 0..num_cols {
            if prefs2[i] == 3 {
                if prefs3[i] == 2 {
                    data_types[i] = String::from("f64");
                }
                else {
                    data_types[i] = String::from("f32");
                }
            }
        }


        for j in start..num_rows {
            for i in 0..num_cols {
                let index = j*num_cols + i;
                if prefs2[i] == 1 {
                    if down_cast[i] {
                        vector[index] = (vector[index].parse::<f64>().ok().unwrap() as usize).to_string();
                    }
                    
                    if vector[index].parse::<u8>().is_ok() && prefs3[i] <= 1 {
                        prefs3[i] = 1;
                    }
                    else if vector[index].parse::<u16>().is_ok() && prefs3[i] <= 2 {
                        prefs3[i] = 2;
                    }
                    else if vector[index].parse::<u32>().is_ok() && prefs3[i] <= 3 {
                        prefs3[i] = 3;
                    }
                    else if vector[index].parse::<u64>().is_ok() && prefs3[i] <= 4 {
                        prefs3[i] = 4;
                    }
                    else {
                        prefs3[i] = 5;
                    }
                }
            }
        }

        for i in 0..num_cols {
            if prefs2[i] == 1 {
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
        }

        for j in start..num_rows {
            for i in 0..num_cols {
                let index = j*num_cols + i;

                if prefs2[i] == 2 {
                    if down_cast[i] {
                        vector[index] = (vector[index].parse::<f64>().ok().unwrap() as isize).to_string();
                    }
                    
                    if vector[index].parse::<i8>().is_ok() && prefs3[i] <= 1 {
                        prefs3[i] = 1;
                    }
                    else if vector[index].parse::<i16>().is_ok() && prefs3[i] <= 2 {
                        prefs3[i] = 2;
                    }
                    else if vector[index].parse::<i32>().is_ok() && prefs3[i] <= 3 {
                        prefs3[i] = 3;
                    }
                    else if vector[index].parse::<i64>().is_ok() && prefs3[i] <= 4 {
                        prefs3[i] = 4;
                    }
                    else {
                        prefs3[i] = 5;
                    }
                }
            }
        }

        for i in 0..num_cols {
            if prefs2[i] == 2 {
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
    
        return data_types;
    }
    
    pub fn update_data_type(vector:&mut Vec<String>, dtypes: &Vec<String>, num_rows:usize, num_cols:usize, header:bool) -> Vec<Option<DataWizDataTypes>> {
        let mut new_data:Vec<Option<DataWizDataTypes>> = vec![None;num_rows*num_cols];
        
        let mut start:usize = 0;
        if header {
            start = 1;
        }

        for j in start..num_rows {
            for i in 0..num_cols {
                let index = j*num_cols + i;

                if dtypes[i] == "u8" {
                    if vector[index].parse::<u8>().is_ok() {
                        let val = vector[index].parse::<u8>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::U8(val));
                    }
                }

                else if dtypes[i] == "u16" {
                    if vector[index].parse::<u16>().is_ok() {
                        let val = vector[index].parse::<u16>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::U16(val));
                    }
                }
                else if dtypes[i] == "u32" {
                    if vector[index].parse::<u32>().is_ok() {
                        let val = vector[index].parse::<u32>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::U32(val));
                    }
                }
                else if dtypes[i] == "u64" {
                    if vector[index].parse::<u64>().is_ok() {
                        let val = vector[index].parse::<u64>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::U64(val));
                    }
                }
                else if dtypes[i] == "u128" {
                    if vector[index].parse::<u128>().is_ok() {
                        let val = vector[index].parse::<u128>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::U128(val));
                    }
                }
                else if dtypes[i] == "i8" {
                    if vector[index].parse::<i8>().is_ok() {
                        let val = vector[index].parse::<i8>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::I8(val));
                    }
                }
                else if dtypes[i] == "i16" {
                    if vector[index].parse::<i16>().is_ok() {
                        let val = vector[index].parse::<i16>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::I16(val));
                    }
                }
                else if dtypes[i] == "i32" {
                    if vector[index].parse::<i32>().is_ok() {
                        let val = vector[index].parse::<i32>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::I32(val));
                    }
                }
                else if dtypes[i] == "i64" {
                    if vector[index].parse::<i64>().is_ok() {
                        let val = vector[index].parse::<i64>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::I64(val));
                    }
                }
                else if dtypes[i] == "i128" {
                    if vector[index].parse::<i128>().is_ok() {
                        let val = vector[index].parse::<i128>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::I128(val));
                    }
                }
                else if dtypes[i] == "f32" {
                    if vector[index].parse::<f32>().is_ok() {
                        let val = vector[index].parse::<f32>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::F32(val));
                    }
                }
                else if dtypes[i] == "f64" {
                    if vector[index].parse::<f64>().is_ok() {
                        let val = vector[index].parse::<f64>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::F64(val));
                    }
                }
                else if dtypes[i] == "bool" {
                    if vector[index].parse::<bool>().is_ok() {
                        let val = vector[index].parse::<bool>().ok().unwrap();
                        new_data[index] = Some(DataWizDataTypes::Bool(val));
                    }
                }
                else {
                    new_data[index] = Some(DataWizDataTypes::Text(vector[index].clone()));
                }
            }
        }
    
        return new_data;
    }
    
    pub fn create_index(data:&Vec<Option<DataWizDataTypes>>, dtypes: &Vec<String>, num_rows:usize, num_cols:usize, header:bool) -> Vec<SkipList<DataWizDataTypes>> {
        let mut index:Vec<SkipList<DataWizDataTypes>> = Vec::new();
        
        let mut start:usize = 0;
        if header {
            start = 1;
        }

        for i in 0..num_cols {
            let mut sl:SkipList<DataWizDataTypes>;
            let n = num_rows;
    
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
    
            // for j in start..n {
            //     let index = j*num_cols + i;
            //     sl.insert(data[index].as_ref().unwrap(), j);
            // }
    
            index.push(sl);
        }

        let mut transpose:Vec<DataWizDataTypes> = vec![DataWizDataTypes::Text(String::from(""));num_rows*num_cols];
        
        for i in start..num_rows {
            for j in 0..num_cols {
                transpose[j*num_rows + i] = data[i*num_cols + j].clone().unwrap();
            }
        }

        for i in 0..num_cols {
            let mut vector:Vec<DataWizDataTypes> = vec![DataWizDataTypes::Text(String::from(""));num_rows];
            for j in 0..num_rows {
                vector[j] = transpose[i*num_rows + j].clone();
            }

            vector.sort_by(|a, b| a.partial_cmp(b).unwrap());
            index[i].insert_initial(&vector);
        }
    
        return index;
    }

}

fn main() {
    use crate::data_processor::*;
    use crate::data_type::DataWizDataTypes;

    let path: String = String::from("/mnt/c/Users/amondal/Downloads/products.csv");
    let start_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    let mut data = read_file(path, String::from("\n"), true);
    let dtypes = infer_data_types(&mut data.0, data.2, data.3, true);
    let new_data = update_data_type(&mut data.0, &dtypes, data.2, data.3, true);
    let index = create_index(&new_data, &dtypes, data.2, data.3, true);
    let end_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    println!("{:?}", end_time-start_time);
    println!("{:?}", data.1);
    println!("{:?}", dtypes);
    // match &mut data {
    //     Some(x) => {
    //         // println!("{:?}", x);
    //         let colnames = get_column_names(x);
    //         println!("{:?}", colnames);
    //         // println!("{:?}", x);
    //         let dtypes = infer_data_types(x);
    //         println!("{:?}", dtypes);

    //         let new_data = update_data_type(x, &dtypes);
    //         // println!("{:?}", new_data);
    //         x.clear();

    //         let index = create_index(&new_data, &dtypes);
    //         println!();
    //         println!();

    //         let start_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    //         let filtered = index[29].search_gte(DataWizDataTypes::I16(64));
    //         let end_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    //         println!();
    //         println!("{:?}", filtered.len());
    //         println!("{:?}", end_time-start_time);
    //     }
    //     None => {}
    // }
}