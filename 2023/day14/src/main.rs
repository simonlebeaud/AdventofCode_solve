use std::fs;
use std::cmp::max;

fn parse_file(path: &str) -> Vec<Vec<char>> {
    let mut mountain:Vec<Vec<char>> = Vec::new();
    for l in fs::read_to_string(path).unwrap().lines(){
        mountain.push(l.trim().chars().collect());
    }
    mountain
}

fn total(mountain: &Vec<Vec<char>>) {
    let mlen = mountain.len();
    let mut score = 0;
    for i in 0..mlen {
        for j in 0..mountain[0].len() {
            if mountain[i][j]=='O' {
                score+=mlen-i;
            }
        }
    }

    println!("{}",score);
}

fn move_north(mountain: &mut Vec<Vec<char>>) {
    let mlen = mountain.len();
    for j in 0..mountain[0].len() {
        let mut nm_rock=-1;
        let mut last_m_rock=-1;
        for i in 0..mlen{
            match mountain[i][j] {
                'O' => {
                    mountain[i][j] = '.';
                    mountain[max(nm_rock+1,last_m_rock+1) as usize][j]='O';
                    last_m_rock=max(nm_rock+1,last_m_rock+1);
                },
                '#' => nm_rock=i as i64,
                _ => continue,
            }
        }
    }
}

fn rotate(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }
    let n = matrix.len();
    let m = matrix[0].len();
    let mut rotated = vec![vec![' '; n]; m];
    for i in 0..n {
        for j in 0..m {
            rotated[j][n - i - 1] = matrix[i][j];
        }
    }
    rotated
}

fn solve_p1() {
    let mut mountain = parse_file("input.txt");
    move_north(&mut mountain);
    print!("P1 = ");
    total(&mountain);
}

// This is a very slow implementation but we dont need to compute 1000000000 only 1000 does the trick, we found the cycle.
fn solve_p2() {
    let mut mountain = parse_file("input.txt");

    for _ in 0..1000{
        for _ in 0..4 {
            move_north(&mut mountain);
            mountain = rotate(mountain);
        }   
    }
    print!("P2 = ");
    total(&mountain);
}

fn main() {
    solve_p1();
    solve_p2();
    
    
}


