fn zfunc(inp:String)->Vec<usize> {
    let n = inp.len();
    let mut z:Vec<usize> = vec![0;n];
    let mut l:usize = 0;
    let mut r:usize = 0;

    for i in 0..n {
        if i > r {
            l = i;
            r = i;
            while (r < n) && (inp.as_bytes()[r] == inp.as_bytes()[r-l]) {
                r += 1;
            }
            z[i] = r-l;
        }
        else {
            if z[i-l] <= r-i {
                z[i] = z[i-l];
                r = i + z[i];
            }

            l = i;
            while (r < n) && (inp.as_bytes()[r] == inp.as_bytes()[r-l]) {
                r += 1;
            }
            z[i] = r-l;
        }
    }
    
    return z;
}

fn main() {
    let inp:String = String::from("abababab");
    let sep:String = String::from("#");
    let pat:String = String::from("baba");

    let mut new_str:String = pat.clone();
    new_str.push_str(&sep);
    new_str.push_str(&inp);

    println!("{:?}", new_str);

    let n = new_str.len();

    let output = zfunc(new_str);
    println!("{:?}", output);

    let m = pat.len() + sep.len();

    let mut positions:Vec<usize> = Vec::new();

    for i in m..n {
        if output[i] == pat.len() {
            positions.push(i);
            println!("Found at {:?}", i);
        }
    }

    println!("{:?}", positions);
}