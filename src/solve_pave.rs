


pub mod solve_pave {
    use rand::Rng;


    fn print_tiling(vec : &Vec<Vec<i32>>) {
        let mut max = -1;
        let n = vec.len();
        for i in vec {
            for j in i {
                if j > &max { max = *j}
            }
        }
        let mut max_num_digits;
        if max == -1 {max_num_digits = 2;}
        else if max == 0 {max_num_digits = 1;}
        else {max_num_digits = 0;}
        while  max != 0 {
            max /= 10;
            max_num_digits += 1;
        }
        max_num_digits += 1;
        for i in 0..n {
            for j in 0..n {
                print!(" [{number:wit$}] ",number = vec[i as usize][j as usize], wit = max_num_digits);
            }
            println!()
        }        
    }
    
    fn tile(vec : &mut Vec<Vec<i32>>, x_1 : i32,y_1 :i32 ,  x_2 : i32, y_2 :i32 , x_3 :i32, y_3:i32, val : i32) {
        vec[x_1 as usize][y_1 as usize] = val;
        vec[x_2 as usize][y_2 as usize] = val;
        vec[x_3 as usize][y_3 as usize] = val; 
    }

    // Solves when only given k
    pub fn solve_with_rand_sqr(k : u32) {
        let n : i32  = 2_i32.pow(k);
        let rand_x : i32 = rand::thread_rng().gen_range(0..n).into();
        let rand_y : i32 = rand::thread_rng().gen_range(0..n).into();
        let mut vec : Vec<Vec<i32>> = vec![vec![0;n as usize];n as usize];
        vec[rand_x as usize][rand_y as usize] = -1;
        solve(n, 0, 0, &mut vec, &mut 0);
        println!("Solved");
        print_tiling(&vec);
    }


    // Solves for when k, x and y it's given
    pub fn solve_with_pre_sqr(k : u32, x : i32, y : i32) {
        let n : i32  = 2_i32.pow(k);
        let mut vec : Vec<Vec<i32>> = vec![vec![0;n as usize];n as usize];
        vec[x as usize][y as usize] = -1;
        solve(n, 0, 0, &mut vec, &mut 0);
        println!("Solved");
        print_tiling(&vec);
    }

    // Solves the problem
    fn solve(n: i32, x :i32, y : i32 ,vec :&mut Vec<Vec<i32>>, n_tile : &mut i32) {

        let mut special_sqr_row : i32 = -1;
        let mut special_sqr_col : i32 = -1;
        let center = n / 2;

        // Base case
        if n == 2 {
            *n_tile = *n_tile + 1;
            for i in 0..n {
                for j in 0..n {
                    if vec[(x+i) as usize][(y+j) as usize ] == 0 {
                        vec[(x+i) as usize][(y+j) as usize] = *n_tile;
                    }
                } 
            }
            return;
        }

        // Find the square that is already fill
        for i in 0..(x+n) {
            for j in 0..(y+n) {
                if vec[i as usize][j as usize] != 0 {
                    special_sqr_row = i;
                    special_sqr_col = j;
                }
            }
        }

        //Puts the L tile.

        // 1st quadrant
        if special_sqr_row < x + center && special_sqr_col < y + center {
            *n_tile = *n_tile +1;
            tile(vec,
                x+ center , y +center - 1,
                x + center, y + center,
                x + center - 1, y + center,
                *n_tile);
        //     vec[(x + n / 2) as usize][(y + (n / 2) - 1) as usize] = *n_tile;
        //     vec[(x + n / 2) as usize][(y + n / 2) as usize] = *n_tile;
        //     vec[(x + n / 2 - 1) as usize][(y + n / 2) as usize] = *n_tile;
        } // 3rd quadrant
        else if special_sqr_row >= x + (n / 2) && special_sqr_col < y +(n / 2) {
            *n_tile = *n_tile +1;
            tile(vec,
                x+ center -1, y +center,
                x + center, y + center,
                x + center - 1, y + center - 1,
                *n_tile);
            // vec[(x + center - 1) as usize][(y + center) as usize] = *n_tile;
            // vec[(x + center) as usize][(y  + center) as usize] = *n_tile;
            // vec[(x + center - 1) as usize][(y + center - 1) as usize] = *n_tile; 

        } // 2nd quadrant
        else if special_sqr_row < x + center && special_sqr_col >= y + center {
            *n_tile = *n_tile +1; 
            tile(vec,
                x+ center, y +center - 1,
                x + center, y + center,
                x + center - 1, y + center - 1,
                *n_tile);
            // vec[(x + center) as usize][(y + center - 1) as usize] = *n_tile;
            // vec[(x + center) as usize][(y + center) as usize] = *n_tile;
            // vec[(x + center - 1) as usize][(y + center - 1) as usize] = *n_tile;

        } // 4th quadrant
        else if special_sqr_row >= x + center && special_sqr_col >= y + center {
            *n_tile = *n_tile + 1;
            tile(vec,
                x+ center -1, y +center,
                x + center, y + center - 1,
                x + center - 1, y + center - 1,
                *n_tile);
            // vec[(x + center - 1) as usize][(y + center) as usize] = *n_tile;
            // vec[(x + center) as usize][(y + center - 1) as usize] = *n_tile;
            // vec[(x + center - 1) as usize][(y + center - 1) as usize] = *n_tile;
        }


        println!("Step {}", n_tile);
        print_tiling(vec);
        // Recurcsion
        solve(center, x, y, vec, n_tile); 
        println!("Step {}", n_tile);
        print_tiling(vec);

        solve(center, x + center, y, vec, n_tile);
        println!("Step {}", n_tile);
        print_tiling(vec);

        solve(center, x, y + center, vec, n_tile);
        println!("Step {}", n_tile);
        print_tiling(vec);

        solve(center, x + center, y + center, vec, n_tile);
        println!("Step {}", n_tile);
        print_tiling(vec);

    }
} 