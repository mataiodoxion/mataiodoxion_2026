---
title: 3.1 Variables and Assignments - Python vs JavaScript
description: Learn variables and assignments through the Peppa Pig Maze game
permalink: /csp/big-idea-3/variables-and-assignments/p4/introduction
author_profile: False
---

# 3.1 Variables & Data Types: Peppa's Maze Adventure

## What Are Variables?

Variables are **named storage locations** that hold data your program needs. Think of them as labeled boxes where you can store information and retrieve it later.

In the Peppa Pig Maze game, variables remember important information:
- Where Peppa is standing in the maze
- How many moves she has left
- Whether it's Peppa's turn or George's turn
- What score each player has

**Why we need variables:**
Without variables, the game would forget Peppa's position every time she moves! Variables let programs remember and update information as the game runs.

## Creating Variables

### Python - Simple Assignment

Python creates variables by just assigning a value:


```python
# No special keyword needed
peppa_x = 0
peppa_y = 0
player_name = "Peppa"
is_moving = False
dice_roll = 4

# Print the variables to see their values
print(f"Peppa's X position: {peppa_x}")
print(f"Peppa's Y position: {peppa_y}")
print(f"Player name: {player_name}")
print(f"Is moving: {is_moving}")
print(f"Dice roll: {dice_roll}")
```

**Variable anatomy:**
- `peppa_x` = variable name
- `=` = assignment operator (means "store this value")
- `0` = the value to store

### JavaScript - Use let or const

JavaScript requires a keyword when creating variables:


```python
// Use let for values that change
let peppaX = 0;
let movesLeft = 5;
let animating = false;

// Use const for values that stay the same
const CELL_SIZE = 60;
const peppaName = "Peppa";
const MAZE_WIDTH = 10;

// Display the variables
console.log("Peppa's X position:", peppaX);
console.log("Moves left:", movesLeft);
console.log("Animating:", animating);
console.log("Cell size:", CELL_SIZE);
console.log("Peppa's name:", peppaName);
console.log("Maze width:", MAZE_WIDTH);
```

**When to use each:**
- `let` - the variable's value will change during the game
- `const` - the variable's value stays constant throughout

---

## Naming Conventions

Different languages have different naming styles. The Peppa Maze uses both!

### Python: snake_case

Python variables use **lowercase with underscores**:


```python
peppa_position = 0
current_player = 1
moves_remaining = 5
waiting_for_answer = False
cell_size = 60

print("Python snake_case variables:")
print(f"peppa_position = {peppa_position}")
print(f"current_player = {current_player}")
print(f"moves_remaining = {moves_remaining}")
print(f"waiting_for_answer = {waiting_for_answer}")
print(f"cell_size = {cell_size}")
```

**Rules:**
- All lowercase letters
- Use underscores between words
- Descriptive names (not just `x` or `p`)

### JavaScript: camelCase

JavaScript variables use **no spaces with capital letters for new words**:


```python
let peppaPosition = 0;
let currentPlayer = 1;
let movesRemaining = 5;
let waitingForAnswer = false;
const cellSize = 60;

console.log("JavaScript camelCase variables:");
console.log("peppaPosition =", peppaPosition);
console.log("currentPlayer =", currentPlayer);
console.log("movesRemaining =", movesRemaining);
console.log("waitingForAnswer =", waitingForAnswer);
console.log("cellSize =", cellSize);
```

**Rules:**
- First word lowercase
- Capitalize the first letter of each new word
- No underscores between words
- Descriptive names

### Side-by-Side Comparison

| Python | JavaScript | What it stores |
|--------|------------|----------------|
| `peppa_x` | `peppaX` | Peppa's X position |
| `moves_left` | `movesLeft` | Remaining moves |
| `current_player` | `currentPlayer` | Whose turn it is |
| `waiting_for_answer` | `waitingForAnswer` | Quiz state |

### Why naming matters

Good names make code readable:


