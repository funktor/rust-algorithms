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
    root_node: Option<Box<Node<T>>>,
}

impl<T: MyTrait> BST<T> {
    fn new() -> Self {
        Self {
            root_node: None,
        }
    }
}

#[derive(Clone)]
struct Node<T> where T: MyTrait {
    val: T,
    lt_node: Option<Box<Node<T>>>,
    rt_node: Option<Box<Node<T>>>,
}

impl<T: MyTrait> Node<T> {
    fn new(&val:&T) -> Self {
        Self {
            val,
            lt_node: None,
            rt_node: None,
        }
    }
}

impl<T: MyTrait> BST<T> {
    fn insert(&mut self, root_node:&mut Option<Box<Node<T>>>, &val:&T) {  
        if (self.root_node.is_none()) && (!root_node.is_none()) {
            self.root_node = root_node.clone();
        }

        let new_node = Node::new(&val);

        match root_node {
            Some(node) => {
                if val <= node.val {
                    if node.lt_node.is_none() {
                        node.lt_node = Some(Box::new(new_node));
                    }
                    else {
                        self.insert(&mut node.lt_node, &val);
                    }
                }
                else {
                    if node.rt_node.is_none() {
                        node.rt_node = Some(Box::new(new_node));
                    }
                    else {
                        self.insert(&mut node.rt_node, &val);
                    }
                }
            }
            None => {
                *root_node = Some(Box::new(new_node));
            }
        }
    }
}

impl<T: MyTrait> BST<T> {
    fn delete(&mut self, root_node:&mut Option<Box<Node<T>>>, &val:&T) {  
        match root_node {
            Some(ref mut node) => {
                if val == node.val {
                    if (node.lt_node.is_none()) && (node.rt_node.is_none()) {
                        *root_node = None;
                    }
                    else if node.lt_node.is_none() {
                        match &node.rt_node {
                            Some(x) => {
                                node.val = x.val;
                                node.rt_node = None;
                            }
                            None => {}
                        }
                    }
                    else if node.rt_node.is_none() {
                        match &node.lt_node {
                            Some(x) => {
                                node.val = x.val;
                                node.lt_node = None;
                            }
                            None => {}
                        }
                    }
                    else {
                        let mut new_node = &node.rt_node;
                        let mut has_successor:bool = false;

                        while !new_node.is_none() {
                            match new_node {
                                Some(x) => {
                                    has_successor = true;
                                    node.val = x.val;
                                    new_node = &x.lt_node;
                                }
                                None => {}
                            }
                        }

                        if has_successor {
                            self.delete(&mut node.rt_node, &node.val);
                        }
                    }
                }

                else if val < node.val {
                    self.delete(&mut node.lt_node, &val);
                }

                else {
                    self.delete(&mut node.rt_node, &val);
                }
            }
            None => {}
        }
    }
}

impl<T: MyTrait> BST<T> {
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
    let mut bst:BST<u32> = BST::new();
    let mut root:Option<Box<Node<u32>>> = bst.root_node.clone();

    bst.insert(&mut root, &10);
    bst.insert(&mut root, &20);
    bst.insert(&mut root, &5);
    bst.insert(&mut root, &9);
    bst.insert(&mut root, &2);
    bst.insert(&mut root, &15);
    bst.insert(&mut root, &30);
    bst.print_tree(&root, 0);
    println!();
    bst.delete(&mut root, &10);
    bst.print_tree(&root, 0);
}
 
