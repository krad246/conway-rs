use rayon::prelude::*;

use std::sync::{Arc, OnceLock, RwLock};
use std::thread::available_parallelism;

fn main() {
    let result = game_of_life(vec![vec![Status::Live, Status::Live, Status::Live], vec![Status::Dead, Status::Live, Status::Dead], vec![Status::Live, Status::Live, Status::Dead]], 3);
    println!("{result:?}");
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum Status {
    Live,
    Dead,
}

fn game_of_life(input_grid: Vec<Vec<Status>>, iterations: usize) -> Vec<Vec<Status>> {
    let m = input_grid.len();
    let n = input_grid[0].len();
    let chunk_size = m / 8;
    let output: Vec<Vec<OnceLock<Status>>> = vec![];
    todo!()
}

type Board = Vec<Vec<Arc<RwLock<Status>>>>;

fn get_next_state(grid: &Board, r: usize, c: usize) -> Status {
    let m = grid.len();
    let n = grid[0].len();
    let mut num_neighbors_alive  = 0;
    let cur_state = grid[r][c].read().expect("A writer has been poisoned");
    for i in r.checked_sub(1).unwrap_or(0)..std::cmp::max(r + 1, m) {
        for j in c.checked_sub(1).unwrap_or(0)..std::cmp::max(c + 1, n) {
            let neighbor_status = grid[i][j].read().expect("Lock was poisoned");
            if *neighbor_status == Status::Live {
                num_neighbors_alive += 1;
            }
        }
    }
    if *cur_state == Status::Live {
        num_neighbors_alive -= 1;
    }

    if num_neighbors_alive == 2 || num_neighbors_alive == 3 {
        return Status::Live;
    }

    if num_neighbors_alive == 3 && *cur_state == Status::Dead {
        return Status::Live;
    }
    Status::Dead
}

fn game_of_life_tarun(input_grid: Vec<Vec<Status>>, iterations: usize) {
    let m = input_grid.len();
    let n = input_grid[0].len();

    let available_workers = match available_parallelism() {
        Ok(num) => num.get(),
        Err(_) => 8,
    };

    let chunk_size = m / available_workers;
    let mut cur_grid: Board = input_grid
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|elem| Arc::new(RwLock::new(elem)))
                .collect()
        })
        .collect();

    let mut next_grid: Board = vec![];

    for _ in 0..iterations {
        cur_grid
            .par_chunks(chunk_size)
            .enumerate()
            .for_each(|(chunk_idx, _chunk)| {
                let start_idx = chunk_idx * chunk_size;
                for r in start_idx..start_idx + chunk_size {
                    for c in 0..n {
                        let next_state = get_next_state(&cur_grid, r, c);
                        let mut writer = next_grid[r][c].write().expect("Lock is poisoned!");
                        *writer = next_state;
                    }
                }
            });
        std::mem::swap(&mut cur_grid, &mut next_grid);
    }

    dbg!(cur_grid);

}
