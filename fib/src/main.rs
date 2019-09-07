use std::io;

fn main() {
    let mut seq = vec![0,1];

    let int = get_usize();

    let mut next_to_last_i = 0;
    let mut last_i = 1;

    while seq.len() < int {
        let next = seq[next_to_last_i] + seq[last_i];
        seq.push(next);
        next_to_last_i += 1;
        last_i += 1;
    }

    println!("result: {}", seq[int - 1]);
}


fn get_usize() -> usize {
    let mut int = String::new();

    loop {
        println!("Enter an integer");

        io::stdin().read_line(&mut int)
            .expect("Failed to read line");

        let _int: u32 = match int.trim().parse() {
            Ok(num) => {
                return num;
            },
            Err(_) => continue,
        };
    }
}
