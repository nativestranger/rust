// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and
// “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to
// the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

// TODO: add specs & split on whitespace to translate longer strings

fn main() {
    fn is_vowel(s: &str) -> bool {
        s == "a" || s == "e" || s == "i" || s == "o" || s == "u"
    }

    fn translate_word(s: String) -> String {
        if is_vowel(&s[0..1]) {
            format!("{}-hay", &s)
        } else {
            format!("{}-fay", &s[1..])
        }
    }

    println!("{}", translate_word(String::from("first")));
    println!("{}", translate_word(String::from("apple")));
}
