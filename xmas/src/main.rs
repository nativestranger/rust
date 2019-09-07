fn main() {
    let day_lines = vec!["A partridge in a pear tree", "Two turtle doves, and", "Three french hens", "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "10 lords a-leaping", "11 pipers piping", "12 drummers drumming"];
    let ordinals  = vec!["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    for day_number in 1..13 {
        println!("On the {} day of Christmas my true love gave to me", ordinals[day_number - 1]);

        for countdown_day_number in (0..day_number).rev() {
            println!("{}", day_lines[countdown_day_number]);
        }

        println!("");
        println!(" 🎄 🎄 🎄");
        println!("");
    }
}
