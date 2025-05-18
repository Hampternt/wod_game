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
fn roll_the_dice(amount_of_dice: Option<i32>, dice_type: Option<i32>) -> Vec<DiceTemplate> {
    let amount = amount_of_dice.unwrap_or(1);
    let dice_t = dice_type.unwrap_or(10);  

    dice_roller_function(amount,dice_t)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
