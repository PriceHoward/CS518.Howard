use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write, BufWriter};
// Documentation for std::io https://doc.rust-lang.org/std/io/index.html


fn file_trimmer(file_name: &str) -> Result<(), io::Error> {
    let i_file = File::open(file_name)
                .map_err(|e| { eprintln!("!COULD NOT OPEN INPUT FILE!: {}", e); e})?;
    let o_file = File::create(format!("corrected_{}", file_name))
                .map_err(|e| { eprintln!("!COULD NOT OPEN OUTPUT FILE!: {}", e); e})?;

    let file_reader = BufReader::new(i_file);
    let mut file_writer = BufWriter::new(o_file);
    let mut line_counter = 1;

    for line in file_reader.lines(){
        let line = line?;
        let trimmed = line.trim();

        if !trimmed.is_empty() {
            writeln!(file_writer, "{}: {}", line_counter, trimmed)?;
            line_counter+=1;
        }
    }

    Ok(())
}




fn main() {
    if let Err(e) = file_trimmer("CoolText.txt") {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
