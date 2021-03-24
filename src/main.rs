use std::io::{stdin, stdout, Write};

fn main() {
    println!("Convert your temperature!\n");

    let temp = get_temp();
    let scale = get_scale();

    println!("Converting {} to {}", temp, scale);
    println!("- - - - - - - - - - - - -");

    if scale == "C".to_string() {
        println!("{} Fahrenheit is {} degrees Celsius.", temp, fc(&temp));
    } else if scale == "F".to_string() {
        println!("{} Celsius is {} degrees Fahrenheit.", temp, cf(&temp));
    }
}

// Converts fahrenheit to celsius
fn fc(&input: &f32) -> f32 {
    (input - 32.0) * 5.0 / 9.0
}

// Converts celsius to fahrenheit
fn cf(&input: &f32) -> f32 {
    input * 9.0 / 5.0 + 32.0
}

// Reads temp from stdin -> returns as f32
fn get_temp() -> f32 {
    let mut uinput = String::new();
    
    println!("- - - - - - - - - - - - -");
    print!("Input a temperature: ");

    flush_out();
    read_in(&mut uinput);

    let uinput: f32 = match uinput.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("\nNumbers only... lol");
            get_temp()
        }
    };

    uinput

}

// Reads scale from stdin and checks for validity
fn get_scale() -> String {
    let mut uinput = String::new();
    let c = "C".to_string();
    let f = "F".to_string();

    println!("- - - - - - - - - - - - -");
    print!("Converting FROM C or F: ");

    flush_out();
    read_in(&mut uinput);

    println!("- - - - - - - - - - - - -");

    uinput = uinput[..1].to_uppercase();

    if uinput == c {
        uinput = f;
    } else if uinput == f {
        uinput = c;
    } else {
        println!("\nLol, what?");
        uinput = get_scale();
    }

    uinput

}

// Flushes stdout
fn flush_out() {
    stdout().flush()
        .expect("Failed to flush stdout.");
}

// Reads text from stdin
fn read_in(input: &mut String) {
    stdin().read_line(input)
        .expect("Failed to read stdin.");
}