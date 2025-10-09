const canvas = document.getElementById('game');
const ctx = canvas.getContext('2d');
const scoreEl = document.getElementById('score');

// Grid settings mirroring Rust version (30x30)
const GRID_W = 30;
const GRID_H = 30;
const CELL = canvas.width / GRID_W; // 600/30 = 20px

// Colors aligned with styles.css and README
const COLORS = {
  wall: getCssVar('--wall'),
  food: getCssVar('--food'),
  snake: getCssVar('--snake'),
  gameover: getCssVar('--gameover'),
};

function getCssVar(name){
  return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
}

// Game constants loosely matching Rust config.rs intent
const MOVING_PERIOD = 0.12;     // seconds between moves
const RESTART_TIME = 1.8;       // seconds before auto-restart
const START_SNAKE_X = 2;
const START_SNAKE_Y = 2;

const INITIAL_FOOD_X = 6;
const INITIAL_FOOD_Y = 4;

const Direction = {
  Up: 'Up',
  Down: 'Down',
  Left: 'Left',
  Right: 'Right',
};

class Snake {
  constructor(x, y){
    this.body = [{x, y}, {x: x-1, y}, {x: x-2, y}];
    this.dir = Direction.Right;
    this.tailBackup = null;
  }

  head(){
    return this.body[0];
  }

  headDirection(){
    return this.dir;
  }

  opposite(d){
    switch(d){
      case Direction.Up: return Direction.Down;
      case Direction.Down: return Direction.Up;
      case Direction.Left: return Direction.Right;
      case Direction.Right: return Direction.Left;
    }
  }

  nextHead(optionalDir){
    const d = optionalDir || this.dir;
    const {x, y} = this.head();
    switch(d){
      case Direction.Up: return {x, y: y-1};
      case Direction.Down: return {x, y: y+1};
      case Direction.Left: return {x: x-1, y};
      case Direction.Right: return {x: x+1, y};
    }
  }

  overlapTail(x, y){
    return this.body.slice(1).some(s => s.x === x && s.y === y);
  }

  moveForward(optionalDir){
    const d = optionalDir || this.dir;
    this.dir = d;

    const next = this.nextHead(d);
    this.body.unshift(next);
    this.tailBackup = this.body.pop();
  }

  restoreTail(){
    if (this.tailBackup) {
      this.body.push(this.tailBackup);
      this.tailBackup = null;
    }
  }

  draw(){
    ctx.fillStyle = COLORS.snake;
    for (const s of this.body) {
      drawCell(s.x, s.y);
    }
  }
}

class Game {
  constructor(width, height){
    this.width = width;
    this.height = height;

    this.snake = new Snake(START_SNAKE_X, START_SNAKE_Y);

    this.foodExists = true;
    this.foodX = INITIAL_FOOD_X;
    this.foodY = INITIAL_FOOD_Y;

    this.gameOver = false;
    this.waitingTime = 0;
    this.paused = false;
    this.score = 0;
    scoreEl.textContent = '0';
  }

  keyPressed(key){
    if (key === ' ') { // Space
      this.paused = !this.paused;
      return;
    }
    if (key === 'Escape') {
      // Restart immediately on Esc
      this.restart();
      return;
    }
    if (this.gameOver) return;

    let dir = null;
    switch(key){
      case 'ArrowUp': dir = Direction.Up; break;
      case 'ArrowDown': dir = Direction.Down; break;
      case 'ArrowLeft': dir = Direction.Left; break;
      case 'ArrowRight': dir = Direction.Right; break;
    }

    if (dir) {
      if (dir === this.snake.opposite(this.snake.headDirection())) {
        return;
      }
      if (!this.paused) {
        this.updateSnake(dir);
      }
    }
  }

  draw(){
    // background
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // snake
    this.snake.draw();

    // food
    if (this.foodExists) {
      ctx.fillStyle = COLORS.food;
      drawCell(this.foodX, this.foodY);
    }

    // walls as 1-cell border
    ctx.fillStyle = COLORS.wall;
    drawRect(0, 0, this.width, 1);                 // top
    drawRect(0, this.height - 1, this.width, 1);   // bottom
    drawRect(0, 0, 1, this.height);                // left
    drawRect(this.width - 1, 0, 1, this.height);   // right

    if (this.gameOver) {
      ctx.fillStyle = COLORS.gameover;
      drawRect(0, 0, this.width, this.height);
    }
  }

  update(dt){
    this.waitingTime += dt;

    if (this.gameOver) {
      if (this.waitingTime > RESTART_TIME) {
        this.restart();
      }
      return;
    }
    if (this.paused) return;

    if (!this.foodExists) this.addFood();
    if (this.waitingTime > MOVING_PERIOD) {
      this.updateSnake(null);
    }
  }

  checkEating(){
    const head = this.snake.head();
    if (this.foodExists && this.foodX === head.x && this.foodY === head.y) {
      this.foodExists = false;
      this.snake.restoreTail();
      this.score += 1;
      scoreEl.textContent = String(this.score);
    }
  }

  checkIfSnakeAlive(dir){
    const next = this.snake.nextHead(dir);

    if (this.snake.overlapTail(next.x, next.y)) return false;

    return next.x > 0 && next.y > 0 &&
           next.x < this.width - 1 &&
           next.y < this.height - 1;
  }

  addFood(){
    let newX, newY;
    do {
      newX = randInt(1, this.width - 2);
      newY = randInt(1, this.height - 2);
    } while (this.snake.overlapTail(newX, newY));

    this.foodX = newX;
    this.foodY = newY;
    this.foodExists = true;
  }

  updateSnake(dir){
    if (this.checkIfSnakeAlive(dir)) {
      this.snake.moveForward(dir);
      this.checkEating();
    } else {
      this.gameOver = true;
    }
    this.waitingTime = 0;
  }

  restart(){
    this.snake = new Snake(START_SNAKE_X, START_SNAKE_Y);
    this.waitingTime = 0;
    this.foodExists = true;
    this.foodX = INITIAL_FOOD_X;
    this.foodY = INITIAL_FOOD_Y;
    this.gameOver = false;
    this.paused = false;
    this.score = 0;
    scoreEl.textContent = '0';
  }
}

// Utilities
function drawCell(x, y){
  ctx.fillRect(x * CELL, y * CELL, CELL, CELL);
}

function drawRect(x, y, w, h){
  ctx.fillRect(x * CELL, y * CELL, w * CELL, h * CELL);
}

function randInt(min, maxInclusive){
  return Math.floor(Math.random() * (maxInclusive - min + 1)) + min;
}

// Input
window.addEventListener('keydown', (e) => {
  game.keyPressed(e.key);
});

// Main loop (fixed-step like)
let last = performance.now();
const game = new Game(GRID_W, GRID_H);

function loop(now){
  const dt = (now - last) / 1000;
  last = now;
  game.update(dt);
  game.draw();
  requestAnimationFrame(loop);
}
requestAnimationFrame(loop);