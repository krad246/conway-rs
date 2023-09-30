use rayon::prelude::*;
use std::sync::OnceLock;
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
    if status == Status::Live && (num_live_neighbors == 2 || num_live_neighbors == 3){
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

    let compute = |input: &[&[Status]], &mut output|-> {
        for (y, &row) in input.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                output[y][x] = determine_next_state(*cell, num_live_neighbors(x, y))
            }
        }
    }

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

fn game_of_life_tarun(input_grid: Vec<Vec<Status>>, iterations: usize) -> Vec<Vec<Status>> {
    let m = input_grid.len();
    let n = input_grid[0].len();
    let chunk_size = m / 8;
    let mut cur_grid = input_grid;

    /* cur_grid = (0..m).par_chunks(chunk_size)
    .map(|chunk| {
        chunk.iter().map(|row| {
            chunk
            row.iter().enumerate()
                .map(|cell: &Status| num_live_neighbors(cell, row_num, col_num))
                .collect()
        })
    })
    .collect(); */

    for r in 0..m {
        for c in 0..n {
            let cur_elem_status = &cur_grid[r][c];
            let num_live_neighbors = num_live_neighbors(cur_elem_status, r, c);

        }
    }

    todo!()
}
