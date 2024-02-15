use std::{cmp::max, fmt::Debug, rc::Rc, cell::{RefCell, RefMut}};
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
    root_node: Option<Rc<RefCell<Node<T>>>>,
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
    lt_node: Option<Rc<RefCell<Node<T>>>>,
    rt_node: Option<Rc<RefCell<Node<T>>>>,
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
    fn get_lt_rt_heights(&self, root_borrow:&RefMut<'_, Node<T>>) -> (isize, isize) {
        let mut lheight:isize = 0;
        let mut rheight:isize = 0;

        let lt_node = &root_borrow.lt_node;
        let rt_node = &root_borrow.rt_node;

        match lt_node {
            Some(x) => {
                lheight = x.borrow().height;
            }
            None => {}
        }

        match rt_node {
            Some(x) => {
                rheight = x.borrow().height;
            }
            None => {}
        }

        return (lheight, rheight);
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn height_diff(&self, root_borrow:&RefMut<'_, Node<T>>) -> isize {
        let lr_heights = self.get_lt_rt_heights(&root_borrow);
        return lr_heights.0 - lr_heights.1;
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn set_height(&self, root_borrow:&mut RefMut<'_, Node<T>>) {
        let lr_heights = self.get_lt_rt_heights(&root_borrow);
        root_borrow.height = 1 + max(lr_heights.0, lr_heights.1);
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn right_rotate(&mut self, root_borrow:&mut RefMut<'_, Node<T>>)->Option<Rc<RefCell<Node<T>>>> {
        let root_clone = root_borrow.clone();
        let lt_node = &root_clone.lt_node;

        match lt_node {
            Some(x) => {
                let lt_rt_node = &x.borrow().rt_node;
                match lt_rt_node {
                    Some(y) => {
                        root_borrow.lt_node = Some(y.clone());
                    }
                    None => {
                        root_borrow.lt_node = None;
                    }
                }
            }
            None => {}
        }

        self.set_height(root_borrow);

        match lt_node {
            Some(ref x) => {
                let xborrow = &mut x.borrow_mut();
                xborrow.rt_node = Some(Rc::new(RefCell::new(root_borrow.clone())));
                self.set_height(xborrow);
            }
            None => {}
        }

        return lt_node.clone();

    }
}

impl<T: MyTrait> AVLTree<T> {
    fn left_rotate(&mut self, root_borrow:&mut RefMut<'_, Node<T>>)->Option<Rc<RefCell<Node<T>>>> {
        let root_clone = root_borrow.clone();
        let rt_node = &root_clone.rt_node;

        match rt_node {
            Some(x) => {
                let rt_lt_node = &x.borrow().lt_node;
                match rt_lt_node {
                    Some(y) => {
                        root_borrow.rt_node = Some(y.clone());
                    }
                    None => {
                        root_borrow.rt_node = None;
                    }
                }
            }
            None => {}
        }

        self.set_height(root_borrow);

        match rt_node {
            Some(ref x) => {
                let xborrow = &mut x.borrow_mut();
                xborrow.lt_node = Some(Rc::new(RefCell::new(root_borrow.clone())));
                self.set_height(xborrow);
            }
            None => {}
        }

        return rt_node.clone();

    }
}

impl<T: MyTrait> AVLTree<T> {
    fn insert(&mut self, root_node:&mut Option<Rc<RefCell<Node<T>>>>, &val:&T)->Option<Rc<RefCell<Node<T>>>> {        
        let mut output:Option<Rc<RefCell<Node<T>>>> = root_node.clone();

        match root_node {
            Some(ref node) => {
                let mut node_borrow = node.borrow_mut();

                if val <= node_borrow.val {
                    let lt_node = &mut node_borrow.lt_node;
                    let new_lt:Option<Rc<RefCell<Node<T>>>>;

                    match lt_node {
                        Some(_x) => {
                            new_lt = self.insert(lt_node, &val);
                        }
                        None => {
                            let new_node:Node<T> = Node::new(&val);
                            new_lt = Some(Rc::new(RefCell::new(new_node)));
                        }
                    }

                    node_borrow.lt_node = new_lt;
                }
                else {
                    let rt_node = &mut node_borrow.rt_node;
                    let new_rt:Option<Rc<RefCell<Node<T>>>>;

                    match rt_node {
                        Some(_x) => {
                            new_rt = self.insert(rt_node, &val);
                        }
                        None => {
                            let new_node:Node<T> = Node::new(&val);
                            new_rt = Some(Rc::new(RefCell::new(new_node)));
                        }
                    }
                    node_borrow.rt_node = new_rt;
                }
            }
            None => {
                let new_node:Node<T> = Node::new(&val);
                output = Some(Rc::new(RefCell::new(new_node)));
            }
        }

        let mut hdiff:isize = 0;

        match root_node {
            Some(ref node) => {
                let mut node_borrow = node.borrow_mut();

                self.set_height(&mut node_borrow);
                hdiff = self.height_diff(&node_borrow);
            }
            None => {}
        }

        match root_node {
            Some(ref node) => {
                let mut node_borrow = node.borrow_mut();
                
                if hdiff > 1 {
                    let lt_node = &mut node_borrow.lt_node;

                    match lt_node {
                        Some(ref x) => {
                            if val <= x.borrow().val {
                                output = self.right_rotate(&mut node_borrow);
                            }
                            else if val > x.borrow().val {
                                let new_lt = self.left_rotate(&mut x.borrow_mut());
                                node_borrow.lt_node = new_lt;
                                output = self.right_rotate(&mut node_borrow);
                            }
                        }
                        None => {}
                    }
                }
        
                else if hdiff < -1 {
                    let rt_node = &mut node_borrow.rt_node;

                    match rt_node {
                        Some(ref x) => {
                            if val <= x.borrow().val {
                                let new_rt = self.right_rotate(&mut x.borrow_mut());
                                node_borrow.rt_node = new_rt;
                                output = self.left_rotate(&mut node_borrow);
                            }
                            else if val > x.borrow().val {
                                output = self.left_rotate(&mut node_borrow);
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        return output.clone();
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn print_tree(&self, root_node:&Option<Rc<RefCell<Node<T>>>>, level:usize) {  
        match root_node {
            Some(node) => {
                let node_borrow = node.borrow();
                println!("{} {:?}", "-".repeat(2*level), node_borrow.val);
                self.print_tree(&node_borrow.lt_node, level+1);
                self.print_tree(&node_borrow.rt_node, level+1);
            }
            None => {}
        }
    }
}

fn main() {
    let mut avltree:AVLTree<usize> = AVLTree::new();
    let mut root:Option<Rc<RefCell<Node<usize>>>> = None;

    let mut vec: Vec<usize> = (1..32).collect();
    vec.shuffle(&mut thread_rng());

    for v in vec.iter() {
        root = avltree.insert(&mut root, &v);
        avltree.print_tree(&root, 0);
        println!();
    }
    avltree.root_node = root.clone();
}