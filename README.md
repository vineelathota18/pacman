# Pacman Game in Rust + Yew

### Install the required tools

    cargo install trunk

    rustup target add wasm32-unknown-unknown

### Build and run the development server
    trunk serve

### Game Features:
Start Game <br/>
Restart Game <br/>
Play Again Once you WIN!




### Aggressiveness Probability

Each ghost has unique behavior: <br/>
Red: 100% chance to chase directly <br/>
Pink: 40% chance to ambush ahead of Pacman <br/>
Blue: 30% chance to flank from side <br/>
Orange: 0% very less chance to follow, often wanders randomly

### Best Move Calculation

Each ghost calculates possible moves (up, down, left, right) <br/>
Filters out invalid moves (walls and out-of-bounds)

#### For each valid move:
Calculates Manhattan distance to Pacman <br/>
Weighs move based on ghost's personality <br/>
Applies randomization factor