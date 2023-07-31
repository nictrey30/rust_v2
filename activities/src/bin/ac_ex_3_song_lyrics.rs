// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
// taking advantage of the repetition in the song.

fn main() {
    println!("TWELVE DAYS OF CHRISTMAS!");
    for day in 1..13 {
        day_intro(day);
        for gift_day in (1..(day + 1)).rev() {
            gift(
                gift_day,
                if gift_day == 1 && day != 1 {
                    "and "
                } else {
                    ""
                },
            )
        }
    }
}
fn day_intro(verse_num: u32) {
    let mut day = match verse_num {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eigth",
        9 => "nineth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelveth",
        _ => "",
    };
    println!(
        "\nOn the {} day of Christmas\nMy true love sent to me:",
        day
    )
}
fn gift(n: u32, prefix: &str) {
    let gift_text = match n {
        1 => "a Partridge in a Pear Tree",
        2 => "Two Turtle Doves",
        3 => "Three French Hens",
        4 => "Four Calling Birds",
        5 => "Five Golden Rings",
        6 => "Six Geese a Laying",
        7 => "Seven Swans a Swimming",
        8 => "Eight Maids a Milking",
        9 => "Nine Ladies Dancing",
        10 => "Ten Lords a Leaping",
        11 => "Eleven Pipers Piping",
        12 => "Twelve Drummers Drumming",
        _ => "",
    };

    println!("{}{}", prefix, gift_text);
}
