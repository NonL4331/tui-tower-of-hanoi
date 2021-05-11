use std::{env, fmt, process};

use crossterm::{
    cursor::{DisableBlinking, Hide, MoveTo},
    execute,
    terminal::{Clear, ClearType::All},
};

const DELAY_MS: u64 = 100;
const TOWER_SIZE: u32 = 6;

enum Column {
    First,
    Second,
    Third,
}

enum LogLevel {
    None,
    Minimal,
    All,
}

impl Column {
    pub fn get_value(&self) -> usize {
        match self {
            Column::First => 0,
            Column::Second => 1,
            Column::Third => 2,
        }
    }
}

struct Tower {
    height: u32,
    print_delay: u32,
    state: [Vec<u32>; 3],
}

impl fmt::Display for Tower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_height = self.height as usize;
        let mut printed_str = String::new();
        for i in (0..max_height).rev() {
            printed_str.push_str(&self.get_layer_string(i).to_owned());
        }
        write!(f, "{}", printed_str)
    }
}

impl Tower {
    pub fn new(height: u32, delay: u32) -> Self {
        let mut starting_col = Vec::new();
        for i in 0..height {
            starting_col.push(height - i);
        }
        let state = [starting_col, Vec::new(), Vec::new()];
        Tower {
            height,
            print_delay: delay,
            state,
        }
    }

    pub fn solve(&mut self) {
        self.move_stack(self.height, &Column::First, &Column::Third, &Column::Second);
    }

    fn move_peg(&mut self, from: &Column, to: &Column) {
        let from = from.get_value();
        let to = to.get_value();

        let val = self.state[from].pop().unwrap();
        self.state[to].push(val);
    }

    fn move_stack(&mut self, size: u32, start_col: &Column, target_col: &Column, aux_col: &Column) {
        if size > 0 {
            self.move_stack(size - 1, start_col, aux_col, target_col);
            self.move_and_print(start_col, target_col);
            self.move_stack(size - 1, aux_col, target_col, start_col);
        }
    }

    fn move_and_print(&mut self, from: &Column, to: &Column) {
        self.move_peg(from, to);

        execute!(std::io::stdout(), Clear(All), MoveTo(0, 0)).unwrap();
        println!("{}", self);
        std::thread::sleep(std::time::Duration::from_millis(self.print_delay as u64));
    }

    fn get_layer_string(&self, layer: usize) -> String {
        let mut layer_string = String::new();

        let box_width = (self.height * 2 + 6) as usize;

        for col in 0..3 {
            match self.state[col].get(layer) {
                Some(value) => {
                    let peg_string_length = (value * 2) as usize;
                    let pad_spaces = (box_width - peg_string_length) / 2;

                    layer_string.push_str(&" ".to_string().repeat(pad_spaces).to_owned());

                    layer_string.push_str(&"■".to_string().repeat(peg_string_length).to_owned());

                    layer_string.push_str(&" ".to_string().repeat(pad_spaces).to_owned());
                }
                None => layer_string.push_str(&" ".to_string().repeat(box_width).to_owned()),
            }
        }
        layer_string.push_str(&"\n".to_string().to_owned());
        layer_string
    }
}

fn main() {
    execute!(std::io::stdout(), DisableBlinking, Hide,).unwrap();
    let args: Vec<String> = env::args().collect();
    let (delay, height, loglevel) = get_parameters(args);
    let mut tower = Tower::new(height, delay);
    println!("{}", tower);
    tower.solve();
    match loglevel {
        LogLevel::None => {}
        LogLevel::Minimal => {
            println!("Completed in {} moves", 2u32.pow(tower.height) - 1);
        }
        LogLevel::All => {
            println!("Completed in {} moves", 2u32.pow(tower.height) - 1);
            println!("Tower height: {} pegs", tower.height);
            println!("Delay: ~{}ms", delay);
        }
    }
}

fn get_parameters(args: Vec<String>) -> (u32, u32, LogLevel) {
    let (mut delay, mut height) = (DELAY_MS as u32, TOWER_SIZE as u32);
    let mut log = LogLevel::Minimal;
    if args.len() < 2 {
        return (delay, height, log);
    }
    for arg_i in (0..(args.len() / 2)).map(|i| i * 2 + 1) {
        match args.get(arg_i) {
            Some(arg) => match &arg[..] {
                "-H" => {
                    display_help();
                    process::exit(0);
                }
                "--help" => {
                    display_help();
                    process::exit(0);
                }
                "-D" => {
                    delay = get_delay(&args, arg_i + 1);
                }
                "--delay" => {
                    delay = get_delay(&args, arg_i + 1);
                }
                "-N" => {
                    height = get_height(&args, arg_i + 1);
                }
                "--height" => {
                    height = get_height(&args, arg_i + 1);
                }
                "-L" => {
                    log = get_log(&args, arg_i + 1);
                }
                "--loglevel" => {
                    log = get_log(&args, arg_i + 1);
                }
                _ => {
                    println!("Unknown argument \"{}\"!", args[arg_i]);
                    println!("Do -H or --help for more informatin.");
                    process::exit(0);
                }
            },
            None => break,
        }
    }

    (delay, height, log)
}

fn display_help() {
    println!("Usage: hanoi [OPTION...]");
    println!("Solves the tower of hanoi in your terminal!\n");
    println!("Arguments:");
    println!("-H, --help");
    println!("\t Displays help");
    println!("-D [value], --delay [value]");
    println!("\tSets the delay between peg moves; [value] is a positive integer in milliseconds.");
    println!("\tDefault value of 100");
    println!("-N [value], --height [value]");
    println!("\tSets the height of the tower; [value] is a positive integer.");
    println!("-L [value], --loglevel [value]");
    println!("\tSets the loglevel for the program (not capital sensitive).");
    println!("\tPossible values are:");
    println!("\t\t[None] - print nothing");
    println!("\t\t[Minimal] - only print moves taken");
    println!("\t\t[All] - print both moves taken, tower height and print delay");
    println!("\tDefault value of [Minimal]");
}

fn get_delay(args: &Vec<String>, index: usize) -> u32 {
    match args.get(index) {
        None => {
            println!("Please specify a value for delay!");
            println!("Do -H or --help for more informatin.");
            process::exit(0);
        }
        Some(string) => match string.parse::<u32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Please specify a valid value for delay!");
                println!("Do -H or --help for more informatin.");
                process::exit(0);
            }
        },
    }
}

fn get_height(args: &Vec<String>, index: usize) -> u32 {
    match args.get(index) {
        None => {
            println!("Please specify a value for height!");
            println!("Do -H or --help for more informatin.");
            process::exit(0);
        }
        Some(string) => match string.parse::<u32>() {
            Ok(val) => val,
            Err(_) => {
                println!("{} is not a valid value for height!", string);
                println!("Please specify a valid value for height!");
                println!("Do -H or --help for more informatin.");
                process::exit(0);
            }
        },
    }
}

fn get_log(args: &Vec<String>, index: usize) -> LogLevel {
    match args.get(index) {
        None => {
            println!("Please specify a value for log level!");
            println!("Do -H or --help for more informatin.");
            process::exit(0);
        }
        Some(string) => match &string.to_lowercase()[..] {
            "all" => LogLevel::All,
            "minimal" => LogLevel::Minimal,
            "none" => LogLevel::None,
            _ => {
                println!("{} is not a valid value for log level!", string);
                println!("Please specify a valid value for log level!");
                println!("Do -H or --help for more informatin.");
                process::exit(0);
            }
        },
    }
}
