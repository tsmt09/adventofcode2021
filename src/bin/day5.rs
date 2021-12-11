extern crate utilities;
use std::path::PathBuf;
use structopt::StructOpt;
use regex::Regex;
use std::cmp::max;

#[derive(StructOpt, Debug)]
#[structopt(name = "day3", version = "1.0.0")]
struct Args {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    ///#[structopt(short, parse(from_occurrences))]
    ///verbose: u8,
    /// Files to process
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize
}
#[derive(Debug)]
struct Command {
    from: Coordinate,
    to: Coordinate
}
#[derive(Debug)]
struct FieldDefinition {
    commands: Vec<Command>,
    max_x: usize,
    max_y: usize
}

#[derive(Debug)]
struct Field {
    definition: FieldDefinition,
    field: Vec<Vec<i8>>
}

fn parse_field_definition(string: Vec<String>) -> FieldDefinition {
    let mut field_definition = FieldDefinition {
        commands: Vec::new(),
        max_x: 0,
        max_y: 0
    };
    let re = Regex::new(r"^(\d+),(\d+)\s->\s(\d+),(\d+)$").unwrap();
    for str_line in string {
        match re.captures(&str_line) {
            Some(captures) => {
                let mut cmd: Command = Command {
                    from: Coordinate { x:0 , y:0 },
                    to: Coordinate { x:0 , y:0 }
                };
                /* parse values */
                cmd.from.x = captures[1].parse::<usize>().unwrap();
                cmd.from.y = captures[2].parse::<usize>().unwrap(); 
                cmd.to.x = captures[3].parse::<usize>().unwrap();
                cmd.to.y = captures[4].parse::<usize>().unwrap();
                /* find max values */
                field_definition.max_x = max(field_definition.max_x, cmd.from.x);
                field_definition.max_x = max(field_definition.max_x, cmd.to.x);
                field_definition.max_y = max(field_definition.max_y, cmd.from.y);
                field_definition.max_y = max(field_definition.max_y, cmd.to.y);
                /* push data */
                field_definition.commands.push(cmd);
            }
            None => panic!("cannot regex parse line: {}", str_line)
        }
    }
    field_definition
}

fn build_field(field_definition: FieldDefinition) -> Field {
    let field = Field {
        field: vec![
            vec![
                0; 
                field_definition.max_x + 1
            ]; 
            field_definition.max_y + 1
        ],
        definition: field_definition
    };
    field
}

fn print_field(field: &Field) {
    for line in &field.field {
        println!("{:?}", line);
    }
}

fn deploy_commands(field: &mut Field) {
    for command in &field.definition.commands {
        let mut left_right = true;
        if command.from.x > command.to.x {
            left_right = false;
        }
        let mut up_down = true;
        if command.from.y > command.to.y {
            up_down = false;
        }
        if command.from.x == command.to.x {
            // println!("horizontal: {},{} -> {},{}", command.from.x, command.from.y, command.to.x, command.to.y);
            /* horizontal */
            let index_range;
            if up_down {
                index_range = (command.from.y)..=(command.to.y);
            } else {
                index_range = (command.to.y)..=(command.from.y);
            }
            for index in index_range {
                field.field[index][command.from.x] += 1;
            }
        } else if command.from.y == command.to.y {
            // println!("vertical: {},{} -> {},{}", command.from.x, command.from.y, command.to.x, command.to.y);
            /* vertical */
            let index_range;
            if left_right {
                index_range = (command.from.x)..=(command.to.x);
            } else {
                index_range = (command.to.x)..=(command.from.x);
            }
            for index in index_range {
                field.field[command.from.y][index] += 1;
            }
        } else {
            // println!("cannot handle: {},{} -> {},{}", command.from.x, command.from.y, command.to.x, command.to.y);
            let mut start_x = command.from.x;
            let target_x = command.to.x;
            let mut start_y = command.from.y;
            let target_y = command.to.y;
            loop {
                field.field[start_y][start_x] += 1;
                if start_y == target_y && start_x == target_x {
                    break;
                }
                if up_down && start_y != target_y {
                    start_y += 1;
                } else {
                    start_y -= 1;
                }
                if left_right && start_x != target_x {
                    start_x += 1;
                } else {
                    start_x -= 1;
                }
            }
        }
    }
}

fn get_score(field: &Field) -> i32 {
    let mut count: i32 = 0;
    for vertical in &field.field {
        for horizontal in vertical {
            if *horizontal >= 2 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let args = Args::from_args();
    let vec_str = utilities::files::open_file(args.file);
    let field_definition = parse_field_definition(vec_str);
    let mut field = build_field(field_definition);
    deploy_commands(&mut field);
    println!("The score is: {}", get_score(&field));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_test() {
        let testvec: Vec<String> = vec![
            "0,9 -> 5,9".to_string(),
            "8,0 -> 0,8".to_string(),
            "9,4 -> 3,4".to_string(),
            "2,2 -> 2,1".to_string(),
            "7,0 -> 7,4".to_string(),
            "6,4 -> 2,0".to_string(),
            "0,9 -> 2,9".to_string(),
            "3,4 -> 1,4".to_string(),
            "0,0 -> 8,8".to_string(),
            "5,5 -> 8,2".to_string(),
        ];
        let field_definition = parse_field_definition(testvec);
        let mut field = build_field(field_definition);
        deploy_commands(&mut field);
        print_field(&field);
        assert_eq!(get_score(&field), 12);
    }
}