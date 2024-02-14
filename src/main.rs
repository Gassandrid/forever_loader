mod event_options;
use event_options::EventOptions;
use std::env;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let event_options = EventOptions::new();

    let count = args[1].parse::<i32>().unwrap();

    println!("Generating {} sections", count);
    event_options.new_section();
    println!("\n");
    
    for _ in 0..count {
        event_options.generateSection();
    }
}
