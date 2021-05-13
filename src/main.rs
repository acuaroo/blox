use std::env;
use colored::*;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

static GITIGNORE: &str = ".DS_Store\ntests/";


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
        // let c_dir = Path::new(dir);
        // let git_dir = dir.concat("");
        // let gitig = Path::new("{}");
        // let display = gitig.display();

        // println!("{} is a valid dir", dir.green().bold());

        // let mut file = match File::create(&gitig) {
        //   Err(why) => panic!("Couldn't create {}: {}", display, why),
        //   Ok(file) => file,
        // };
  
        // // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        // match file.write_all(GITIGNORE.as_bytes()) {
        //     Err(why) => panic!("Couldn't write to {}: {}", display, why),
        //     Ok(_) => println!("Successfully wrote to {}", display),
        // }
        
      } else {
        println!("{} {} is not a valid dir", "ERR:".red().bold(), dir.yellow().bold())
      }
      
    } else if query == "n" {
      // TODO: Write normal configurator here
    }
}
