# Tic-Tac-Toe Game with Slint and Rust

A This project implements a simple Tic-Tac-Toe game using Slint for the UI and Rust for the game logic.

## Features

- Two-player Tic-Tac-Toe game
- Reset the game when a player wins or when it's a draw
- Simple and clean UI

## Prerequisites

- Rust programming language: Install Rust
- Slint library: Install Slint

## Getting Started

Clone the repository
   
    ```
    git clone https://github.com/BrunoCoelho98/tic-tac-toe-Slint.git
      cd tic-tac-toe-Slint
    ```
## Build and Run

     ```
     cargo run
     ```
Or you can just acess the WebAssembly version: https://brunocoelho98.github.io/tic-tac-toe-Slint/


## Game Logic

The game logic is written in Rust and is responsible for initializing the UI, setting up the game board, and handling the game logic.

- 'reset_game' function: This function resets the game state by reinitializing the squares on the board. It collects the current state of squares, creates a new model, and sets this model to the UI.

- 'main' function: This is the entry point of the application. It initializes the UI, sets up the game board matrix, and manages the game state. It also includes a callback to check if the game is over, which resets the game if a win or draw condition is met.

## UI Design

The UI is designed using Slint and consists of a grid of squares representing the Tic-Tac-Toe board.

- 'Square' Component: Represents an individual cell in the Tic-Tac-Toe grid. It has properties for checking if the cell has been clicked, displaying the appropriate icon (X or O), and determining if the game is over.

- 'App Window' Component: Represents the main window of the application, containing the Tic-Tac-Toe board. It manages the overall game state, including the player's turn and the game over condition. It also initializes the squares and handles user interactions.


