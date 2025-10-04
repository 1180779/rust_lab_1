use rand::Rng;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    let mut input: String;
    let mut file = File::create("foo.txt").expect("File not created!");

    let exited_on_error = loop {
        println!("Podaj liczbę: ");

        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let number: u64 = match input.trim().parse::<u64>() {
            Ok(num) => num,
            Err(_) => break true,
        };
        if number == 0 {
            break false;
        }

        let x = number + rand::rng().random_range(0..=5);
        println!("x = {}", x);

        let arr = exponents(x);
        println!("arr = {:?}", arr);

        let mut colatz_arr = [false; 10];
        for i in 0..arr.len() {
            colatz_arr[i] = collatz_hypothesis(arr[i], 100);
        }
        println!("colatz_arr = {:?}", colatz_arr);

        // write all to file
        file.write_all(
            format!(
                "number = {}\nx = {}\narr = {:?}\ncoltz_arr = {:?}\n\n",
                number, x, arr, colatz_arr
            )
            .as_bytes(),
        ).expect("Failed to write to file!");
    };

    if exited_on_error {
        println!("Exited loop on error");
    } else {
        println!("Exited loop on user request");
    }

    // my own functionality that returns a tuple with different types directly from the inner loop
    // let tuple: (String, i32) = 'outer: loop {
    //     loop {
    //         break 'outer (String::from("123123"), 12);
    //     }
    // };
}

fn exponents(x: u64) -> [u64; 10] {
    let mut arr = [0; 10];
    let mut c = x;
    for item in arr.iter_mut() {
        *item = c;
        c *= x;
    }
    arr
}

fn collatz_hypothesis(mut number: u64, iter_limit: u64) -> bool {
    for _ in 0..iter_limit {
        if number == 1 {
            return true;
        }
        number = match number % 2 {
            1 => 3 * number + 1,
            0 => number / 2,
            _ => panic!(),
        }
    }
    false
}
