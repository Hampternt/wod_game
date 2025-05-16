use std::io::stdin;

pub fn dice_roller() {
    let dice: i32 = 3;
    let sides: i32 = 8;
}

fn throw_dice(number_dice: i32, dice_sides: i32) -> Vec<i32> {
    let mut dice_rolled: Vec<i32> = Vec::new();
    let mut input_read: String = stdin().read_line(&mut input_read);

    dice_rolled
}


#[derive(Default)]
struct SelectorFormat {
    amount_of_dice: i32,
    difficulty: i32,
    type_of_dice: i32,
    successes_needed: i32,
}


fn dice_selector_formater() -> SelectorFormat {
    let mut format_value: SelectorFormat = SelectorFormat::default();

    format_value
}

