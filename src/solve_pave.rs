


pub mod solve_pave {
    use rand::Rng;
    

    pub fn solve_with_rand_sqr(k : u32) {
        let n : i32  = 2_i32.pow(k);
        let rand_x : i32 = rand::thread_rng().gen_range(0..n).into();
        let rand_y : i32 = rand::thread_rng().gen_range(0..n).into();
        let mut vec : Vec<Vec<i32>> = vec![vec![0;n as usize];n as usize];
        vec[rand_x as usize][rand_y as usize] = -1;
        solve(n, 0, 0, &mut vec, &mut 0);
        dbg!(vec);
    }

    pub fn solve_with_pre_sqr(k : u32, x : i32, y : i32) {
        let n : i32  = 2_i32.pow(k);
        let mut vec : Vec<Vec<i32>> = vec![vec![0;n as usize];n as usize];
        vec[x as usize][y as usize] = -1;
        solve(n, 0, 0, &mut vec, &mut 0);
        dbg!(vec);
    }

    fn solve(n: i32, x :i32, y : i32 ,vec :&mut Vec<Vec<i32>>, n_tile : &mut i32) {
        let mut row : i32 = -1;
        let mut col : i32 = -1;

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
        for i in 0..(x+n) {
            for j in 0..(y+n) {
                if vec[i as usize][j as usize] != 0 {
                    row = i;
                    col = j;
                }
            }
        }


        if row < x + n / 2 && col < y + n / 2 {
            *n_tile = *n_tile +1;
            vec[(x + n / 2) as usize][(y + (n / 2) - 1) as usize] = *n_tile;
            vec[(x + n / 2) as usize][(y + n / 2) as usize] = *n_tile;
            vec[(x + n / 2 - 1) as usize][(y + n / 2) as usize] = *n_tile;
        } else if row >= x + n / 2 && col < y + n / 2 {
            *n_tile = *n_tile +1;
            vec[(x + (n / 2) - 1) as usize][(y + (n / 2)) as usize] = *n_tile;
            vec[(x + (n / 2)) as usize][(y + n / 2) as usize] = *n_tile;
            vec[(x + (n / 2) - 1) as usize][(y + (n / 2) - 1) as usize] = *n_tile; 
        } else if row < x + n / 2 && col >= y + n / 2 {
            *n_tile = *n_tile +1; 
            vec[(x + n / 2) as usize][(y + (n / 2) - 1) as usize] = *n_tile;
            vec[(x + n / 2) as usize][(y + n / 2) as usize] = *n_tile;
            vec[(x + n / 2 - 1) as usize][(y + n / 2 - 1) as usize] = *n_tile;
        } else if row >= x + n / 2 && col >= y + n / 2 {
            *n_tile = *n_tile + 1;
            vec[(x + (n / 2) - 1) as usize][(y + (n / 2)) as usize] = *n_tile;
            vec[(x + (n / 2)) as usize][(y + (n / 2) - 1) as usize] = *n_tile;
            vec[(x + (n / 2) - 1) as usize][(y + (n / 2) - 1) as usize] = *n_tile;
        }



        solve(n / 2, x, y + n / 2, vec, n_tile);
        solve(n / 2, x, y, vec, n_tile);
        solve(n / 2, x + n / 2, y, vec, n_tile);
        solve(n / 2, x + n / 2, y + n / 2, vec, n_tile);

    }
} 