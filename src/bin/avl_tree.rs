use std::{cmp::max, fmt::Debug};
use rand::thread_rng;
use rand::seq::SliceRandom;
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

struct AVLTree<T> where T: PartialOrd {
    root_node: Option<Box<Node<T>>>,
}

impl<T: MyTrait> AVLTree<T> {
    fn new() -> Self {
        Self {
            root_node: None,
        }
    }
}

#[derive(Clone)]
struct Node<T> {
    val: T,
    lt_node: Option<Box<Node<T>>>,
    rt_node: Option<Box<Node<T>>>,
    height: isize
}

impl<T: MyTrait> Node<T> {
    fn new(&val:&T) -> Self {
        Self {
            val,
            lt_node: None,
            rt_node: None,
            height: 1,
        }
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn get_lt_rt_heights(&self, root_node:&Option<Box<Node<T>>>) -> (isize, isize) {
        let mut lheight:isize = 0;
        let mut rheight:isize = 0;

        match root_node {
            Some(root) => {
                let lt_node = &root.lt_node;
                let rt_node = &root.rt_node;

                match lt_node {
                    Some(x) => {
                        lheight = x.height;
                    }
                    None => {}
                }

                match rt_node {
                    Some(x) => {
                        rheight = x.height;
                    }
                    None => {}
                }
            }
            None => {}
        }

        return (lheight, rheight);
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn height_diff(&self, root_node:&Option<Box<Node<T>>>) -> isize {
        let lr_heights = self.get_lt_rt_heights(&root_node);
        return lr_heights.0 - lr_heights.1;
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn set_height(&mut self, root_node:&mut Option<Box<Node<T>>>) {
        let lr_heights = self.get_lt_rt_heights(&root_node);

        match root_node {
            Some(root) => {
                root.height = 1 + max(lr_heights.0, lr_heights.1);
            }
            None => {}
        }
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn right_rotate(&mut self, root_node:&mut Option<Box<Node<T>>>) {
        match root_node {
            Some(node) => {
                let lt_node = &mut node.lt_node;
                let lt_node_clone = &mut lt_node.clone();

                match lt_node {
                    Some(x) => {
                        let lt_rt_node = &x.rt_node;
                        node.lt_node = lt_rt_node.clone();
                    }
                    None => {}
                }

                self.set_height(root_node);

                match lt_node_clone {
                    Some(x) => {
                        x.rt_node = root_node.clone();
                    }
                    None => {}
                }

                *root_node = lt_node_clone.clone();
                self.set_height(root_node);
            }
            None => {}
        }
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn left_rotate(&mut self, root_node:&mut Option<Box<Node<T>>>) {
        match root_node {
            Some(node) => {
                let rt_node = &mut node.rt_node;
                let rt_node_clone = &mut rt_node.clone();

                match rt_node {
                    Some(x) => {
                        let rt_lt_node = &x.lt_node;
                        node.rt_node = rt_lt_node.clone();
                    }
                    None => {}
                }

                self.set_height(root_node);

                match rt_node_clone {
                    Some(x) => {
                        x.lt_node = root_node.clone();
                    }
                    None => {}
                }

                *root_node = rt_node_clone.clone();
                self.set_height(root_node);
            }
            None => {}
        }
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn insert(&mut self, root_node:&mut Option<Box<Node<T>>>, &val:&T) {
        let mut hdiff:isize = 0;

        match root_node {
            Some(node) => {
                if val <= node.val {
                    let lt_node = &mut node.lt_node;
                    match lt_node {
                        Some(_x) => {
                            self.insert(lt_node, &val);
                        }
                        None => {
                            let new_node:Node<T> = Node::new(&val);
                            node.lt_node = Some(Box::new(new_node));
                        }
                    }
                }
                else {
                    let rt_node = &mut node.rt_node;
                    match rt_node {
                        Some(_x) => {
                            self.insert(rt_node, &val);
                        }
                        None => {
                            let new_node:Node<T> = Node::new(&val);
                            node.rt_node = Some(Box::new(new_node));
                        }
                    }
                }

                self.set_height(root_node);
                hdiff = self.height_diff(root_node);
            }
            None => {
                let new_node:Node<T> = Node::new(&val);
                *root_node = Some(Box::new(new_node));
            }
        }

        if hdiff > 1 {
            match root_node {
                Some(node) => {
                    let lt_node = &mut node.lt_node;
                    match lt_node {
                        Some(x) => {
                            if val <= x.val {
                                self.right_rotate(root_node);
                            }
                            else if val > x.val {
                                self.left_rotate(lt_node);
                                self.right_rotate(root_node);
                            }
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }

        else if hdiff < -1 {
            match root_node {
                Some(node) => {
                    let rt_node = &mut node.rt_node;
                    match rt_node {
                        Some(x) => {
                            if val <= x.val {
                                self.right_rotate(rt_node);
                                self.left_rotate(root_node);
                            }
                            else if val > x.val {
                                self.left_rotate(root_node);
                            }
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn print_tree(&self, root_node:&Option<Box<Node<T>>>, level:usize) {  
        match root_node {
            Some(node) => {
                println!("{} {:?}", "-".repeat(2*level), node.val);
                self.print_tree(&node.lt_node, level+1);
                self.print_tree(&node.rt_node, level+1);
            }
            None => {}
        }
    }
}

fn main() {
    let mut avltree:AVLTree<usize> = AVLTree::new();
    let mut root:Option<Box<Node<usize>>> = None;

    let mut vec: Vec<usize> = (1..1000000).collect();
    vec.shuffle(&mut thread_rng());

    for v in vec.iter() {
        avltree.insert(&mut root, &v);
        avltree.print_tree(&root, 0);
        println!();
    }
    avltree.root_node = root.clone();
}