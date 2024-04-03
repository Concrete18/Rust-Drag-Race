use clearscreen::ClearScreen;
use rand::Rng;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
mod utils;

extern crate termsize;

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
    fn get_position_display(&mut self, total_distance: u16, car_won: bool) -> String {
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
            car_line.push_str("||"); // End the line
        }
        car_line
    }
}

/// Starts the race countdown starting from 3
fn start_countdown(ms_delay: u64) {
    const COUNTDOWN: [&str; 5] = ["\nStarting Race", " 3", " 2", " 1", " Go...\n"];
    for (i, el) in COUNTDOWN.iter().enumerate() {
        print!("{el}");
        io::stdout().flush().unwrap();
        if i != 4 {
            sleep(Duration::from_millis(ms_delay));
        }
    }
}

/// Starts the race with the given cars
fn start_race(total_cars: u16) {
    let start_line = utils::get_current_line_number().unwrap();
    let termsize::Size { rows: _, cols } = termsize::get().unwrap();
    let finish_distance: u16 = cols - 20;
    let mut cars: Vec<Car> = create_cars(total_cars);
    let mut car_won = false;
    println!();
    loop {
        for car in &mut cars {
            car.drive();
            car_won = car.distance >= finish_distance;
            let car_line = car.get_position_display(finish_distance, car_won);
            utils::replace_line(car.num, car_line, start_line);
            if car_won {
                let win_str = format!("\n\nCar {} Won the race!", car.num);
                utils::replace_line(total_cars, win_str, start_line);
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

/// Waits for a number input that wll  returned as a u16
///
/// Defaults to 5 if input errors
fn ask_for_u16() -> u16 {
    let response = utils::input();
    match response.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Using default value 5.");
            5
        }
    }
}

/// Waits for enter to be pressed to continue
fn ask_to_restart_or_exit() {
    println!("\nType y to rerun or just press Enter to exit");
    let response = utils::input();
    if response == "y" {
        ClearScreen::default().clear().expect("Clear Failed");
        start();
    }
}

/// starts everything and can be rerun to race again
fn start() {
    println!("Welcome to The Rust Car Race Simulator");

    println!("\nHow many cars should race?");
    let total_cars = ask_for_u16();

    start_countdown(500);
    start_race(total_cars);
    ask_to_restart_or_exit();
}

fn main() {
    start()
}

#[cfg(test)]
mod hangman_tests {
    use super::*;

    #[test]
    fn car_init() {
        let car = Car::new(1);
        assert!(car.num == 1);
        assert!(car.distance == 0);
    }

    #[test]
    fn drive_success() {
        let mut car = Car::new(1);
        car.drive();
        assert!(car.distance > 0);
        assert!(car.distance < 4);
    }

    #[test]
    fn print_position_success() {
        let mut car = Car::new(1);
        let output = car.get_position_display(15, false);
        assert!(output == "   1 |∙،°.˘Ô≈ôﺣ               ||");
        car.distance = 16;
        let output = car.get_position_display(15, true);
        println!("{output}");
        assert!(output == "   1 |                ∙،°.˘Ô≈ôﺣ");
    }

    #[test]
    fn load_words_from_file() {
        let cars: Vec<Car> = create_cars(5);
        assert!(cars.len() == 5);
    }
}
