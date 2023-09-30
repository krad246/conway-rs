use rayon::prelude::*;
use std::sync::{Arc, OnceLock, RwLock};
use std::thread::available_parallelism;
fn main() {
    let result = game_of_life(vec![vec![]], 3);
    println!("{result:?}");
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Live,
    Dead,
}

fn num_live_neighbors(cell: Status, row: usize, col: usize) -> usize {
    todo!()
}

fn determine_next_state(status: Status, num_live_neighbors: usize) -> Status {
    if status == Status::Live && (num_live_neighbors == 2 || num_live_neighbors == 3) {
        return Status::Live;
    }

    if status == Status::Dead && num_live_neighbors == 3 {
        return Status::Live;
    }

    Status::Dead
}

fn game_of_life(input_grid: Vec<Vec<Status>>, iterations: usize) -> Vec<Vec<Status>> {
    let m = input_grid.len();
    let n = input_grid[0].len();
    let chunk_size = m / 8;
    //let mut cur_grid = input_grid;
    let output: Vec<Vec<OnceLock<Status>>> = vec![];

    /* let compute = |input: &[&[Status]], &mut output|-> {
        for (y, &row) in input.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                output[y][x] = determine_next_state(*cell, num_live_neighbors(x, y))
            }
        }
    }; */

    /* cur_grid = input_grid
    .par_chunks(chunk_size)
    .map( {
        chunk.iter().map(|row: &Vec<Status>| {
            chunk
            row.iter().enumerate()
                .map(|cell: &Status| num_live_neighbors(cell, row_num, col_num))
                .collect()
        })
    })
    .collect(); */

    todo!()
}
type Board = Vec<Vec<Arc<RwLock<Status>>>>;

fn get_next_state(grid: &Board, r: usize, c: usize) -> Status {
    let cur_state = grid[r][c].read().expect("A writer has been poisoned");
    todo!()
}

fn game_of_life_tarun(input_grid: Vec<Vec<Status>>, iterations: usize) -> Vec<Vec<Status>> {
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
    }

    std::mem::swap(&mut cur_grid, &mut next_grid);

    todo!()
}
