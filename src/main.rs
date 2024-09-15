const GIFTS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three French hens",
    "four calling birds",
    "five gold rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

fn gifts_for_day(day: &usize) -> impl Iterator<Item = &&'static str> {
    GIFTS.iter().take(day + 1).rev()
}

fn main() {
    for (i, day) in DAYS.iter().enumerate() {
        println!("On the {} day of Christmas, my true love gave to me:", day);
        let mut gifts = gifts_for_day(&i).peekable();
        while let Some(gift) = gifts.next() {
            if gifts.peek().is_none() && i > 0 {
                println!("and {}", gift);
            } else {
                println!("{}", gift);
            }
        }
        println!();
    }
}
