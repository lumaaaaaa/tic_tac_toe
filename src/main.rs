use std::io;

fn main() {
    println!("TicTacToe");

    let mut board = vec![vec!['.'; 3]; 3];
    let mut round_count = 1;
    let mut game_over = false;

    let mut tile_location = String::new();

    while !game_over {
        print_board(&board);

        println!("Player {}: Enter the tile you want to place your symbol (A1 would be top left):", (round_count - 1) % 2 + 1);

        io::stdin()
            .read_line(&mut tile_location)
            .expect("Failed to read line!");

        place_symbol(&tile_location, &mut board, round_count);

        tile_location.clear();

        game_over = check_for_win(&board, round_count);

        round_count += 1;
    }
}

fn place_symbol(tile_location: &String, board: &mut Vec<Vec<char>>, round_count: u32) {
    let mut tile_y = 0;

    match tile_location.to_uppercase().chars().nth(0).expect("Index out of bounds!") {
        'A' => tile_y = 0,
        'B' => tile_y = 1,
        'C' => tile_y = 2,
        _ => {} // what should I do here?
    }

    let tile_x = tile_location.chars().nth(1).expect("Index out of bounds!");
    let tile_x = tile_x.to_digit(10).expect("Not a digit!") - 1;

    if round_count % 2 == 0 {
        board[tile_x as usize][tile_y] = 'O';
    } else {
        board[tile_x as usize][tile_y] = 'X';
    }

}

fn print_board(board: &Vec<Vec<char>>) {
    for y in board {
        for x in y {
            print!("{}", x);
            print!(" ");
        }
        println!()
    }
}

// returns true if a player has won, returns false otherwise
// this function is only designed to be used after a player's turn, called at the end of every round
fn check_for_win(board: &Vec<Vec<char>>, round_count: u32) -> bool {
    if round_count == 9 {
        println!("Draw! Final board:");
        print_board(&board);
        return true;
    }

    let player_num = (round_count - 1) % 2 + 1;

    // horizontal wins
    for y in board {
        if y[0] == y[1] && y[0] == y[2] && y[0] != '.' {
            println!("Player {} wins! Final board:", player_num);
            print_board(&board);
            return true;
        }
    }

    // vertical wins
    for i in 0..=2 {
        if board[0][i] == board[1][i] && board[0][i] == board[2][i] && board[0][i] != '.' {
            println!("Player {} wins! Final board:", player_num);
            print_board(&board);
            return true;
        }
    }

    // diagonal wins
    if board[0][0] == board[1][1] && board[0][0] == board[2][2] && board[0][0] != '.' {
        println!("Player {} wins! Final board:", player_num);
        print_board(&board);
        return true;
    }
    if board[0][2] == board[1][1] && board[0][2] == board[2][0] && board[0][2] != '.' {
        println!("Player {} wins! Final board:", player_num);
        print_board(&board);
        return true;
    }

    return false;
}