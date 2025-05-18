pub fn dice_roller_function(amount_of_dice: Option<i32>, dice_type: Option<i32>) -> Vec<DiceTemplate> {
    let amount = amount_of_dice.unwrap_or(1);
    let dice_t = dice_type.unwrap_or(10);

    let mut counter: i32 = 0;
    let mut dice_rolled: Vec<DiceTemplate> = Vec::new();
    loop {
        dice_rolled.push(roll_dice(dice_t));
        counter += 1;
        if counter >= amount {
            break;
        }
    }

    dice_rolled
}

pub fn roll_dice(_dice_sides: i32) -> DiceTemplate {

    DiceTemplate {
        dice_type: 3,
        roll_number: 1,
        successes: 1,
    }
}

pub struct DiceTemplate {
    dice_type: i32,
    roll_number: i32,
    successes: i32,
}