```python
// Bad - unclear
let x = 5;
let p = 1;
let m = "Hi";

console.log("Bad naming:");
console.log("x =", x, "(what is this?)");
console.log("p =", p, "(what does p mean?)");
console.log("m =", m, "(unclear purpose)");

console.log("\n" + "=".repeat(30) + "\n");

// Good - clear meaning
let movesLeft = 5;
let currentPlayer = 1;
let message = "Hi";

console.log("Good naming:");
console.log("movesLeft =", movesLeft, "(clearly remaining moves)");
console.log("currentPlayer =", currentPlayer, "(clearly the active player)");
console.log("message =", message, "(clearly a message to display)");
```

---

## Updating Variables

Variables are called "variable" because their values can **vary** - they can change!

### Changing Number Values

The maze constantly updates variables as the game runs:


```python
# Python - Peppa moves right
peppa_x = 2          # Peppa starts at x = 2
print(f"Peppa starts at x = {peppa_x}")

peppa_x = peppa_x + 1  # Add 1 to current value
print(f"After moving right: peppa_x = {peppa_x}")

# Show the process step by step
print("\nStep by step:")
print("1. Read current value from peppa_x:", 2)
print("2. Calculate new value: 2 + 1 =", 2 + 1)
print("3. Store new value back in peppa_x:", peppa_x)
```


```python
// JavaScript - same logic
let peppaX = 2;
console.log("Peppa starts at x =", peppaX);

peppaX = peppaX + 1;  // Take old value, add 1, store result
console.log("After moving right: peppaX =", peppaX);

console.log("\nHow it works:");
console.log("1. Read current value from the variable (2)");
console.log("2. Calculate new value (2 + 1 = 3)");
console.log("3. Store new value back in the variable");
```

### Updating After Dice Roll


```python
let moves = 0;           // Start with no moves
const diceRoll = 4;      // Roll the dice
console.log("Starting moves:", moves);
console.log("Dice roll:", diceRoll);

moves = diceRoll;        // Store the roll
console.log("After dice roll: moves =", moves);

// Each time Peppa moves:
console.log("\nPeppa moves once:");
moves = moves - 1;  // Subtract 1
console.log("Moves remaining:", moves);

console.log("Peppa moves again:");
moves = moves - 1;
console.log("Moves remaining:", moves);
```

### Updating Booleans

Booleans flip between true and false:


```python
let animating = false;   // Not moving
console.log("Initially animating:", animating);

// Player presses a key
animating = true;        // Start moving
console.log("Player presses key - animating:", animating);

// Movement finishes
animating = false;       // Done moving
console.log("Movement finished - animating:", animating);
```

### Updating Strings

You can change strings too:


```python
let currentMessage = "Roll the dice";
console.log("Initial message:", currentMessage);

currentMessage = "Peppa's turn!";  // Changed the message
console.log("Updated message:", currentMessage);

currentMessage = "George's turn!"; // Changed again
console.log("Final message:", currentMessage);
```

---

## Variables Working Together

The maze uses multiple variables to track complete game state:


```python
// Position variables
let peppaX = 0;
let peppaY = 0;

// Game state variables
let movesLeft = 0;
let currentPlayer = 1;
let animating = false;

// Display variables
let turnMessage = "Peppa's Turn!";
const playerName = "Peppa";

console.log("=== Initial Game State ===");
console.log("Position: (" + peppaX + ", " + peppaY + ")");
console.log("Moves left:", movesLeft);
console.log("Current player:", currentPlayer);
console.log("Animating:", animating);
console.log("Turn message:", turnMessage);
console.log("Player name:", playerName);
```

**During a turn, these update:**

1. Dice roll: `movesLeft = 4`
2. Player moves: `peppaX = peppaX + 1`, `movesLeft = movesLeft - 1`
3. Animation starts: `animating = true`
4. Animation ends: `animating = false`
5. Turn switches: `currentPlayer = 2`, `turnMessage = "George's Turn!"`

Let's simulate a complete turn:


