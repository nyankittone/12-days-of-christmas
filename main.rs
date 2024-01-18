// Behold, a primarily C programmer trying to write Rust code! (this shit is imperative asf)
// As I get better at this language, I hope my code will become less noob-y. :(

// this was all written from my poor memory of writing a smidge of Rust in the past, plus 20
// minutes or so of me scanning through chapter 3 of the Rust Book. this is absolutely noobcore
// Rust code.

// The " 'static " in the return type is needed, because otherwise, the borrow checker will bully
// me and beat me to death with a wooden bat.
fn yeild_number_suffix(n: isize) -> &'static str {
    if (n / 10) % 10 == 1 {
        return "th";
    }

    match n % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}

// This thingie prints out a single "day" of the Christmas carol. If a vaild number for the day is
// passed, the corrosponding day is printed out, and the function returns true. Otherwise, it
// returns false.
fn print_one_iteration(day: isize) -> bool {
    let items = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "F I V E  G O L D E N  R I N G S",
        "six geese of laying",
        "seven swans of swimming",
        "eight maids of milking",
        "nine ladies dancing",
        "ten lords of leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    if day <= 0 || day as usize > items.len() {
        return false;
    }

    println!("On the {day}{} day of Christmas, my true love gave to me:",yeild_number_suffix(day));

    if day > 1 {
        for i in (0..day).rev() {
            println!("{},",items[i as usize]);
        }

        print!("and ");
    }

    println!("{}!",items[0]);
    true
}

// pretend this comment has any meaning please
fn main() {
    for i in 1.. {
        if !print_one_iteration(i) {
            break;
        }

        print!("\n");
    }
}

