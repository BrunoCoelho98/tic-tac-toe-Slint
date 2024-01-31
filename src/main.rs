use std::rc;
use std::process::exit;

use slint::Model;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
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
            ui.set_squares(square_model.clone().into());
            board = [[0; 3]; 3];
            ui.set_playerTurn(true);
            //set all images to the original state
            exit(1);
            return true;
        } else if finished.contains(&8){
            let ui = ui_handle.upgrade().unwrap();
            // if O win reset the game
            ui.set_squares(square_model.clone().into());
            board = [[0; 3]; 3];
            ui.set_playerTurn(true);
            exit(1);
            return true;
        } else if finished.contains(&0){
            // if not yet finished do nothing
            let ui = ui_handle.upgrade().unwrap();
            return false;
        } else {
            // if it is a draw reset the game
            let ui = ui_handle.upgrade().unwrap();
            ui.set_squares(square_model.clone().into());
            board = [[0; 3]; 3];
            ui.set_playerTurn(true);
            exit(1);
            return true;
        }
    });

    ui.run()
}
