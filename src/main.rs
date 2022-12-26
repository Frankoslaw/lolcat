use std::{f32::consts::PI, io};

fn rgb(i: usize) -> (i32, i32, i32){
    let f: f32 = 0.1;
    return (
        ((f*i as f32 + 0.).sin() * 127. + 128.) as i32, 
        ((f*i as f32 + 2.*PI/3.).sin() * 127. + 128.) as i32, 
        ((f*i as f32 + 4.*PI/3.).sin() * 127. + 128.) as i32
    )
}

fn print(output: String) {
    for n in 0..output.len(){
        let (r, g, b) = rgb(n);
        print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, output.chars().nth(n).unwrap());
    }
    println!("");
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from pipe");
        input = input.trim().to_string();
        if input == "" {
            break;
        }

        print(input);
    }
}
