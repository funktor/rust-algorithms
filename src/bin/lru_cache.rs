use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

pub trait HashKeyTrait: Eq + Hash + PartialOrd + Debug + Copy {}
pub trait MyTrait: PartialOrd + Debug + Copy {}

impl MyTrait for i8 {}
impl MyTrait for i16 {}
impl MyTrait for i32 {}
impl MyTrait for i64 {}
impl MyTrait for isize {}

impl MyTrait for u8 {}
impl MyTrait for u16 {}
impl MyTrait for u32 {}
impl MyTrait for u64 {}
impl MyTrait for usize {}

impl MyTrait for f32 {}
impl MyTrait for f64 {}

impl MyTrait for &str {}

impl HashKeyTrait for i8 {}
impl HashKeyTrait for i16 {}
impl HashKeyTrait for i32 {}
impl HashKeyTrait for i64 {}
impl HashKeyTrait for isize {}

impl HashKeyTrait for u8 {}
impl HashKeyTrait for u16 {}
impl HashKeyTrait for u32 {}
impl HashKeyTrait for u64 {}
impl HashKeyTrait for usize {}

impl HashKeyTrait for &str {}

struct LRUCache<S, T> where S: HashKeyTrait, T: MyTrait {
    max_size: usize,
    key_head: Option<S>,
    key_tail: Option<S>,
    node_map: HashMap<S, Node<S, T>>,
    curr_size: usize,
}

impl<S:HashKeyTrait, T: MyTrait> LRUCache<S, T> {
    fn new(len:usize) -> Self {
        Self {
            max_size: len,
            key_head: None,
            key_tail: None,
            node_map: HashMap::new(),
            curr_size: 0
        }
    }
}

struct Node<S, T> where S: HashKeyTrait, T: MyTrait {
    key: S,
    val: T,
    prev_key: Option<S>,
    next_key: Option<S>,
}

impl<S:HashKeyTrait, T: MyTrait> Node<S, T> {
    fn new(&key: &S, &val:&T) -> Self {
        Self {
            key,
            val,
            prev_key: None,
            next_key: None,
        }
    }
}

impl<S:HashKeyTrait, T: MyTrait> LRUCache<S, T> {
    fn add(&mut self, &key:&S,  &val:&T) {
        if self.curr_size == self.max_size {
            match self.key_head {
                Some(x) => {
                    self.delete(&x);
                }
                None => {}
            }
        }

        match self.key_tail {
            Some(x) => {
                let node_tail = self.node_map.get(&x);
                match node_tail {
                    Some(_y) => {
                        self.node_map.entry(x).and_modify(|mynode| mynode.next_key = Some(key));
                        
                        let new_node = Node {key, val, prev_key: Some(x), next_key:None};
                        self.node_map.insert(key, new_node);
                        self.curr_size += 1;
                        self.key_tail = Some(key);
                    }
                    None => {}
                }
            }
            None => {
                let new_node = Node::new(&key, &val);
                self.node_map.insert(key, new_node);
                self.curr_size += 1;

                self.key_head = Some(key);
                self.key_tail = Some(key);
            }
        }
    }
}

impl<S:HashKeyTrait, T: MyTrait> LRUCache<S, T> {
    fn get(&mut self, &key:&S) -> Option<T> {
        let output:Option<T>;

        match self.node_map.get(&key) {
            Some(x) => {
                output = Some(x.val);
            }
            None => {
                output = None;
            }
        }

        self.delete(&key);

        match output {
            Some(v) => {
                self.add(&key, &v);
            }
            None => {}
        }

        return output;
    }
}

impl<S:HashKeyTrait, T: MyTrait> LRUCache<S, T> {
    fn delete(&mut self, &key: &S) {
        match self.node_map.get(&key) {
            Some(node) => {
                match node.prev_key {
                    Some(x) => {
                        match node.next_key {
                            Some(y) => {
                                let next_node = self.node_map.get(&y);
                                match next_node {
                                    Some(_y1) => {
                                        self.node_map.entry(y).and_modify(|mynode| mynode.prev_key = Some(x));
                                    }
                                    None => {}
                                }

                                let prev_node = self.node_map.get(&x);
                                match prev_node {
                                    Some(_x1) => {
                                        self.node_map.entry(x).and_modify(|mynode| mynode.next_key = Some(y));
                                    }
                                    None => {}
                                }
                            }
                            None => {
                                let prev_node = self.node_map.entry(x);
                                prev_node.and_modify(|mynode| mynode.next_key = None);
                                
                                let prev_node = self.node_map.get(&x);
                                match prev_node {
                                    Some(x1) => {
                                        self.key_tail = Some(x1.key);
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                    None => {
                        match node.next_key {
                            Some(y) => {
                                let next_node = self.node_map.entry(y);
                                next_node.and_modify(|mynode| mynode.prev_key = None);
                                
                                let next_node = self.node_map.get(&y);
                                match next_node {
                                    Some(y1) => {
                                        self.key_head = Some(y1.key);
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }
                    }
                }

                self.node_map.remove(&key);
                self.curr_size -= 1;
                
            }
            None => {}
        }
    }
}

impl<S:HashKeyTrait, T: MyTrait> LRUCache<S, T> {
    fn update(&mut self, &key:&S, &val:&T) {
        self.delete(&key);
        self.add(&key, &val);
    }
}

impl<S:HashKeyTrait, T: MyTrait> LRUCache<S, T> {
    fn print_cache(&self, head:Option<&Node<S, T>>) {
        match head {
            Some(x) => {
                println!("{:?}, {:?}", x.key, x.val);
                match x.next_key {
                    Some(y) => {
                        self.print_cache(self.node_map.get(&y));
                    }
                    None => {}
                }
            }
            None => {
                match self.key_head {
                    Some(x) => {
                        self.print_cache(self.node_map.get(&x));
                    }
                    None => {}
                }
            }
        }
    }
}

fn main() {
    let mut cache:LRUCache<&str, i32> = LRUCache::new(3);
    cache.add(&"a", &1);
    cache.print_cache(None);
    println!();

    cache.add(&"b", &2);
    cache.print_cache(None);
    println!();

    cache.add(&"c", &3);
    cache.print_cache(None);
    println!();

    cache.update(&"a", &4);
    cache.print_cache(None);
    println!();

    cache.add(&"d", &4);
    cache.print_cache(None);
    println!();

    cache.add(&"e", &5);
    cache.print_cache(None);
    println!();

    cache.add(&"f", &6);
    cache.print_cache(None);
    println!();

    cache.update(&"e", &8);
    cache.print_cache(None);
    println!();

    match cache.get(&"f") {
        Some(x) => {
            println!("{:?}", x);
        }
        None => {
            println!("Non existing key!!!");
        }
        
    }
    cache.print_cache(None);
    println!();
}