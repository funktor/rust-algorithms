use std::collections::HashMap;

fn topsort<'a>(dependency:&mut HashMap<&'a str, Vec<&'a str>>)->Vec<&'a str> {
    let mut in_deg:HashMap<&str, u32> = HashMap::new();

    for (key, values) in &*dependency {
        in_deg.entry(key).or_insert(0);

        for value in values.iter() {
            let deg: &mut u32 = in_deg.entry(value).or_insert(0);
            *deg += 1;
        }
    }

    let mut output:Vec<&str> = Vec::new();

    let mut my_vector:Vec<&str> = Vec::new();
    for (key, deg) in &in_deg {
        if *deg == 0 {
            my_vector.push(key);
        }
    }

    while my_vector.len() > 0 {
        let mut next_level:Vec<&str> = Vec::new();

        for key in my_vector.iter() {
            output.push(key);

            match dependency.get(key) {
                Some(children) => { 
                    for child in children.iter() {
                        match in_deg.get(child) {
                            Some(deg) => {
                                if *deg == 1 {
                                    next_level.push(child);
                                }
                                in_deg.insert(child, *deg-1);
                            }
                            None => {}
                        }
                    }
                }
                None => {}
            }
        }

        my_vector.clear();

        for key in next_level.iter() {
            my_vector.push(key);
        }
    }

    return output;
}

fn main() {
    let mut dependency:HashMap<&str, Vec<&str>> = HashMap::new();

    dependency.insert("A", vec!["D"]);
    dependency.insert("B", vec!["D", "E"]);
    dependency.insert("C", vec!["E"]);
    dependency.insert("D", vec!["F", "G"]);
    dependency.insert("E", vec!["G"]);
    dependency.insert("F", vec!["H"]);
    dependency.insert("G", vec!["H"]);

    let sorted = topsort(&mut dependency);
    println!("Topological sorting = {:?}", sorted);
}