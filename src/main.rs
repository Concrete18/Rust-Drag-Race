use crossterm::{
    cursor::{position, MoveTo},
    execute,
};

use rand::Rng;
use std::io::{self, Write};

use std::thread::sleep;
use std::time::Duration;

struct Car {
    num: u8,
    distance: u16,
}

impl Car {
    fn drive(&mut self) {
        let mut rng = rand::thread_rng();
        let drive_distance: u16 = rng.gen_range(1..=3);
        self.distance += drive_distance;
    }
}

fn start_race() {
    print!("Starting Race");
    io::stdout().flush().unwrap();
    sleep(Duration::from_millis(500));
    print!(" 3");
    io::stdout().flush().unwrap();
    sleep(Duration::from_millis(500));
    print!(" 2");
    io::stdout().flush().unwrap();
    sleep(Duration::from_millis(500));
    print!(" 1");
    io::stdout().flush().unwrap();
    sleep(Duration::from_millis(500));
    print!(" Go...");
    io::stdout().flush().unwrap();
}

fn create_cars(total_cars: i32) -> Vec<Car> {
    let mut cars: Vec<Car> = vec![];
    for n in 1..=total_cars {
        let new_car = Car {
            num: n as u8,
            distance: 0,
        };
        cars.push(new_car);
    }
    cars
}

fn replace_line(line_num: u16, new_contents: String, start_line: u16) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    // Move cursor to line 5 (0-based index)
    execute!(handle, MoveTo(0, start_line + line_num)).unwrap();
    // Clear line and print new content
    write!(handle, "{}", new_contents).unwrap();
    handle.flush().unwrap();
}

fn print_car_position(car: &Car, total_distance: u16, car_won: bool, start_line: u16) {
    let ascii_car = "∙،°.˘Ô≈ôﺣ";
    let car_distance = car.distance as usize;
    let spaces_before = car_distance;
    let spaces_after = total_distance.saturating_sub(car_distance as u16);

    let mut car_line = String::new();

    // Adjust spacing for car number
    car_line.push_str(&format!("{: >4} |", car.num));

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
    replace_line(car.num as u16, car_line, start_line)
}

fn get_current_line_number() -> Result<u16, crossterm::ErrorKind> {
    let (_, line) = position()?;
    Ok(line)
}

fn main() {
    let start_line = get_current_line_number().unwrap();
    start_race();
    let finish_distance: u16 = 120;
    let total_cars = 15;
    let mut cars: Vec<Car> = create_cars(total_cars);
    let mut car_won = false;
    loop {
        for car in cars.iter_mut() {
            car.drive();
            // println!("Car {} is at {}", car.num, car.distance);
            car_won = car.distance >= finish_distance;
            print_car_position(car, finish_distance, car_won, start_line);
            if car_won {
                let win_str = format!("\nCar {} Won!", car.num);
                replace_line(total_cars as u16, win_str, start_line);
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
