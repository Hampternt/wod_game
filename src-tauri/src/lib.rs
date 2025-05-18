pub mod data_types {
    pub mod human;
    pub mod basic_stats; 
}

pub mod helper_functions {
    pub mod dice_mechanics;
}


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn dice_roller(amount_of_dice: &i32) -> Vec<DiceTemplate> {
    let mut counter: i32 = 0;
    let mut dice_rolled: Vec<DiceTemplate> = Vec::new();
    loop {
        dice_rolled.push(roll_dice());
        counter += 1;
        if counter >= *amount_of_dice {
            break;
        }
    }

    dice_rolled
}

fn roll_dice() -> DiceTemplate {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