```python
// Starting state
let peppaX = 0;
let movesLeft = 0;
let currentPlayer = 1;
let animating = false;
let turnMessage = "Peppa's Turn!";

console.log("=== COMPLETE TURN SIMULATION ===");
console.log("Starting state:");
console.log("peppaX:", peppaX, "| movesLeft:", movesLeft, "| animating:", animating);

// 1. Dice roll
movesLeft = 4;
console.log("\n1. Dice roll - movesLeft =", movesLeft);

// 2. Player moves
peppaX = peppaX + 1;
movesLeft = movesLeft - 1;
console.log("2. Player moves - peppaX =", peppaX, "| movesLeft =", movesLeft);

// 3. Animation starts
animating = true;
console.log("3. Animation starts - animating =", animating);

// 4. Animation ends
animating = false;
console.log("4. Animation ends - animating =", animating);

// 5. Turn switches
currentPlayer = 2;
turnMessage = "George's Turn!";
console.log("5. Turn switches - currentPlayer =", currentPlayer, "| turnMessage =", turnMessage);

console.log("\nAll variables updated to reflect what's happening in the game!");
```

---

## Common Mistakes

### Wrong naming convention


```python
# Bad in Python (JavaScript style)
print("❌ Bad in Python:")
try:
    peppaPosition = 0  # This works but violates Python convention
    movesLeft = 5      # This works but violates Python convention
    print("peppaPosition =", peppaPosition, "(works but wrong style)")
    print("movesLeft =", movesLeft, "(works but wrong style)")
except Exception as e:
    print("Error:", e)

print("\n✅ Good in Python:")
peppa_position = 0  # Correct Python naming
moves_left = 5      # Correct Python naming
print("peppa_position =", peppa_position, "(correct snake_case)")
print("moves_left =", moves_left, "(correct snake_case)")
```


```python
// Bad in JavaScript (Python style)
console.log("❌ Bad in JavaScript:");
// let peppa_position = 0;  // This works but violates JavaScript convention
// let moves_left = 5;      // This works but violates JavaScript convention
console.log("peppa_position and moves_left work but violate JavaScript conventions");

console.log("\n✅ Good in JavaScript:");
let peppaPosition = 0;  // Correct JavaScript naming
let movesLeft = 5;      // Correct JavaScript naming
console.log("peppaPosition =", peppaPosition, "(correct camelCase)");
console.log("movesLeft =", movesLeft, "(correct camelCase)");
```

### Using variable before creating it


```python
console.log("❌ Bad - trying to use before creating:");
try {
    // This will cause an error because movesLeft doesn't exist yet
    // movesLeft = movesLeft + 5;  // ERROR! movesLeft doesn't exist yet
    console.log("Can't use movesLeft before creating it - would cause ReferenceError");
} catch (error) {
    console.log("Error:", error.message);
}

console.log("\n✅ Good - create it first:");
let movesLeft = 0;          // Create the variable first
movesLeft = movesLeft + 5;  // Now it works
console.log("movesLeft =", movesLeft, "(created first, then used)");
```

### Forgetting quotes for strings


```python
console.log("❌ Bad - no quotes:");
try {
    // let name = Peppa;  // ERROR! Thinks Peppa is a variable
    console.log("'let name = Peppa;' would cause ReferenceError - thinks Peppa is a variable");
} catch (error) {
    console.log("Error:", error.message);
}

console.log("\n✅ Good - use quotes:");
let name = "Peppa";  // Correctly stores text
console.log("name =", name, "(correctly stores text in quotes)");
```

---

## Key Takeaways

**What are variables?**
- Named storage locations for data
- Remember information while program runs
- Can be read and updated as needed

**Creating variables:**
- **Python**: Just assign: `name = "Peppa"`
- **JavaScript**: Use `let` or `const`: `let name = "Peppa";`

**Naming:**
- **Python**: `snake_case` (underscores)
- **JavaScript**: `camelCase` (capital letters for new words)
- Always use descriptive names

**Updating:**
- Variables can change: `x = x + 1`
- Strings, numbers, and booleans can all be updated
- Updates happen constantly as the game runs

Now you understand how variables store and track information in the Peppa Maze game!
