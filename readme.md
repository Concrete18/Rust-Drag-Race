# Rust Drag Race

Rust Drag Race is a drag race simulator implemented entirely within the terminal using Rust programming language. This project is designed to showcase Rust's capabilities in handling simulations and terminal-based applications.

![Console Preview](https://raw.githubusercontent.com/Concrete18/Rust-Race/main/images/example.png)

## Introduction

Rust Drag Race is a project developed to improve understanding and usage of Rust structs, ownership, and terminal-based applications. It provides a drag race simulation experience right in your terminal, allowing users to configure race parameters and observe the race progress in real-time.

## Features

- **Customizable Car Count**: Choose the number of cars participating in the race.
- **Tie Detection**: Detect and handle tie scenarios gracefully during the race.
- **Dynamic Race Length**: Automatically sets the race length based on the terminal's width for an immersive experience.
- **Optimized for Terminal Height**: Limits the maximum number of cars based on the terminal's height to ensure smooth rendering.
- **User Interaction**: Provides options to restart the race or exit the application once the race finishes.

## Getting Started

To get started with Rust Drag Race, follow these steps:

1. Clone the repository:

If this does not work, copy the HTTPS clone string at the top of the Github repo.

```bash
git clone https://github.com/Concrete18/Rust-Drag-Race.git
```

2. Navigate to the project directory:

```bash
cd Rust-Race
```

3. Build and run the project:

```bash
cargo run --release
```

## Usage

Once the project is running, you can interact with the drag race simulation as follows:

- Choose the number of cars to participate in the race.
- Observe the race progress in real-time as cars move towards the finish line.
- Upon race completion, you will be prompted with options to restart the race or exit the application.
