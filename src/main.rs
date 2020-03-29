use std::{thread, time};

struct Board {
    //Alive cell: 1, dead cell: 0
    d: [[i32; 20]; 20],
    //Number of rows
    rows: i32,
    //Number of columns
    cols: i32,
}

impl Board {
    fn to_string(&self) -> String {
        let mut ret = String::new();
        for row in &self.d {
            for &num in row {
                if num == 1 {
                    ret.push_str("*");
                } else {
                    ret.push_str(".");
                }
                ret.push_str(" ");
            }
            ret.push_str("\n");
        }
        ret
    }

    //Next generation
    fn next(&self) -> Board {
        let mut aux: [[i32; 20]; 20] = self.d;

        for r in 1..self.rows - 1 {
            for c in 1..self.cols - 1 {
                let mut alive = 0;

                for i in -1..2 {
                    for j in -1..2 {
                        alive += self.d[(r + i) as usize][(c + j) as usize];
                    }
                }

                alive -= self.d[r as usize][c as usize];

                if (self.d[r as usize][c as usize] == 1) & (alive < 2) {
                    //If cell is lonely
                    aux[r as usize][c as usize] = 0;
                } else if (self.d[r as usize][c as usize] == 1) & (alive > 3){
                    //If cell is overpopulated
                    aux[r as usize][c as usize] = 0;
                }else if (self.d[r as usize][c as usize] == 0) & (alive == 3) {
                    //If dead cell has three alive neighbours
                    aux[r as usize][c as usize] = 1;
                }
            }
        }
        //Board with the next generation
        Board {
            d: aux,
            cols: self.cols,
            rows: self.rows,
        }
    }
}

fn main() {
    let mut grid = Board {
        d: [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0],
            [0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ],
        cols: 20,
        rows: 20,
    };

    let wait = time::Duration::from_millis(1000);

    loop {
        println!("{}", grid.to_string());
        grid = grid.next();
        thread::sleep(wait);
    }
}
