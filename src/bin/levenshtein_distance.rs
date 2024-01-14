use clap::{Arg, Command};

fn ldistance(str1: &str, str2: &str) -> u32 {
    let n:usize = str1.len();
    let m:usize = str2.len();

    let mut dist:Vec<Vec<u32>> = vec![vec![0; m]; n];

    for (i, c) in str1.chars().enumerate() {
        for (j, d) in str2.chars().enumerate() {
            if (i == 0) && (j == 0) {
                if c == d {
                    dist[i][j] = 0;
                } else {
                    dist[i][j] = 1;
                }
            } else if i == 0 {
                dist[i][j] = 1 + dist[i][j-1];
            } else if j == 0 {
                dist[i][j] = 1 + dist[i-1][j];
            } else {
                let mut b:u32 = 0;
                if c == d {
                    b = 1;
                }

                let x:u32 = 1 + dist[i-1][j];
                let y:u32 = 1 + dist[i][j-1];
                let z:u32 = b + dist[i-1][j-1];

                if (x <= y) && (x <= z) {
                    dist[i][j] = x;
                } else if (y <= x) && (y <= z) {
                    dist[i][j] = y;
                } else {
                    dist[i][j] = z;
                }
            }
        }
    }

    dist[n-1][m-1]
}

fn main() {
    let matches = Command::new("Edit Distance Program")
        .version("0.1.0")
        .author("Abhijit Mondal <abhi2iitk@gmail.com>")
        .about("Find edit distance between 2 strings")
        .arg(Arg::new("string1")
                 .short('s')
                 .long("string1")
                 .help("1st string"))
        .arg(Arg::new("string2")
                 .short('t')
                 .long("string2")
                 .help("2nd string"))
        .get_matches();

    let default = String::from("apple");
    let str1 = matches.get_one::<String>("string1").unwrap_or(&default);
    let str2 = matches.get_one::<String>("string2").unwrap_or(&default);

    let distance: u32 = ldistance(str1.as_str(), str2.as_str());
    println!("Distance = {}", distance);
}