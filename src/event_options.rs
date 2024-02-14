use rand::Rng;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use rand::prelude::*;
use std::io;
use std::thread;
use std::time::Duration;
use colored::*;

pub struct EventOptions {
    pub loader_lines: Vec<String>,
    pub long_lines: Vec<String>,
    pub input_lines: Vec<String>,
}

impl EventOptions {
    pub fn new() -> EventOptions {
        let loader_filename = "/usr/local/bin/loading_bar.txt".to_string();
        let long_filename = "/usr/local/bin/long_messages.txt".to_string();
        let input_filename = "/usr/local/bin/cainput_options.txt".to_string();

        let loader_file = File::open(&loader_filename).unwrap();
        let loader_lines: Vec<String> = BufReader::new(loader_file).lines().map(|line| line.unwrap()).collect();

        let long_file = File::open(&long_filename).unwrap();
        let long_lines: Vec<String> = io::BufReader::new(long_file).lines().map(|line| line.unwrap()).collect();

        let input_file = File::open(&input_filename).unwrap();
        let input_lines: Vec<String> = io::BufReader::new(input_file).lines().map(|line| line.unwrap()).collect();

        EventOptions {loader_lines, long_lines, input_lines}
    }

    pub fn new_section(&self) {
        println!("-----------------------------------------------");
    }
    pub fn generateSection(&self) {
        // will call a random option function within this object, run that function for a random amount of times
        // number of times functions runs should depend on the event type
        let mut rng = rand::thread_rng();
        let event_type = rng.gen_range(0..3);

        let mut range: i8 = 0;

        if event_type == 0 {
            range = 10;
        } else if event_type == 1 {
            range = 5;
        } else if event_type == 2 {
            range = 3;
        }

        let mut event_count = rng.gen_range(1..range);

        for _ in 0..event_count {
            if event_type == 0 {
                self.Loading_bar();
            } else if event_type == 1 {
                self.Long_message();
            } else if event_type == 2 {
                self.Input_Options();
            }
        }

        self.new_section();
        println!("\n");
    }
    pub fn Loading_bar(&self) {
        //get random message from loader_lines vector
        let message = &self.loader_lines[rand::thread_rng().gen_range(0..self.loader_lines.len())];

        //get random speed for the loader to run at
        let mut rng = rand::thread_rng();
        let mut random_speed = rng.gen_range(0.001..0.1);

        //get random length for the loader to run for
        let mut rng = rand::thread_rng();
        let random_length = rng.gen_range(10..115);

        //set up the loading bar
        let mut i = 0;
        let mut percent: f64 = 0.0;
        let percent_increment: f64 = 100.0 / random_length as f64;

        //set up the loading message
        let final_word = message.chars().collect::<Vec<_>>();
        let mut scrambled: Vec<char> = final_word.clone();
        scrambled.shuffle(&mut rand::thread_rng());
        let label_speed = &random_speed/4.0;

        //generate the message
        for i in 0..final_word.len() {
            while final_word[i] != scrambled[i] {
                let temp = scrambled[i];
                scrambled.remove(i);
                scrambled.push(temp);
                print!("\r{}", scrambled.iter().collect::<String>());
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_secs_f64(label_speed));
            }
        }
        print!("\n");

        //generate the loading bar
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        while i <= random_length {
            print!("\r[");
            for _ in 0..i {
                print!("█");
            }
            for _ in 0..random_length - i {
                print!("░");
            }
            let rounded_percent = format!("{:.2}", percent); // Round the percent value to 2 decimal places using format!
            print!("] {}% ", rounded_percent); // Print the rounded percent value with 2 decimal places
            percent += percent_increment;
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs_f64(random_speed));
            i += 1;

            //randomly defide if the spped SHOULD be changed
            let mut rng = rand::thread_rng();
            if rng.gen_range(0..100) > 80 {
                random_speed = rng.gen_range(0.001..0.1);
            }
        }
        println!("\n");
    }
    pub fn Long_message(&self) {
        //get random message from long_lines vector
        let message = &self.long_lines[rand::thread_rng().gen_range(0..self.long_lines.len())];

        //get random speed for the loader to run at
        let mut rng = rand::thread_rng();
        let mut random_speed = rng.gen_range(0.001..0.01);

        //set up the long message
        let final_word = message.chars().collect::<Vec<_>>();
        let mut scrambled: Vec<char> = final_word.clone();
        scrambled.shuffle(&mut rand::thread_rng());
        let label_speed = &random_speed / 4.0;

        // split the long message into separate variables if it is too long for a line
        let max_line_length = 80; // maximum length for a line
        let mut lines: Vec<String> = Vec::new();
        let mut current_line = String::new();
        for word in message.split_whitespace() {
            if current_line.len() + word.len() + 1 <= max_line_length {
                current_line.push_str(word);
                current_line.push(' ');
            } else {
                lines.push(current_line.trim().to_string());
                current_line = word.to_string();
                current_line.push(' ');
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line.trim().to_string());
        }

        // run a separate message generator for each line
        for line in lines {
            let line_final_word = line.chars().collect::<Vec<_>>();
            let mut line_scrambled: Vec<char> = line_final_word.clone();
            line_scrambled.shuffle(&mut rand::thread_rng());
            let line_label_speed = &random_speed / 4.0;

            for i in 0..line_final_word.len() {
                while line_final_word[i] != line_scrambled[i] {
                    let temp = line_scrambled[i];
                    line_scrambled.remove(i);
                    line_scrambled.push(temp);
                    print!("\r{}", line_scrambled.iter().collect::<String>().color(Self::random_color()));
                    io::stdout().flush().unwrap();
                    thread::sleep(Duration::from_secs_f64(line_label_speed));
                }
            }
            print!("\n");
        }
        print!("\n");
    }
    fn random_color() -> colored::Color {
        let colors = [
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::White,
        ];
        let mut rng = rand::thread_rng();
        *colors.choose(&mut rng).unwrap()
    }
    pub fn Input_Options(&self) {
        
        //get random message from input_lines vector
        let message = &self.input_lines[rand::thread_rng().gen_range(0..self.input_lines.len())];

        //get random speed for the loader to run at
        let mut rng = rand::thread_rng();
        let mut random_speed = rng.gen_range(0.001..0.1);

        //set up the input message
        let final_word = message.chars().collect::<Vec<_>>();
        let mut scrambled: Vec<char> = final_word.clone();
        scrambled.shuffle(&mut rand::thread_rng());
        let label_speed = &random_speed / 4.0;

        //generate the message
        for i in 0..final_word.len() {
            while final_word[i] != scrambled[i] {
                let temp = scrambled[i];
                scrambled.remove(i);
                scrambled.push(temp);
                print!("\r{}", scrambled.iter().collect::<String>());
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_secs_f64(label_speed));
            }
        }
        print!("\n");

        //get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("You entered: {}", input);

        if input.trim() == "y" {
            println!("{}","proceeding with installation...\n".green()); 
        } else if input.trim() == "n" {
            println!("{}","cancelling installation...\n".red());
        } else {
            println!("{}","invalid input, cancelling installation...\n".yellow());
        }
    }
}