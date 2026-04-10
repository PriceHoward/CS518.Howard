/*This was taken form the notes.

The question confused me. As you said to implement it in Rust yet we already have in the notes.
The DFD is in the same folder as this. If that was all you wanted then I will delete this file.
If you did want a different implementation then the one from the notes or if you wwant it ina  different language. 
Just let me know. I can do that.
*/

use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    x: i32,
    cycles: Vec<i32>,
}

impl State {
    fn new() -> Self {
        Self {
            x: 1,
            cycles: Vec::new(),
        }
    }
}

fn noop(state: &State) -> State {
    let mut cycles = state.cycles.clone();
    cycles.push(state.x);

    State {
        x: state.x,
        cycles,
    }
}

fn addx(n: i32, state: &State) -> State {
    let mut cycles = state.cycles.clone();
    cycles.push(state.x);
    cycles.push(state.x);

    State {
        x: state.x + n,
        cycles,
    }
}

fn execute(state: State, lines: &[&str]) -> Result<State, String> {
    lines.iter().try_fold(state, |state, line| {
        let line = line.trim();

        if line == "noop" {
            Ok(noop(&state))
        } else if let Some(arg) = line.strip_prefix("addx ") {
            let n = arg.parse::<i32>().map_err(|_| format!("invalid addx argument: {line}"))?;
            Ok(addx(n, &state))
        } else {
            Err(format!("TILT: unrecognized instruction: {line}"))
        }
    })
}

fn execute_file(file_name: &str) -> Result<Vec<i32>, String> {
    let contents = fs
        ::read_to_string(file_name)
        .map_err(|e| format!("failed to read {file_name}: {e}"))?;

    let lines: Vec<&str> = contents.lines().collect();
    let ending_state = execute(State::new(), &lines)?;
    Ok(ending_state.cycles)
}

fn render_cycles(cycles: &[i32]) -> Vec<String> {
    let screen: String = cycles
        .iter()
        .enumerate()
        .map(|(t, &x)| {
            let pos = (t % 40) as i32;
            let offset = pos - x;
            if (-1..=1).contains(&offset) {
                '#'
            } else {
                '.'
            }
        })
        .collect();

    screen
        .as_bytes()
        .chunks(40)
        .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
        .collect()
}

fn print_screen(lines: &[String]) {
    for line in lines {
        println!("{line}");
    }
}

fn main() -> Result<(), String> {
    let cycles = execute_file("input")?;
    let screen = render_cycles(&cycles);
    print_screen(&screen);
    Ok(())
}
