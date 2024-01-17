fn dfs(mat: &[Vec<u32>], start_x:u32, start_y:u32, end_x:u32, end_y:u32) -> bool {
    let start = (start_x, start_y);
    let n:usize = mat.len();
    let m:usize = mat[0].len();

    let mut stack:Vec<(u32, u32)> = Vec::new();
    stack.push(start);

    let mut visited:Vec<Vec<i8>> = vec![vec![0; m]; n];
    visited[start_x as usize][start_y as usize] = 1;

    while stack.len() > 0 {
        let top:(u32, u32) = stack[stack.len()-1];
        let x:u32 = top.0;
        let y:u32 = top.1;

        println!("Current = {}, {}", x, y);

        if (x == end_x) && (y == end_y) {
            return true;
        }

        let directions:[(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
        let mut can_move:bool = false;

        for d in directions.iter() {
            let p:i32 = d.0;
            let q:i32 = d.1;

            let mut x_new:u32 = 0;
            let mut y_new:u32 = 0;

            let mut valid:bool = true;

            if (p == -1) || (q == -1) {
                if (p == -1) && (x > 0) {
                    x_new = x-1;
                    y_new = y;
                }
                else if (q == -1) && (y > 0) {
                    x_new = x;
                    y_new = y-1;
                }
                else {
                    valid = false;
                }
            }
            else {
                if p == 1 {
                    x_new = x+1;
                    y_new = y;
                }
                else {
                    x_new = x;
                    y_new = y+1;
                }
            }

            let x_new_cast:usize = x_new as usize;
            let y_new_cast:usize = y_new as usize;

            if (valid) && (x_new_cast < n) && (y_new_cast < m) && (mat[x_new_cast][y_new_cast] != 0) && (visited[x_new_cast][y_new_cast] == 0) {
                let next:(u32, u32) = (x_new, y_new);
                stack.push(next);
                visited[x_new_cast][y_new_cast] = 1;
                can_move = true;
                break;
            }
        }

        if !can_move {
            stack.pop();
        }
    }

    return false;
}

fn main() {
    let mat = vec![vec![1,1,0,1],vec![1,1,0,1],vec![1,1,0,1],vec![1,1,1,1]];
    let reachable: bool = dfs(&mat[..], 0, 0, 0, 3);
    println!("Reachable = {}", reachable);
}