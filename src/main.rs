use std::time::{Instant};
use colored::*;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let now = Instant::now();

    File::create("README.md")?;
    let mut gitignore = File::create(".gitignore")?;
    gitignore.write_all(b".DS_Store")?;

    let mut license = File::create("LICENSE.md")?;
    license.write_all(b"Copyright <YEAR> <COPYRIGHT HOLDER>\nPermission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:\nThe above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.")?;

    println!("{} {} {}", "Initialized in".green().bold(), now.elapsed().as_millis(), "milli-seconds");
    Ok(())
}