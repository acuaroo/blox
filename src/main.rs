use std::env;
use colored::*;
use std::path::Path;

// Main function
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let dir = &args[2];

    if query == "h" {
      // "BLOX - Rojo & General Use initializer\n-r: Rojo setup\n-n: Normal setup"

      println!("{} {}", "Blox - Rojo & General Use initializer\n".green().bold(), "r: Rojo setup\n n: Normal setup\n h: Help command".blue().bold());
    } else if query == "r" {
      let b: bool = Path::new(dir).is_dir();

      if b == true {
        println!("{} is a valid dir", dir.green().bold())
        // TODO: Write rojo configurator here
        
      } else {
        println!("{} {} is not a valid dir", "ERR:".red().bold(), dir.yellow().bold())
      }
      
    } else if query == "n" {
      // TODO: Write normal configurator here
    }
}
