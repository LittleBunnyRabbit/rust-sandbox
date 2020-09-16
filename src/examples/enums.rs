enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Move up"),
        Movement::Down => println!("Move down"),
        Movement::Left => println!("Move left"),
        Movement::Right => println!("Move right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}