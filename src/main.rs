use std::io;

fn main() {
    println!("Tic-Tac-Toe");

    let mut board = [['.'; 3]; 3];
    let mut round_count = 1;
    let mut game_over = false;

    let mut tile_location = String::new();

    while !game_over {
        print_board(&board);

        loop {
            tile_location.clear();

            println!("Player {}: Enter the tile you want to place your symbol (a1 would be top left):", (round_count - 1) % 2 + 1);

            io::stdin()
                .read_line(&mut tile_location)
                .expect("Failed to read line!");

            match validate_input(&tile_location.trim()) {
                Ok(()) => break,
                Err(err) => println!("{}", err)
            }
        }

        place_symbol(&tile_location, &mut board, round_count);

        game_over = check_for_win(&board, round_count);

        round_count += 1;
    }

    if round_count == 10 {
        println!("Draw! Final board:");
        print_board(&board);
    } else {
        println!("Player {} wins! Final board:", (round_count - 1) % 2 + 1);
        print_board(&board);
    }
}

fn validate_input(input: &str) -> Result<(), &str>{
    if input.len() != 2 {
        return Err("Please enter no more than 2 characters!")
    }

    if input.to_lowercase().chars().next().unwrap() != 'a' &&
        input.to_lowercase().chars().next().unwrap() != 'b' &&
        input.to_lowercase().chars().next().unwrap() != 'c' {
        return Err("Make sure your first character (the column) is either 'a', 'b', or 'c'!")
    }

    if input.to_lowercase().chars().nth(1).unwrap() != '1' &&
        input.to_lowercase().chars().nth(1).unwrap() != '2' &&
        input.to_lowercase().chars().nth(1).unwrap() != '3' {
        return Err("Make sure your second character (the row) is either '1', '2', or '3'!")
    }

    Ok(())
}

fn place_symbol(tile_location: &str, board: &mut [[char; 3]; 3], round_count: u32) {
    let mut tile_y = 0;

    match tile_location
        .to_lowercase()
        .chars()
        .next()
        .expect("Index out of bounds!") {
        'a' => tile_y = 0,
        'b' => tile_y = 1,
        'c' => tile_y = 2,
        _ => {} // what should I do here?
    }

    let tile_x = tile_location
        .chars()
        .nth(1)
        .expect("Index out of bounds!");
    let tile_x = tile_x
        .to_digit(10)
        .expect("Not a digit!") - 1;

    if round_count % 2 == 0 {
        board[tile_x as usize][tile_y] = 'O';
    } else {
        board[tile_x as usize][tile_y] = 'X';
    }

}

fn print_board(board: &[[char; 3]; 3]) {
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
fn check_for_win(board: &[[char; 3]; 3], round_count: u32) -> bool {
    if round_count == 9 {
        return true;
    }

    // horizontal wins
    for y in board {
        if y[0] == y[1] && y[0] == y[2] && y[0] != '.' {
            return true;
        }
    }

    // vertical wins
    for i in 0..=2 {
        if board[0][i] == board[1][i] && board[0][i] == board[2][i] && board[0][i] != '.' {
            return true;
        }
    }

    // diagonal wins
    if board[0][0] == board[1][1] && board[0][0] == board[2][2] && board[0][0] != '.' {
        return true;
    }
    if board[0][2] == board[1][1] && board[0][2] == board[2][0] && board[0][2] != '.' {
        return true;
    }

    return false;
}