use std::io;

fn main() {
    let f = String::from("F");
    let c = String::from("C");

    loop {
        println!("Choose your temperature scale (F or C)");

        let mut temperature_scale = String::new();

        io::stdin().read_line(&mut temperature_scale)
            .expect("Failed to read line"); // uses Result enum to crash & print string if Err variant is returned (vs Ok, where expect returns the ok value)

        if temperature_scale.trim() == f {
            let temperature = get_temperature(&temperature_scale);
            let result = (temperature - 32) * 5/9;
            println!("{}°C", result);
            break;
        } else if temperature_scale.trim() == c {
            let temperature = get_temperature(&temperature_scale);
            let result = (temperature * 9/5) + 32;
            println!("{}°F", result);
            break;
        } else {
          println!("You must enter exactly F or C");
        }
    }
}


fn get_temperature(temperature_scale: &String) -> i32 {
    let mut temperature = String::new();

    loop {
        println!("Enter the temperature in {}", temperature_scale);

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => {
                return num;
            },
            Err(_) => continue,
        };
    }
}
