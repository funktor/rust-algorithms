use std::collections::HashMap;
use std::fmt::Debug;

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

struct BST<T> where T: MyTrait {
    auto_inc_id: usize,
    id_to_node_map: HashMap<usize, Node<T>>,
}

impl<T: MyTrait> BST<T> {
    fn new() -> Self {
        Self {
            auto_inc_id: 0,
            id_to_node_map: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy)]
struct Node<T> where T: MyTrait {
    val: T,
    lt_node_id: Option<usize>,
    rt_node_id: Option<usize>,
}

impl<T: MyTrait> Node<T> {
    fn new(&val:&T) -> Self {
        Self {
            val,
            lt_node_id: None,
            rt_node_id: None,
        }
    }
}

impl<T: MyTrait> BST<T> {
    fn insert(&mut self, root_node_id:Option<usize>, &val:&T) -> Option<usize> {  
        match root_node_id {
            Some(node_id) => {
                let node = self.id_to_node_map.get(&node_id);
                match node {
                    Some(node_obj) => {
                        if val <= node_obj.val {
                            let lt_node_id = self.insert(node_obj.lt_node_id, &val);
                            self.id_to_node_map.entry(node_id).and_modify(|mynode| mynode.lt_node_id = lt_node_id);
                        }
                        else {
                            let rt_node_id = self.insert(node_obj.rt_node_id, &val);
                            self.id_to_node_map.entry(node_id).and_modify(|mynode| mynode.rt_node_id = rt_node_id);
                        }
                    }
                    None => {}
                }
            }
            None => {
                self.auto_inc_id += 1;
                let node = Node::new(&val);
                self.id_to_node_map.insert(self.auto_inc_id, node);
                return Some(self.auto_inc_id);
            }
        }
        
        return root_node_id;
    }
}

impl<T: MyTrait> BST<T> {
    fn delete(&mut self, root_node_id:Option<usize>, &val:&T) -> Option<usize> {  
        match root_node_id {
            Some(node_id) => {
                let mut node = self.id_to_node_map.get(&node_id);
                match &mut node {
                    Some(node_obj) => {
                        if val == node_obj.val {
                            let mut new_val:T = node_obj.val;
                            let mut new_lt_node_id: Option<usize> = node_obj.lt_node_id;
                            let mut new_rt_node_id: Option<usize> = node_obj.rt_node_id;

                            if (node_obj.lt_node_id.is_none()) && (node_obj.rt_node_id.is_none()) {
                                return None;
                            }

                            else if node_obj.lt_node_id.is_none() {
                                match node_obj.rt_node_id {
                                    Some(rt_node_id) => {
                                        let rt_node = self.id_to_node_map.get(&rt_node_id);
                                        match rt_node {
                                            Some(rt_node_obj) => {
                                                new_val = rt_node_obj.val;
                                                new_rt_node_id = None;
                                            }
                                            None => {}
                                        }
                                    }
                                    None => {}
                                }
                            }
                            else if node_obj.rt_node_id.is_none() {
                                match node_obj.lt_node_id {
                                    Some(lt_node_id) => {
                                        let lt_node = self.id_to_node_map.get(&lt_node_id);
                                        match lt_node {
                                            Some(lt_node_obj) => {
                                                new_val = lt_node_obj.val;
                                                new_lt_node_id = None;
                                            }
                                            None => {}
                                        }
                                    }
                                    None => {}
                                }
                            }
                            else {
                                let mut succ_node_id = node_obj.rt_node_id;
                                let mut has_successor:bool = false;
        
                                while !succ_node_id.is_none() {
                                    match succ_node_id {
                                        Some(succ_node_id_val) => {
                                            let succ_node = self.id_to_node_map.get(&succ_node_id_val);
                                            match succ_node {
                                                Some(succ_node_obj) => {
                                                    has_successor = true;
                                                    new_val = succ_node_obj.val;
                                                    succ_node_id = succ_node_obj.lt_node_id;
                                                }
                                                None => {}
                                            }                          
                                        }
                                        None => {}
                                    }
                                }
        
                                if has_successor {
                                    let rt_node_id = self.delete(node_obj.rt_node_id, &new_val);
                                    self.id_to_node_map.entry(node_id).and_modify(|mynode| mynode.rt_node_id = rt_node_id);
                                }                
                            }

                            self.id_to_node_map.entry(node_id).and_modify(|mynode| {
                                mynode.val = new_val;
                                mynode.lt_node_id = new_lt_node_id;
                                mynode.rt_node_id = new_rt_node_id;
                            });

                        }
                        else if val < node_obj.val {
                            let lt_node_id = self.delete(node_obj.lt_node_id, &val);
                            self.id_to_node_map.entry(node_id).and_modify(|mynode| mynode.lt_node_id = lt_node_id);
                        }
        
                        else {
                            let rt_node_id = self.delete(node_obj.rt_node_id, &val);
                            self.id_to_node_map.entry(node_id).and_modify(|mynode| mynode.rt_node_id = rt_node_id);
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }

        return root_node_id;
    }
}

impl<T: MyTrait> BST<T> {
    fn print_tree(&self, root_node_id:Option<usize>, level:usize) {  
        match root_node_id {
            Some(node_id) => {
                let node = self.id_to_node_map.get(&node_id);
                match node {
                    Some(node_obj) => {
                        println!("{} {:?}", "-".repeat(2*level), node_obj.val);
                        self.print_tree(node_obj.lt_node_id, level+1);
                        self.print_tree(node_obj.rt_node_id, level+1);
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}

fn main() {
    let mut bst:BST<u32> = BST::new();
    let mut root:Option<usize> = None;

    root = bst.insert(root, &10);
    root = bst.insert(root, &20);
    root = bst.insert(root, &5);
    root = bst.insert(root, &9);
    root = bst.insert(root, &2);
    root = bst.insert(root, &15);
    root = bst.insert(root, &30);
    bst.print_tree(root, 0);
    println!();
    root = bst.delete(root, &10);
    bst.print_tree(root, 0);
}
 
