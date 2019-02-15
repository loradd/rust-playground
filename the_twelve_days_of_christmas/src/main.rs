const DAYS: [&str; 12] = [
    "twelfth",
    "eleventh",
    "tenth",
    "ninth",
    "eighth",
    "seventh",
    "sixth",
    "fifth",
    "fourth",
    "third",
    "second",
    "first"
];

const LINES: [&str; 12] = [
    "Twelve drummers drumming",
    "Eleven pipers piping",
    "Ten lords a-leaping",
    "Nine ladies dancing",
    "Eight maids a-milking",
    "Seven swans a-swimming",
    "Six geese a-laying",
    "Five golden rings",
    "Four calling birds",
    "Three french hens",
    "Two turtle doves, and",
    "A partridge in a pear tree"
];

fn main() {
    for day_index in (0..12).rev() {
        println!("On the {} day of Christmas my true love sent to me", DAYS[day_index]);
        for line_index in day_index..12 {
            println!("{}", LINES[line_index])
        }
        println!();
    }
}