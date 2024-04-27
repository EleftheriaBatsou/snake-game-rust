## Snake Game in Rust

### Basic information about the Components of the Game

🐍 The Snake can move in all directions. If our Snake eats an apple, it will grow by one "point". When we push an arrow button, the head will move up or down or left or right. The Snake cannot touch itself. For example, if it's going to the left and I push the right key, it won't be able to go backward, this would cause the game to enter a fail state because the Snake cannot actually touch itself. If the Snake touches itself, it dies and we have to restart the game. Also, if the Snake hits the wall, it dies. Those are the two main failure states for the game.

🍎 The second component is the Apple. In most Snake games, the Apple appears at random places. With this game, you'll notice that the Apple appears at the same place every time we start the game. The same with the Snake. The Snake will appear at a fixed place every time we restart it.

🧱 The third component is the walls. The walls will be represented as a rectangle that goes around the entire border of our game board. The Snake cannot pass through the walls.

🕹️ Finally, we have the game board itself. This will be a 2D plane that the Snake moves along and that the Apple spawns in.

--

To run it `cargo run`.

--

_More info is coming soon. I'm working on a detailed article to explain how I built it_

Update: The 1st part of the article is ready, you can read it [here](https://eleftheriabatsou.hashnode.dev/tutorial-snake-game-in-rust-part-12).
