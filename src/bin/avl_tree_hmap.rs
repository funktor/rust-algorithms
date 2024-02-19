use std::{cmp::max, fmt::Debug, collections::HashMap};
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
    auto_inc_id: usize,
    id_to_node_map: HashMap<usize, Node<T>>,
}

impl<T: MyTrait> AVLTree<T> {
    fn new() -> Self {
        Self {
            auto_inc_id: 0,
            id_to_node_map: HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct Node<T> {
    val: T,
    lt_node_id: Option<usize>,
    rt_node_id: Option<usize>,
    height: isize
}

impl<T: MyTrait> Node<T> {
    fn new(&val:&T) -> Self {
        Self {
            val,
            lt_node_id: None,
            rt_node_id: None,
            height: 1,
        }
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn get_lt_rt_heights(&self, root_id:Option<usize>) -> (isize, isize) {
        let mut lheight:isize = 0;
        let mut rheight:isize = 0;

        match root_id {
            Some(id) => {
                let root = self.id_to_node_map.get(&id);

                match root {
                    Some(x) => {
                        let lt_node_id = x.lt_node_id;
                        let rt_node_id = x.rt_node_id;

                        match lt_node_id {
                            Some(y) => {
                                match self.id_to_node_map.get(&y) {
                                    Some(z) => {
                                        lheight = z.height;
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }

                        match rt_node_id {
                            Some(y) => {
                                match self.id_to_node_map.get(&y) {
                                    Some(z) => {
                                        rheight = z.height;
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }
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
    fn height_diff(&self, root_node_id:Option<usize>) -> isize {
        let lr_heights = self.get_lt_rt_heights(root_node_id);
        return lr_heights.0 - lr_heights.1;
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn set_height(&mut self, root_node_id:Option<usize>) {
        let lr_heights = self.get_lt_rt_heights(root_node_id);

        match root_node_id {
            Some(id) => {
                match self.id_to_node_map.get(&id) {
                    Some(_x) => {
                        self.id_to_node_map.entry(id).and_modify(|root| root.height = 1 + max(lr_heights.0, lr_heights.1));
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn right_rotate(&mut self, root_node_id:Option<usize>) -> Option<usize> {
        match root_node_id {
            Some(root_node_id_val) => {
                let root = self.id_to_node_map.get(&root_node_id_val);
                match root {
                    Some(root_ref) => {
                        let lt_node_id = root_ref.lt_node_id;
                        match lt_node_id {
                            Some(lt_node_id_val) => {
                                let lt_node = self.id_to_node_map.get(&lt_node_id_val);
                                match lt_node {
                                    Some(lt_node_ref) => {
                                        let lt_rt_node_id = lt_node_ref.rt_node_id;

                                        match lt_rt_node_id {
                                            Some(_lt_rt_node_id_val) => {
                                                self.id_to_node_map.entry(root_node_id_val).and_modify(|x| x.lt_node_id = lt_rt_node_id);
                                            }
                                            None => {
                                                self.id_to_node_map.entry(root_node_id_val).and_modify(|x| x.lt_node_id = None);
                                            }
                                        }

                                        self.id_to_node_map.entry(lt_node_id_val).and_modify(|x| x.rt_node_id = root_node_id);
                                        self.set_height(root_node_id);
                                        self.set_height(lt_node_id);

                                        return lt_node_id;
                                    }
                                    None => {}
                                }
                            }
                            None => {
                                return root_node_id;
                            }
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
        return None;
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn left_rotate(&mut self, root_node_id:Option<usize>) -> Option<usize> {
        match root_node_id {
            Some(root_node_id_val) => {
                let root = self.id_to_node_map.get(&root_node_id_val);
                match root {
                    Some(root_ref) => {
                        let rt_node_id = root_ref.rt_node_id;
                        match rt_node_id {
                            Some(rt_node_id_val) => {
                                let rt_node = self.id_to_node_map.get(&rt_node_id_val);
                                match rt_node {
                                    Some(rt_node_ref) => {
                                        let rt_lt_node_id = rt_node_ref.lt_node_id;

                                        match rt_lt_node_id {
                                            Some(_rt_lt_node_id_val) => {
                                                self.id_to_node_map.entry(root_node_id_val).and_modify(|x| x.rt_node_id = rt_lt_node_id);
                                            }
                                            None => {
                                                self.id_to_node_map.entry(root_node_id_val).and_modify(|x| x.rt_node_id = None);
                                            }
                                        }

                                        self.id_to_node_map.entry(rt_node_id_val).and_modify(|x| x.lt_node_id = root_node_id);
                                        self.set_height(root_node_id);
                                        self.set_height(rt_node_id);

                                        return rt_node_id;
                                    }
                                    None => {}
                                }
                            }
                            None => {
                                return root_node_id;
                            }
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
        return None;
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn check_and_rotate(&mut self, root_node_id:Option<usize>, &val:&T)->Option<usize> {
        match root_node_id {
            Some(root_node_id_val) => {
                let hdiff = self.height_diff(root_node_id);

                if hdiff > 1 {
                    let root = self.id_to_node_map.get(&root_node_id_val);
                    match root {
                        Some(root_ref) => {
                            let lt_node_id = root_ref.lt_node_id;
                            match lt_node_id {
                                Some(lt_node_id_val) => {
                                    let lt_node = self.id_to_node_map.get(&lt_node_id_val);
                                    match lt_node {
                                        Some(lt_node_ref) => {
                                            if val <= lt_node_ref.val {
                                                return self.right_rotate(root_node_id);
                                            }
                                            else if val > lt_node_ref.val {
                                                let new_lt_id = self.left_rotate(lt_node_id);
                                                self.id_to_node_map.entry(root_node_id_val).and_modify(|z| z.lt_node_id = new_lt_id);
                                                return self.right_rotate(root_node_id);
                                            }
                                        }
                                        None => {}
                                    }
                                }
                                None => {}
                            }  
                        }
                        None => {}
                    } 
                }

                else if hdiff < -1 {
                    let root = self.id_to_node_map.get(&root_node_id_val);
                    match root {
                        Some(root_ref) => {
                            let rt_node_id = root_ref.rt_node_id;
                            match rt_node_id {
                                Some(rt_node_id_val) => {
                                    let rt_node = self.id_to_node_map.get(&rt_node_id_val);
                                    match rt_node {
                                        Some(rt_node_ref) => {
                                            if val > rt_node_ref.val {
                                                return self.left_rotate(root_node_id);
                                            }
                                            else if val <= rt_node_ref.val {
                                                let new_rt_id = self.right_rotate(rt_node_id);
                                                self.id_to_node_map.entry(root_node_id_val).and_modify(|z| z.rt_node_id = new_rt_id);
                                                return self.left_rotate(root_node_id);
                                            }
                                        }
                                        None => {}
                                    }
                                }
                                None => {}
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        return root_node_id;
    }
}


impl<T: MyTrait> AVLTree<T> {
    fn check_and_rotate_deletion(&mut self, root_node_id:Option<usize>)->Option<usize> {
        match root_node_id {
            Some(root_node_id_val) => {
                let hdiff = self.height_diff(root_node_id);

                if hdiff > 1 {
                    let root = self.id_to_node_map.get(&root_node_id_val);
                    match root {
                        Some(root_ref) => {
                            let lt_node_id = root_ref.lt_node_id;
                            let hdiff_lt_node_id = self.height_diff(lt_node_id);

                            if hdiff_lt_node_id >= 0 {
                                return self.right_rotate(root_node_id);
                            }
                            else {
                                let new_lt_id = self.left_rotate(lt_node_id);
                                self.id_to_node_map.entry(root_node_id_val).and_modify(|z| z.lt_node_id = new_lt_id);
                                return self.right_rotate(root_node_id);
                            }
                        }
                        None => {}
                    } 
                }

                else if hdiff < -1 {
                    let root = self.id_to_node_map.get(&root_node_id_val);
                    match root {
                        Some(root_ref) => {
                            let rt_node_id = root_ref.rt_node_id;
                            let hdiff_rt_node_id = self.height_diff(rt_node_id);

                            if hdiff_rt_node_id < 0 {
                                return self.left_rotate(root_node_id);
                            }
                            else {
                                let new_rt_id = self.right_rotate(rt_node_id);
                                self.id_to_node_map.entry(root_node_id_val).and_modify(|z| z.rt_node_id = new_rt_id);
                                return self.left_rotate(root_node_id);
                            }                     
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        return root_node_id;
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn insert(&mut self, root_node_id:Option<usize>, &val:&T)->Option<usize> {
        match root_node_id {
            Some(root_node_id_val) => {
                let root = self.id_to_node_map.get(&root_node_id_val);
                match root {
                    Some(root_ref) => {
                        if val <= root_ref.val {
                            let lt_node_id = root_ref.lt_node_id;
                            match lt_node_id {
                                Some(_lt_node_id_val) => {
                                    let out = self.insert(lt_node_id, &val);
                                    self.id_to_node_map.entry(root_node_id_val).and_modify(|x| x.lt_node_id = out);
                                }
                                None => {
                                    let new_node:Node<T> = Node::new(&val);
                                    self.auto_inc_id += 1;
                                    self.id_to_node_map.insert(self.auto_inc_id, new_node);
                                    self.id_to_node_map.entry(root_node_id_val).and_modify(|x| x.lt_node_id = Some(self.auto_inc_id));
                                }
                            }
                        }
                        else {
                            let rt_node_id = root_ref.rt_node_id;
                            match rt_node_id {
                                Some(_rt_node_id_val) => {
                                    let out = self.insert(rt_node_id, &val);
                                    self.id_to_node_map.entry(root_node_id_val).and_modify(|x| x.rt_node_id = out);
                                }
                                None => {
                                    let new_node:Node<T> = Node::new(&val);
                                    self.auto_inc_id += 1;
                                    self.id_to_node_map.insert(self.auto_inc_id, new_node);
                                    self.id_to_node_map.entry(root_node_id_val).and_modify(|x| x.rt_node_id = Some(self.auto_inc_id));
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
            None => {
                let new_node:Node<T> = Node::new(&val);
                self.auto_inc_id += 1;
                self.id_to_node_map.insert(self.auto_inc_id, new_node);
                return Some(self.auto_inc_id);
            }
        }

        match root_node_id {
            Some(_root_node_id_val) => {
                self.set_height(root_node_id);
            }
            None => {}
        }
        
        return self.check_and_rotate(root_node_id, &val);
    }
}

impl<T: MyTrait> AVLTree<T> {
    fn delete(&mut self, root_node_id:Option<usize>, &val:&T)->Option<usize> {
        let mut root_id:Option<usize> = root_node_id;

        match root_node_id {
            Some(node) => {
                let root = &mut self.id_to_node_map.get(&node);
                match root {
                    Some(root_ref) => {
                        if val == root_ref.val {
                            let lt_node_id = root_ref.lt_node_id;
                            let rt_node_id = root_ref.rt_node_id;

                            if (lt_node_id.is_none()) && (rt_node_id.is_none()) {
                                self.id_to_node_map.remove_entry(&node);
                                root_id = None;
                            }

                            else if lt_node_id.is_none() {
                                match rt_node_id {
                                    Some(x) => {
                                        let rt_node = self.id_to_node_map.get(&x);
                                        match rt_node {
                                            Some(rt_node_ref) => {
                                                let v = rt_node_ref.val;
                                                self.id_to_node_map.entry(node).and_modify(|z| {z.val = v; z.rt_node_id = None;});
                                            }
                                            None => {}
                                        }
                                        self.id_to_node_map.remove_entry(&x);
                                    }
                                    None => {}
                                }
                            }

                            else if rt_node_id.is_none() {
                                match lt_node_id {
                                    Some(x) => {
                                        let lt_node = self.id_to_node_map.get(&x);
                                        match lt_node {
                                            Some(lt_node_ref) => {
                                                let v = lt_node_ref.val;
                                                self.id_to_node_map.entry(node).and_modify(|z| {z.val = v; z.lt_node_id = None;});
                                            }
                                            None => {}
                                        }
                                        self.id_to_node_map.remove_entry(&x);
                                    }
                                    None => {}
                                }
                            }

                            else {
                                let mut next_id = rt_node_id;
                                let mut sval:Option<T> = None;

                                while !next_id.is_none() {
                                    match next_id {
                                        Some(x) => {
                                            let next = &mut self.id_to_node_map.get(&x);
                                            match next {
                                                Some(next_ref) => {
                                                    sval = Some(next_ref.val);
                                                    next_id = next_ref.lt_node_id;
                                                }
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                }

                                match sval {
                                    Some(v) => {
                                        let new_rt_id = self.delete(root_ref.rt_node_id, &v);
                                        self.id_to_node_map.entry(node).and_modify(|z: &mut Node<T>| {z.rt_node_id = new_rt_id; z.val = v;});
                                    }
                                    None => {}
                                }
                            }
                        }

                        else if val < root_ref.val {
                            let new_lt_id = self.delete(root_ref.lt_node_id, &val);
                            self.id_to_node_map.entry(node).and_modify(|z: &mut Node<T>| z.lt_node_id = new_lt_id);
                        }

                        else {
                            let new_rt_id = self.delete(root_ref.rt_node_id, &val);
                            self.id_to_node_map.entry(node).and_modify(|z: &mut Node<T>| z.rt_node_id = new_rt_id);
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }

        match root_id {
            Some(x) => {
                let root = &mut self.id_to_node_map.get(&x);
                match root {
                    Some(_y) => {
                        self.set_height(root_id);
                    }
                    None => {}
                }
            }
            None => {}
        }

        return self.check_and_rotate_deletion(root_id);

    }
}

impl<T: MyTrait> AVLTree<T> {
    fn print_tree(&self, root_node_id:Option<usize>, level:usize) {  
        match root_node_id {
            Some(node) => {
                let root = self.id_to_node_map.get(&node);
                match root {
                    Some(x) => {
                        println!("{} {:?}", "-".repeat(2*level), x.val);
                        self.print_tree(x.lt_node_id, level+1);
                        self.print_tree(x.rt_node_id, level+1);                        
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}

fn main() {
    let mut avltree:AVLTree<usize> = AVLTree::new();
    let mut root:Option<usize> = None;

    let mut vec: Vec<usize> = (1..1000000).collect();
    vec.shuffle(&mut thread_rng());

    for v in vec.iter() {
        root = avltree.insert(root, &v);
        avltree.print_tree(root, 0);
        println!();
    }

    for i in 16..32 {
        root = avltree.delete(root, &i);
        avltree.print_tree(root, 0);
        println!();
    }
}