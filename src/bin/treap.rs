use std::{cmp::max, fmt::Debug};
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

struct Treap<T> where T: PartialOrd {
    root_node: Option<Box<Node<T>>>,
}

impl<T: MyTrait> Treap<T> {
    fn new() -> Self {
        Self {
            root_node: None,
        }
    }
}

#[derive(Clone)]
struct Node<T> {
    val: T,
    priority: usize,
    lt_node: Option<Box<Node<T>>>,
    rt_node: Option<Box<Node<T>>>,
    height: isize
}

impl<T: MyTrait> Node<T> {
    fn new(&val:&T, &priority:&usize) -> Self {
        Self {
            val,
            priority,
            lt_node: None,
            rt_node: None,
            height: 1,
        }
    }
}

impl<T: MyTrait> Treap<T> {
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

impl<T: MyTrait> Treap<T> {
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

impl<T: MyTrait> Treap<T> {
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

impl<T: MyTrait> Treap<T> {
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

impl<T: MyTrait> Treap<T> {
    fn insert(&mut self, root_node:&mut Option<Box<Node<T>>>, &val:&T, &priority:&usize) {
        match root_node {
            Some(node) => {
                if val <= node.val {
                    let lt_node = &mut node.lt_node;
                    match lt_node {
                        Some(_x) => {
                            self.insert(lt_node, &val, &priority);
                        }
                        None => {
                            let new_node:Node<T> = Node::new(&val, &priority);
                            node.lt_node = Some(Box::new(new_node));
                        }
                    }
                }
                else {
                    let rt_node = &mut node.rt_node;
                    match rt_node {
                        Some(_x) => {
                            self.insert(rt_node, &val, &priority);
                        }
                        None => {
                            let new_node:Node<T> = Node::new(&val, &priority);
                            node.rt_node = Some(Box::new(new_node));
                        }
                    }
                }

                self.set_height(root_node);
            }
            None => {
                let new_node:Node<T> = Node::new(&val, &priority);
                *root_node = Some(Box::new(new_node));
            }
        }

        let mut flag:u8 = 0;
        let mut hpriority:usize;

        match root_node {
            Some(node) => {
                hpriority = node.priority;
                let lt_node = &mut node.lt_node;
                let rt_node = &mut node.rt_node;

                match lt_node {
                    Some(x) => {
                        if x.priority > hpriority {
                            hpriority = x.priority;
                            flag = 1;
                        }
                    }
                    None => {}
                }

                match rt_node {
                    Some(x) => {
                        if x.priority > hpriority {
                            flag = 2;
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }

        if flag == 1 {
            match root_node {
                Some(node) => {
                    let lt_node = &mut node.lt_node;
                    match lt_node {
                        Some(_x) => {
                            self.right_rotate(root_node);
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }

        else if flag == 2 {
            match root_node {
                Some(node) => {
                    let rt_node = &mut node.rt_node;
                    match rt_node {
                        Some(_x) => {
                            self.left_rotate(root_node);
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
    }
}

impl<T: MyTrait> Treap<T> {
    fn delete(&mut self, root_node:&mut Option<Box<Node<T>>>, &val:&T) {
        let mut flag:u8 = 0;
        let mut hpriority:usize;

        match root_node {
            Some(node) => {
                if val == node.val {
                    node.priority = 0;

                    hpriority = node.priority;
                    let lt_node = &mut node.lt_node;
                    let rt_node = &mut node.rt_node;

                    match lt_node {
                        Some(x) => {
                            if x.priority > hpriority {
                                hpriority = x.priority;
                                flag = 1;
                            }
                        }
                        None => {}
                    }

                    match rt_node {
                        Some(x) => {
                            if x.priority > hpriority {
                                flag = 2;
                            }
                        }
                        None => {}
                    }

                    if flag == 0 {
                        *root_node = None;
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

        if flag == 1 {
            self.right_rotate(root_node);

            match root_node {
                Some(node) => {
                    self.delete(&mut node.rt_node, &val);
                }
                None => {}
            }
        }

        else if flag == 2 {
            self.left_rotate(root_node);

            match root_node {
                Some(node) => {
                    self.delete(&mut node.lt_node, &val);
                }
                None => {}
            }
        }
    }
}

impl<T: MyTrait> Treap<T> {
    fn print_tree(&self, root_node:&Option<Box<Node<T>>>, level:usize) {  
        match root_node {
            Some(node) => {
                println!("{} {:?},{:?},{:?}", "-".repeat(2*level), node.val, node.priority, node.height);
                self.print_tree(&node.lt_node, level+1);
                self.print_tree(&node.rt_node, level+1);
            }
            None => {
                println!("{} {:?}", "-".repeat(2*level), "NULL");
            }
        }
    }
}

fn main() {
    let mut treap:Treap<usize> = Treap::new();
    let mut root:Option<Box<Node<usize>>> = None;

    treap.insert(&mut root, &6, &100);
    treap.insert(&mut root, &15, &30);
    treap.insert(&mut root, &3, &70);
    treap.insert(&mut root, &1, &50);
    treap.insert(&mut root, &5, &60);
    treap.insert(&mut root, &11, &20);
    treap.insert(&mut root, &17, &25);

    treap.print_tree(&root, 0);
    println!();

    treap.insert(&mut root, &20, &120);
    treap.print_tree(&root, 0);
    println!();

    treap.delete(&mut root, &6);
    treap.print_tree(&root, 0);
    println!();

    treap.root_node = root.clone();
}