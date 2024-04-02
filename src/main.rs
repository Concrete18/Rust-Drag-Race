use crossterm::{
    cursor::{position, MoveTo},
    execute,
};
use rand::Rng;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

struct Car {
    num: u16,
    distance: u16,
}

impl Car {
    /// Creates a new car with the given number and defaults to 0 distance traveled
    fn new(num: u16) -> Self {
        Self { num, distance: 0 }
    }

    /// Makes the car drive towards the finish line by 1-3 spaces
    fn drive(&mut self) {
        let mut rng = rand::thread_rng();
        let drive_distance: u16 = rng.gen_range(1..=3);
        self.distance += drive_distance;
    }

    /// Prints its current position in the race
    fn print_position(&mut self, total_distance: u16, car_won: bool, start_line: u16) {
        let ascii_car = "∙،°.˘Ô≈ôﺣ";
        let car_distance = self.distance;
        let spaces_before = car_distance;
        let spaces_after = total_distance.saturating_sub(car_distance);
        let mut car_line = String::new();
        // Adjust spacing for car number
        car_line.push_str(&format!("{: >4} |", self.num));
        // Add empty spaces before the car
        for _ in 0..spaces_before {
            car_line.push(' ');
        }
        // Add the car representation
        car_line.push_str(ascii_car);
        // Add empty spaces after the car
        for _ in 0..spaces_after {
            car_line.push(' ');
        }
        if !car_won {
            car_line.push('|'); // End the line
        }
        replace_line(self.num, car_line, start_line);
    }
}

/// Starts the race countdown starting from 3
fn start_countdown(ms_delay: u64) {
    println!();
    for str in ["Starting Race", " 3", " 2", " 1", " Go..."] {
        print!("{str}");
        io::stdout().flush().unwrap();
        if str != " Go..." {
            sleep(Duration::from_millis(ms_delay));
        }
    }
}

/// Starts the race with the given cars
fn start_race(total_cars: u16) {
    let start_line = get_current_line_number().unwrap();
    let finish_distance: u16 = 120;
    let mut cars: Vec<Car> = create_cars(total_cars);
    let mut car_won = false;
    println!();
    loop {
        for car in &mut cars {
            car.drive();
            car_won = car.distance >= finish_distance;
            car.print_position(finish_distance, car_won, start_line);
            if car_won {
                let win_str = format!("\nCar {} Won the race!", car.num);
                replace_line(total_cars, win_str, start_line);
                println!();
                break;
            }
            // TODO allow ties
        }
        if car_won {
            break;
        }
        sleep(Duration::from_millis(50));
    }
}

/// Creates a vector of Cars
fn create_cars(total_cars: u16) -> Vec<Car> {
    let mut cars: Vec<Car> = vec![];
    for n in 1..=total_cars {
        let new_car = Car::new(n);
        cars.push(new_car);
    }
    cars
}

/// Replaces the line at `line_num` with `new_contents` relative to `start_line`
fn replace_line(line_num: u16, new_contents: String, start_line: u16) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    // Move cursor to line 5 (0-based index)
    execute!(handle, MoveTo(0, start_line + line_num)).unwrap();
    // Clear line and print new content
    write!(handle, "{new_contents}").unwrap();
    handle.flush().unwrap();
}

/// Gets the current line number in the terminal
fn get_current_line_number() -> Result<u16, crossterm::ErrorKind> {
    let (_, line) = position()?;
    Ok(line)
}

/// Asks for input after printing a msg
fn input() -> String {
    let mut response: String = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    return response.trim().to_string();
}

/// Waits for a number input that wll  returned as a u16
///
/// Defaults to 5 if input errors
fn ask_for_u16() -> u16 {
    let response = input();
    match response.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Using default value 5.");
            5
        }
    }
}

/// Waits for enter to be pressed to continue
fn enter_to_continue() {
    println!("\nPress Enter to exit");
    input();
}

fn main() {
    println!("Welcome to The Rust Car Race Simulator");

    println!("\nHow many cars should race?");
    let total_cars = ask_for_u16();

    start_countdown(500);
    start_race(total_cars);
    enter_to_continue();
}
