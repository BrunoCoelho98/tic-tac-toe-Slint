#![windows_subsystem = "windows"] // Comment this line out if you are not using windows


use slint::Model;

slint::include_modules!();

pub fn reset_game(ui: &AppWindow) {
    let squares: Vec<squareData> = ui.get_squares().iter().collect();
    let square_model = std::rc::Rc::new(slint::VecModel::from(squares));
    ui.set_squares(square_model.clone().into());
}


//Comment this line out if you are not using wasm
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let ui = AppWindow::new().unwrap();
    let ui_handle: slint::Weak<AppWindow> = ui.as_weak();

    // use the square type that i made inside of the appwindow.slint file
    let squares: Vec<squareData> = ui.get_squares().iter().collect();
    

    // create a new model
    let square_model = std::rc::Rc::new(slint::VecModel::from(squares));

    ui.set_squares(square_model.clone().into());

    //make a matrix representing the tic tac toe board
    let mut board = [[0; 3]; 3];

    // check if game is over
    ui.on_check_if_game_over(move |position: i32, player: bool| {
        let mut finished = vec![];
        //add the current the position and player to the board
        board[position as usize / 3][position as usize % 3] = if player { 1 } else { 2 };
        //check if the game is over
        finished.push(board[0][0]*board[1][1]*board[2][2]);
        finished.push(board[0][2]*board[1][1]*board[2][0]);
        finished.push(board[0][0]*board[0][1]*board[0][2]);
        finished.push(board[1][0]*board[1][1]*board[1][2]);
        finished.push(board[2][0]*board[2][1]*board[2][2]);
        finished.push(board[0][0]*board[1][0]*board[2][0]);
        finished.push(board[0][1]*board[1][1]*board[2][1]);
        finished.push(board[0][2]*board[1][2]*board[2][2]);
        if finished.contains(&1){
            // if X win reset the game
            let ui = ui_handle.upgrade().unwrap();
            reset_game(&ui);
            board = [[0; 3]; 3];
            return true;
        } else if finished.contains(&8){
            let ui = ui_handle.upgrade().unwrap();
            // if O win reset the game
            reset_game(&ui);
            board = [[0; 3]; 3];
            return true;
        } else if finished.contains(&0){
            // if not yet finished do nothing
            return false;
        } else {
            // if it is a draw reset the game
            let ui = ui_handle.upgrade().unwrap();
            reset_game(&ui);
            board = [[0; 3]; 3];
            return true;
        }
    });

    ui.run().unwrap();
}
