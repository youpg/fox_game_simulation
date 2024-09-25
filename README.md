# fox_game_simulation

## Overview

This rust program simulates a board game on finding the word "FOX" in a 4x4 grid. It perform hunderds of million simulations to calculate the probability of the word "FOX" appearing on a randomly filled 4x4 board.

## Game Rules

1. The game is played on a 4x4 grid (16 cells) aka the board.
2. The grid is filled with the following tiles (in code the bag variable):
   - 5 tiles with the letter 'F' (in code represented by 1)
   - 6 tiles with the letter 'O' (in code represented by 2)
   - 5 tiles with the letter 'X' (in code represented by 3)
3. The tiles are chosen randomly on the grid.
4. The word FOX is found when it appears:
   - horizontal (from left to right or right to left)
   - vertical (from top to bottom or bottom to top)
   - diagonal ( - from top left to bottom right -
                - from top right to bottom left -
                - from bottom left to top right -
                - from bottom right to top left - )

     # Example Directions
     # Horizontal from left to right:
     ![image](https://github.com/user-attachments/assets/93f24cf4-46de-4804-8387-533494b44723)
     # Horizontal from right to left:
     ![image](https://github.com/user-attachments/assets/77b0d323-2730-4a0c-b0ce-7d1181709d17)
     # Vertical from top to bottom:
     ![image](https://github.com/user-attachments/assets/5eccc8a7-2de5-44a2-95ee-990a090f2b7e)
     # Vertical from bottom to top:
     ![image](https://github.com/user-attachments/assets/0d4da0cd-4836-4c17-adeb-5149e433fda5)
     # Diagonal from top left to bottom right:
     ![image](https://github.com/user-attachments/assets/e33f11d9-6451-478f-b3e9-4f92fb5107cd)
     # Diagonal from top right to bottom left:
     ![image](https://github.com/user-attachments/assets/f6b1ac5d-2333-4b9b-a7b8-3901a6f03ce2)
     # Diagonal from bottom left to top right:
     ![image](https://github.com/user-attachments/assets/b049b6c2-7172-4382-a9d4-7dc0d2513729)
     # Diagonal from bottom right to top left:
     ![image](https://github.com/user-attachments/assets/4f0ca867-f312-4abd-bc8a-2124835c6a38)

## Features

- High-perfromance simulation using Rust-lang
- Parallel processing with Rayon for faster simulation
- User input for the number of games to simulate
- Accurate probability calculation
- Accurate calculation timing

## Dependencies

- `rand`: For random number generation to choose the letters
- `rayon`: For parallel iteration
- `std::sync::atomic`: For thread safety
- `std::io`: For user input
- `std::time`: For timing the calculations

## How It Works

1. The user inputs how many games they want to simulate.
2. The program devides the total number of games in different batches for parallel processing.
3. For each game:
   - A board is created and filled with randomly chosen tiles in the bag.
   - The board is checked for the presence of "FOX" in the specified directions.
   - The result is recorded and stored (fox_found or fox_not_found)
4. After all simulations, the program calculates and displays the percentages and the time it took for the calculations


## Code Structure

- `Board`: A struct representing the 4x4 game board
  - `new()`: Creates a empty board
  - `fill_and_shuffle()`: Fills the board with tiles and shuffles them
  - `check_for_word_fox()`: Checks if "FOX" is present in the board in any of the specified directions
- `main()`: The entry point of the program, handling user input, simulation, and result display

## Performance Considerations

- The code uses `inline(always)` for critical functions to improve performance.
- Rayon is used for parallel processing of game batches.
- Atomic operations ensure thread-safe counting of results.

## How to Run

1. Install Rust and Cargo on your system: https://www.rust-lang.org/tools/install
2. Clone this repository: ```git clone https://github.com/youpg/fox_game_simulation.git```
3. Navigate to the project directory: ```cd fox_game_simulation```
5. Run the program: ```cargo run --release```
6. Enter the number of games to simulate when the program asks and press enter.

## Results

The program will output the following statistics:
- Number of games where "FOX" was found
- Percentage of games where "FOX" was found
- Number of games where "FOX" was not found
- Percentage of games where "FOX" was not found
- Total time it took for all calculations
- Average time it took per calculation
