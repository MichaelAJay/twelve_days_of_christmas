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

fn iterative_twelve_days() {
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

fn recursive_twelve_days(day: usize) {
    if day >= DAYS.len() {
        return;
    }

    println!(
        "On the {} day of Christmas, my true love gave to me:",
        DAYS[day]
    );
    let mut gifts = gifts_for_day(&day).peekable();
    while let Some(gift) = gifts.next() {
        let is_first_gift = gifts.peek().is_none();
        if is_first_gift && day > 0 {
            println!("and {}", gift);
        } else {
            println!("{}", gift);
        }
    }
    println!();

    recursive_twelve_days(day + 1)
}

fn main() {
    // iterative_twelve_days();
    recursive_twelve_days(0);
}
