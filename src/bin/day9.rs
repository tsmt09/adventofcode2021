extern crate utilities;
use std::collections::HashSet;
use std::path::PathBuf;
use structopt::StructOpt;
use utilities::files::open_file;

#[derive(Debug, StructOpt)]
#[structopt(name = "day8", version = "1.0.0")]
struct Args {
    #[structopt(short = "f", long, parse(from_os_str))]
    file: PathBuf,
}

fn parse_input(input: Vec<String>) -> Vec<Vec<u32>> {
    input
        .iter()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn find_higher_neighbors_recursive(
    input: &Vec<Vec<u32>>,
    basin_coordinates: &mut HashSet<(usize, usize)>,
    row_nr: usize,
    column_nr: usize,
) {
    if input[row_nr][column_nr] == 8 {
        basin_coordinates.insert((row_nr, column_nr));
        return;
    }
    if input[row_nr][column_nr] >= 9 {
        return;
    }
    if row_nr > 0 {
        /* check up */
        if input[row_nr][column_nr] < input[row_nr - 1][column_nr] {
            find_higher_neighbors_recursive(input, basin_coordinates, row_nr - 1, column_nr);
        }
    }
    if row_nr < input.len() - 1 {
        /* check down */
        if input[row_nr][column_nr] < input[row_nr + 1][column_nr] {
            find_higher_neighbors_recursive(input, basin_coordinates, row_nr + 1, column_nr);
        }
    }
    if column_nr > 0 {
        /* check left */
        if input[row_nr][column_nr] < input[row_nr][column_nr - 1] {
            find_higher_neighbors_recursive(input, basin_coordinates, row_nr, column_nr - 1);
        }
    }
    if column_nr < input[row_nr].len() - 1 {
        /* check right */
        if input[row_nr][column_nr] < input[row_nr][column_nr + 1] {
            find_higher_neighbors_recursive(input, basin_coordinates, row_nr, column_nr + 1);
        }
    }
    basin_coordinates.insert((row_nr, column_nr));
}

fn invalidate_coordinates(
    input: &mut Vec<Vec<u32>>,
    basin_coordinates: &mut HashSet<(usize, usize)>,
) {
    for (x, y) in basin_coordinates.iter() {
        input[*x][*y] = 9;
    }
}

fn find_basins(input: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut basins: Vec<u32> = Vec::new();
    let mut work_copy_input = input.clone();
    for (row_number, row) in input.iter().enumerate() {
        for (column_number, _) in row.iter().enumerate() {
            if is_lowpoint(&work_copy_input, row_number, column_number) {
                let mut basin_coordinates: HashSet<(usize, usize)> = HashSet::new();
                find_higher_neighbors_recursive(
                    &work_copy_input,
                    &mut basin_coordinates,
                    row_number,
                    column_number,
                );
                basins.push(basin_coordinates.len() as u32);
                invalidate_coordinates(&mut work_copy_input, &mut basin_coordinates);
            }
        }
    }
    basins.sort_by(|a, b| b.cmp(a));
    basins
}

fn is_lowpoint(input: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let mut trues: Vec<bool> = Vec::new();
    if row > 0 {
        /* check up */
        if input[row][col] < input[row - 1][col] {
            trues.push(true);
        }
    } else {
        trues.push(true);
    }
    if row < input.len() - 1 {
        /* check down */
        if input[row][col] < input[row + 1][col] {
            trues.push(true);
        }
    } else {
        trues.push(true);
    }
    if col > 0 {
        /* check left */
        if input[row][col] < input[row][col - 1] {
            trues.push(true);
        }
    } else {
        trues.push(true);
    }
    if col < input[row].len() - 1 {
        /* check right */
        if input[row][col] < input[row][col + 1] {
            trues.push(true);
        }
    } else {
        trues.push(true);
    }
    if trues.len() == 4 {
        return true;
    }
    return false;
}

fn find_lowpoints(input: Vec<Vec<u32>>) -> Vec<u32> {
    let mut lowpoints: Vec<u32> = Vec::new();
    for (row_number, row) in input.iter().enumerate() {
        for (column_number, _) in row.iter().enumerate() {
            if is_lowpoint(&input, row_number, column_number) {
                lowpoints.push(input[row_number][column_number] + 1);
            }
        }
    }
    lowpoints
}

fn main() {
    let args = Args::from_args();
    let vec_str = open_file(args.file);
    let vec_map = parse_input(vec_str);
    println!(
        "assign1 result: {}",
        find_lowpoints(vec_map.clone())
            .clone()
            .into_iter()
            .reduce(|a, b| a + b)
            .unwrap()
    );
    let basins = find_basins(&mut vec_map.clone());
    dbg!(&basins);
    println!("assign2 result: {}", basins[0] * basins[1] * basins[2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signals_test() {
        let testvec: Vec<String> = vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ];
        let testmap = parse_input(testvec);
        let lowpoints = find_lowpoints(testmap.clone());
        let basins = find_basins(&testmap);
        assert_eq!(
            15,
            lowpoints.clone().into_iter().reduce(|a, b| a + b).unwrap()
        );
        assert_eq!(1134, basins[0] * basins[1] * basins[2]);
    }
}
