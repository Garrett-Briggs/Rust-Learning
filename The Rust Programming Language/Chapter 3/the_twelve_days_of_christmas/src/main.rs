fn day_1_lyric() { println!("A partridge in a pear tree") }
fn day_2_lyric() { println!("Two turtle doves and"); day_1_lyric(); }
fn day_3_lyric() { println!("Three french hens"); day_2_lyric(); }
fn day_4_lyric() { println!("Four calling birds"); day_3_lyric(); }
fn day_5_lyric() { println!("Five golden rings"); day_4_lyric(); }
fn day_6_lyric() { println!("Six geese a-laying"); day_5_lyric(); }
fn day_7_lyric() { println!("Seves swans a-swimming"); day_6_lyric(); }
fn day_8_lyric() { println!("Eight maids a-milking"); day_7_lyric(); }
fn day_9_lyric() { println!("Nine ladies dancing"); day_8_lyric(); }
fn day_10_lyric() { println!("Ten lords a-leaping"); day_9_lyric(); }
fn day_11_lyric() { println!("Eleven pipers piping"); day_10_lyric(); }
fn day_12_lyric() { println!("Twelve drummers drumming"); day_11_lyric(); }

fn on_the_day_lyric(word: &str) {
    println!("\nOn the {} day of Christmas, my true love sent to me", word);
}

fn day_lyric(day: i32) {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
    "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    if (1..=12).contains(&day) {
        on_the_day_lyric(days[day as usize - 1]);
        match day {
            1 => day_1_lyric(),
            2 => day_2_lyric(),
            3 => day_3_lyric(),
            4 => day_4_lyric(),
            5 => day_5_lyric(),
            6 => day_6_lyric(),
            7 => day_7_lyric(),
            8 => day_8_lyric(),
            9 => day_9_lyric(),
            10 => day_10_lyric(),
            11 => day_11_lyric(),
            12 => day_12_lyric(),
            _ => {}
        }
    }
}

fn main() {
    for i in 1..=12 {
        day_lyric(i);
    }
}
