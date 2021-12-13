use regex::Regex;
use std::io;
use std::io::BufRead;

enum CommandType {
    Forward,
    Up,
    Down,
    Unknown,
}

struct Command {
    command: CommandType,
    steps: i32,
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

struct Boat {
    position: Position,
}

trait Movable {
    fn do_command(&mut self, cmd: Command);
    fn forward(&mut self, steps: i32);
    fn up(&mut self, steps: i32);
    fn down(&mut self, steps: i32);
}

impl Movable for Boat {
    fn do_command(&mut self, cmd: Command) {
        match cmd.command {
            CommandType::Forward => self.forward(cmd.steps),
            CommandType::Up => self.up(cmd.steps),
            CommandType::Down => self.down(cmd.steps),
            CommandType::Unknown => panic!("command unknown!"),
        }
    }
    fn forward(&mut self, steps: i32) {
        self.position.horizontal += steps;
        self.position.depth += self.position.aim * steps;
    }
    fn up(&mut self, steps: i32) {
        self.position.aim -= steps;
    }
    fn down(&mut self, steps: i32) {
        self.position.aim += steps;
    }
}

fn parse_commands(vec_str: Vec<String>) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    /* parse line into commands */
    let re = Regex::new(r"^(\w+)\s(\d+)$").unwrap();
    for str_line in vec_str {
        match re.captures(&str_line) {
            Some(captures) => {
                let parsedcmd: CommandType = match &captures[1] {
                    "forward" => CommandType::Forward,
                    "up" => CommandType::Up,
                    "down" => CommandType::Down,
                    _ => CommandType::Unknown,
                };
                commands.push(Command {
                    command: parsedcmd,
                    steps: captures[2].parse().unwrap(),
                });
            }
            None => panic!("cannot regex parse line: {}", str_line),
        }
    }
    commands
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut vec_str: Vec<String> = Vec::new();
    /* parse line into commands */
    for line in lines {
        match line {
            Ok(str_line) => vec_str.push(str_line),
            Err(err) => {
                println!("error reading stdin: {}", err)
            }
        }
    }
    let commands = parse_commands(vec_str);
    /* create new boat */
    let mut myboat: Boat = {
        Boat {
            position: {
                Position {
                    horizontal: 0,
                    depth: 0,
                    aim: 0,
                }
            },
        }
    };
    /* let it ride */
    for command in commands {
        myboat.do_command(command);
    }
    println!(
        "My Boat coordinates:\nhorizontal: {}\ndepth: {}\nmultiplied: {}",
        myboat.position.horizontal,
        myboat.position.depth,
        myboat.position.horizontal * myboat.position.depth
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boat_test() {
        let testvec: Vec<String> = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let commands = parse_commands(testvec);
        let mut myboat: Boat = {
            Boat {
                position: {
                    Position {
                        horizontal: 0,
                        depth: 0,
                        aim: 0,
                    }
                },
            }
        };
        for command in commands {
            myboat.do_command(command);
        }
        assert_eq!(15, myboat.position.horizontal);
        assert_eq!(60, myboat.position.depth);
    }
}
