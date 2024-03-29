use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::Bytes;
use std::str::FromStr;

#[derive(Clone)]
struct Node {
    character : char,
    frequency : usize,
    lt_node : Option<Box<Node>>,
    rt_node : Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.frequency <= other.frequency {
            return Ordering::Greater;
        }
        else {
            return Ordering::Less;
        }
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.frequency <= other.frequency {
            return Some(Ordering::Greater);
        }
        else {
            return Some(Ordering::Less);
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

struct Huffman {
    root_node : Option<Box<Node>>,
    priority_queue : BinaryHeap<Node>,
}

impl Node {
    fn new(character:char, frequency:usize) -> Self {
        Self {
            character,
            frequency,
            lt_node : None,
            rt_node : None,
        }
    }
}

impl Huffman {
    fn new() -> Self {
        Self {
            root_node : None,
            priority_queue : BinaryHeap::new(),
        }
    }
}

impl Huffman {
    fn insert(&mut self, mytext: String) {
        let mut character_cnts:HashMap<char, usize> = HashMap::new();
        for (_i, c) in mytext.chars().enumerate() {
            let cnt = character_cnts.entry(c).or_insert(0);
            *cnt += 1;
        }

        for (c, cnt) in &character_cnts {
            let node:Node = Node::new(*c, *cnt);
            self.priority_queue.push(node);
        }

        while self.priority_queue.len() > 0 {
            let mut lt_node:Option<Box<Node>> = None;
            let mut rt_node:Option<Box<Node>> = None;

            let mut lt_freq:usize = 0;
            let mut rt_freq:usize = 0;

            let pq_root = self.priority_queue.pop();
            match pq_root {
                Some(root) => {
                    lt_freq = root.frequency;
                    lt_node = Some(Box::new(root));
                }
                None => {}
            }

            if self.priority_queue.len() > 0 {
                let pq_root = self.priority_queue.pop();
                match pq_root {
                    Some(root) => {
                        rt_freq = root.frequency;
                        rt_node = Some(Box::new(root));
                    }
                    None => {}
                }
            }

            let node_char:char = '\0';
            let node_freq:usize = lt_freq + rt_freq;

            let mut node:Node = Node::new(node_char, node_freq);

            node.lt_node = lt_node;
            node.rt_node = rt_node;

            self.priority_queue.push(node.clone());

            if self.priority_queue.len() == 1 {
                self.root_node = Some(Box::new(node));
                break;
            }
        }
    }
}

impl Huffman {
    fn print_tree(&self, root_node:&Option<Box<Node>>, level:usize) {  
        match root_node {
            Some(node) => {
                println!("{} {:?}", "-".repeat(2*level), node.character);
                self.print_tree(&node.lt_node, level+1);
                self.print_tree(&node.rt_node, level+1);
            }
            None => {}
        }
    }
}

impl Huffman {
    fn encode(&self, mytext:String, codes:&HashMap<char, String>) -> (Option<Vec<u32>>, usize) {
        let mut out_rep:String = String::from("");
        let mut vector:Vec<u32> = Vec::new();

        for (_i, c) in mytext.chars().enumerate() {
            if codes.contains_key(&c) {
                let rep = codes.get(&c);
                match rep {
                    Some(x) => {
                        out_rep += x;
                    }
                    None => {
                        return (None, 0);
                    }
                }
            }
            else {
                return (None, 0);
            }
        }

        let mut start:usize = 0;

        while start < out_rep.len() {
            let slice:&str;

            if start + 32 > out_rep.len() {
                slice = &out_rep[start..];
            }
            else {
                slice = &out_rep[start..start+32];
            }

            let mut q:u32 = 1 << 31;
            let mut v:u32 = 0;

            for (_j, c) in slice.chars().enumerate() {
                if c == '1' {
                    v += q;
                }
                q = q >> 1;
            }

            vector.push(v);
            start += 32;
        }

        return (Some(vector), mytext.len());
    }
}

impl Huffman {
    fn decode(&self, encoded:(Option<Vec<u32>>, usize), codes:&HashMap<char, String>) -> Option<String> {
        let mut output = String::from("");

        let enc = encoded.0;
        let len = encoded.1;

        let mut codes_rev:HashMap<String, char> = HashMap::new();
        for (k, v) in codes {
            codes_rev.entry(v.to_string()).or_insert(*k);
        }

        match enc {
            Some(coding) => {
                let mut rep = String::from("");
                for i in 0..coding.len() {
                    let mut v = coding[i];
                    let mut r = String::from("");
                    while v > 0 {
                        let rem = v & 1;
                        r = rem.to_string() + &r;
                        v = v >> 1;
                    }

                    if r.len() < 32 {
                        let diff = 32 - r.len();
                        r = "0".repeat(diff) + &r;
                    }

                    rep += &r;
                }

                let mut curr = String::from("");
                let mut num:usize = 0;

                for (_i, c) in rep.chars().enumerate() {
                    curr += &c.to_string();

                    if codes_rev.contains_key(&curr) {
                        num += 1;
                        let character = codes_rev.get(&curr);

                        match character {
                            Some(x) => {
                                output += &x.to_string();
                            }
                            None => {
                                return None;
                            }
                        }

                        if num == len {
                            break;
                        }

                        curr = String::from("");
                    }
                }
            }
            None => {
                return None;
            }
        }

        return Some(output);
    }
    
}

impl Huffman {
    fn get_codes(&self, node: &Node, curr_code: String, codes: &mut HashMap<char, String>) {
        if (node.lt_node.is_none()) && (node.rt_node.is_none()) {
            let character = node.character;
            codes.entry(character).or_insert(curr_code);
        }
        else {
            match &node.lt_node {
                Some(lt_node) => {
                    self.get_codes(&*lt_node, curr_code.clone() + "0", codes);
                }
                None => {}
            }

            match &node.rt_node {
                Some(rt_node) => {
                    self.get_codes(&*rt_node, curr_code.clone() + "1", codes);
                }
                None => {}
            }
        }
    }
}

fn main() {
    let mytext:String = String::from("_Walt Whitman has somewhere a fine and just distinction between “loving
    by allowance” and “loving with personal love.” This distinction applies
    to books as well as to men and women; and in the case of the not very
    numerous authors who are the objects of the personal affection, it
    brings a curious consequence with it. There is much more difference as
    to their best work than in the case of those others who are loved “by
    allowance” by convention, and because it is felt to be the right and
    proper thing to love them. And in the sect--fairly large and yet
    unusually choice--of Austenians or Janites, there would probably be
    found partisans of the claim to primacy of almost every one of the
    novels. To some the delightful freshness and humour of_ Northanger
    Abbey, _its completeness, finish, and_ entrain, _obscure the undoubted
    critical facts that its scale is small, and its scheme, after all, that
    of burlesque or parody, a kind in which the first rank is reached with
    difficulty._ Persuasion, _relatively faint in tone, and not enthralling
    in interest, has devotees who exalt above all the others its exquisite
    delicacy and keeping. The catastrophe of_ Mansfield Park _is admittedly
    theatrical, the hero and heroine are insipid, and the author has almost
    wickedly destroyed all romantic interest by expressly admitting that
    Edmund only took Fanny because Mary shocked him, and that Fanny might
    very likely have taken Crawford if he had been a little more assiduous;
    yet the matchless rehearsal-scenes and the characters of Mrs. Norris and
    others have secured, I believe, a considerable party for it._ Sense and
    Sensibility _has perhaps the fewest out-and-out admirers; but it does
    not want them._
    
    _I suppose, however, that the majority of at least competent votes
    would, all things considered, be divided between_ Emma _and the present
    book; and perhaps the vulgar verdict (if indeed a fondness for Miss
    Austen be not of itself a patent of exemption from any possible charge
    of vulgarity) would go for_ Emma. _It is the larger, the more varied, the
    more popular; the author had by the time of its composition seen rather
    more of the world, and had improved her general, though not her most
    peculiar and characteristic dialogue; such figures as Miss Bates, as the
    Eltons, cannot but unite the suffrages of everybody. On the other hand,
    I, for my part, declare for_ Pride and Prejudice _unhesitatingly. It
    seems to me the most perfect, the most characteristic, the most
    eminently quintessential of its author’s works; and for this contention
    in such narrow space as is permitted to me, I propose here to show
    cause._
    
    _In the first place, the book (it may be barely necessary to remind the
    reader) was in its first shape written very early, somewhere about 1796,
    when Miss Austen was barely twenty-one; though it was revised and
    finished at Chawton some fifteen years later, and was not published till
    1813, only four years before her death. I do not know whether, in this
    combination of the fresh and vigorous projection of youth, and the
    critical revision of middle life, there may be traced the distinct
    superiority in point of construction, which, as it seems to me, it
    possesses over all the others. The plot, though not elaborate, is almost
    regular enough for Fielding; hardly a character, hardly an incident
    could be retrenched without loss to the story. The elopement of Lydia
    and Wickham is not, like that of Crawford and Mrs. Rushworth, a_ coup de
    théâtre; _it connects itself in the strictest way with the course of the
    story earlier, and brings about the denouement with complete propriety.
    All the minor passages--the loves of Jane and Bingley, the advent of Mr.
    Collins, the visit to Hunsford, the Derbyshire tour--fit in after the
    same unostentatious, but masterly fashion. There is no attempt at the
    hide-and-seek, in-and-out business, which in the transactions between
    Frank Churchill and Jane Fairfax contributes no doubt a good deal to the
    intrigue of_ Emma, _but contributes it in a fashion which I do not think
    the best feature of that otherwise admirable book. Although Miss Austen
    always liked something of the misunderstanding kind, which afforded her
    opportunities for the display of the peculiar and incomparable talent to
    be noticed presently, she has been satisfied here with the perfectly
    natural occasions provided by the false account of Darcy’s conduct given
    by Wickham, and by the awkwardness (arising with equal naturalness) from
    the gradual transformation of Elizabeth’s own feelings from positive
    aversion to actual love. I do not know whether the all-grasping hand of
    the playwright has ever been laid upon_ Pride and Prejudice; _and I dare
    say that, if it were, the situations would prove not startling or
    garish enough for the footlights, the character-scheme too subtle and
    delicate for pit and gallery. But if the attempt were made, it would
    certainly not be hampered by any of those loosenesses of construction,
    which, sometimes disguised by the conveniences of which the novelist can
    avail himself, appear at once on the stage._
    ");
    let mut hm = Huffman::new();
    hm.insert(mytext);
    hm.print_tree(&hm.root_node, 0);
    println!();

    let mut codes:HashMap<char, String> = HashMap::new();

    match &hm.root_node {
        Some(root) => {
            hm.get_codes(root, String::from(""), &mut codes);
        }
        None => {}
    }

    println!("{:?}", codes);

    let encoded = hm.encode(String::from("playwright has ever been laid upon_ Pride and Prejudice; _and I dare
    say that, if it were, the situations would prove not startling or
    garish enough for the footlights, the character-scheme too subtle and
    delicate for pit and gallery."), &codes);
    println!("{:?}", encoded);

    let decoded = hm.decode(encoded, &codes);

    println!("{:?}", decoded);
}
