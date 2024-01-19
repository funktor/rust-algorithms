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

struct ArenaTree<T> where T: PartialOrd {
    arena: Vec<Node<T>>,
}

impl<T: MyTrait> ArenaTree<T> {
    fn new() -> Self {
        Self {
            arena: Vec::new(),
        }
    }
}

struct Node<T> {
    val: T,
    lt_idx: Option<usize>,
    rt_idx: Option<usize>,
    height: i32
}

impl<T: MyTrait> Node<T> {
    fn new(&val:&T) -> Self {
        Self {
            val,
            lt_idx: None,
            rt_idx: None,
            height: 1,
        }
    }
}

impl<T: MyTrait> ArenaTree<T> {
    fn get_lt_rt_heights(&self, idx: usize) -> (i32, i32) {
        let mut lheight:i32 = 0;
        let mut rheight:i32 = 0;

        match self.arena[idx].lt_idx {
            Some(x) => {
                lheight = self.arena[x].height;
            }
            None => {}
        }

        match self.arena[idx].rt_idx {
            Some(x) => {
                rheight = self.arena[x].height;
            }
            None => {}
        }

        return (lheight, rheight);
    }
}

impl<T: MyTrait> ArenaTree<T> {
    fn height_diff(&self, idx: usize) -> i32 {
        let lr_heights = self.get_lt_rt_heights(idx);
        return lr_heights.0 - lr_heights.1;
    }
}

impl<T: MyTrait> ArenaTree<T> {
    fn set_height(&mut self, idx: usize) {
        let lr_heights = self.get_lt_rt_heights(idx);
        self.arena[idx].height = 1 + max(lr_heights.0, lr_heights.1);
    }
}

impl<T: MyTrait> ArenaTree<T> {
    fn right_rotate(&mut self, idx: usize) -> usize {
        match self.arena[idx].lt_idx {
            Some(x) => {
                self.arena[idx].lt_idx = self.arena[x].rt_idx;
                self.arena[x].rt_idx = Some(idx);
                self.set_height(idx);
                self.set_height(x);
                return x;
            }
            None => {}
        }

        return idx;
    }
}

impl<T: MyTrait> ArenaTree<T> {
    fn left_rotate(&mut self, idx: usize) -> usize {
        match self.arena[idx].rt_idx {
            Some(x) => {
                self.arena[idx].rt_idx = self.arena[x].lt_idx;
                self.arena[x].lt_idx = Some(idx);
                self.set_height(idx);
                self.set_height(x);
                return x;
            }
            None => {}
        }

        return idx;
    }
}

impl<T: MyTrait> ArenaTree<T> {
    fn insert(&mut self, root: Option<usize>, &val:&T) -> Option<usize> {
        let mut root_idx:usize;

        match root {
            Some(x) => {
                root_idx = x;
            }
            None => {
                let idx = self.arena.len();
                let node:Node<T> = Node::new(&val);
                self.arena.push(node);
                return Some(idx);
            }
        }

        if val <= self.arena[root_idx].val {
            match self.arena[root_idx].lt_idx {
                Some(x) => {
                    self.arena[root_idx].lt_idx = self.insert(Some(x), &val);
                }
                None => {
                    let idx = self.arena.len();
                    let node:Node<T> = Node::new(&val);
                    self.arena.push(node);
                    self.arena[root_idx].lt_idx = Some(idx);
                }
            }
        }
        else {
            match self.arena[root_idx].rt_idx {
                Some(x) => {
                    self.arena[root_idx].rt_idx = self.insert(Some(x), &val);
                }
                None => {
                    let idx = self.arena.len();
                    let node:Node<T> = Node::new(&val);
                    self.arena.push(node);
                    self.arena[root_idx].rt_idx = Some(idx);
                }
            }
        }

        let lt_idx = self.arena[root_idx].lt_idx;
        let rt_idx = self.arena[root_idx].rt_idx;

        self.set_height(root_idx);
        let hdiff = self.height_diff(root_idx);

        if hdiff > 1 {
            match lt_idx {
                Some(x) => {
                    if val <= self.arena[x].val {
                        return Some(self.right_rotate(root_idx));
                    }
                    else if val > self.arena[x].val {
                        root_idx = self.left_rotate(root_idx);
                        return Some(self.right_rotate(root_idx));
                    }
                }
                None => {}
            }
        }

        else if hdiff < -1 {
            match rt_idx {
                Some(x) => {
                    if val > self.arena[x].val {
                        return Some(self.left_rotate(root_idx));
                    }
                    else if val <= self.arena[x].val {
                        root_idx = self.right_rotate(root_idx);
                        return Some(self.left_rotate(root_idx));
                    }
                }
                None => {}
            }
        }

        return Some(root_idx);

    }
}

impl<T: MyTrait> ArenaTree<T> {
    fn print_tree(&self, idx: Option<usize>, level:usize) {
        match idx {
            Some(root) => {
                println!("{} {:?}", "-".repeat(2*level), self.arena[root].val);
                self.print_tree(self.arena[root].lt_idx, level+1);
                self.print_tree(self.arena[root].rt_idx, level+1);
            }
            None => {}
        }
        
    }
}

fn main() {
    let mut avltree:ArenaTree<u32> = ArenaTree::new();
    let mut root:Option<usize> = None;

    for i in 1..32 {
        root = avltree.insert(root, &i);
        avltree.print_tree(root, 0);
        println!();
    }
}