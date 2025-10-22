---
title: 3.9 Developing Algos Python
permalink: /csp/big-idea-3/developing-algorithms/p4/hw-py
author_profile: False
hidden: True
---

# Peppa's Algorithm Adventures - Python Hacks

Welcome to Peppa's algorithm challenges! Complete these three hacks to master algorithm development with interactive Python examples.

## Hack 1: Peppa's Number Comparison Algorithm

Create algorithms that use Boolean expressions to compare numbers, just like in the lesson!

**Your task:** Complete the missing Boolean conditions to help Peppa make smart decisions.


```python
# Peppa's number comparison algorithms (like the lesson examples!)

def algorithm_a_find_maximum(a, b):
    """Algorithm A: Find the larger number using if-else"""
    # TODO: Complete the Boolean condition
    if a > b:  # This is given as an example
        return a
    else:
        return b

def algorithm_b_find_maximum(a, b):
    """Algorithm B: Same problem, different approach using Boolean expression"""
    # TODO: Complete this line using a Boolean expression (like lesson example)
    return a if a > b else b  # Replace _____ with your condition

def peppa_decision_maker():
    """Help Peppa make decisions using Boolean logic"""
    
    peppa_coins = 15
    toy_price = 12
    
    print("Peppa's Decision Algorithm")
    print(f"Peppa has {peppa_coins} coins")
    print(f"Toy costs {toy_price} coins")
    
    # TODO: Complete the Boolean condition
    can_buy_toy = peppa_coins >= toy_price  # Replace _____ with comparison operator
    
    if can_buy_toy:
        print("Peppa can buy the toy!")
    else:
        print("Peppa needs more coins!")
    
    return can_buy_toy

# Test the algorithms (like the lesson does)
print("=== Testing Maximum Algorithms ===")
x, y = 10, 7
print(f"Algorithm A result: {algorithm_a_find_maximum(x, y)}")
print(f"Algorithm B result: {algorithm_b_find_maximum(x, y)}")

print("\n=== Peppa's Decision ===")
peppa_decision_maker()
```

## Hack 2: George's Simple Movement Algorithm

Create a simple movement algorithm like the maze example from the lesson!

**Your task:** Complete the Boolean conditions to control George's movement.


```python
# George's movement algorithm (similar to the lesson's maze example)

def george_movement_algorithm():
    """Simple movement system using Boolean conditions"""
    
    # George's current position
    george_x = 2
    george_y = 1
    
    # Boundary limits (like the maze example)
    max_x = 4
    max_y = 3
    min_x = 0
    min_y = 0
    
    print("George's Movement Algorithm")
    print(f"George is at position ({george_x}, {george_y})")
    print(f"Boundaries: x(0-{max_x}), y(0-{max_y})")
    
    # Movement commands
    print("\n--- Testing Movement ---")
    
    # Try moving right
    new_x = george_x + 1
    # TODO: Complete the Boolean condition (like lesson's canPeppaMove function)
    can_move_right = new_x <= max_x  # Replace _____ with comparison operator
    
    print(f"Move right to ({new_x}, {george_y}): {'Valid' if can_move_right else 'Invalid'}")
    
    # Try moving up  
    new_y = george_y + 1
    # TODO: Complete the Boolean condition
    can_move_up = new_y <= max_y  # Replace _____ with comparison operator
    
    print(f"Move up to ({george_x}, {new_y}): {' Valid' if can_move_up else ' Invalid'}")
    
    # Try moving left
    new_x = george_x - 1
    # TODO: Complete the Boolean condition  
    can_move_left = new_x >= min_x  # Replace _____ with comparison operator
    
    print(f"Move left to ({new_x}, {george_y}): {' Valid' if can_move_left else ' Invalid'}")

def interactive_movement():
    """Let user test the movement algorithm"""
    print("\n Interactive Movement Test")
    
    x, y = 1, 1  # Starting position
    
    direction = input("Which way should George move? (up/down/left/right): ").lower()
    
    if direction == "right":
        new_x, new_y = x + 1, y
    elif direction == "left":
        new_x, new_y = x - 1, y
    elif direction == "up":
        new_x, new_y = x, y + 1
    elif direction == "down":
        new_x, new_y = x, y - 1
    else:
        print(" Invalid direction!")
        return
    
    # TODO: Complete the Boolean condition for boundaries
    is_valid_move = (0 <= new_x <= 4) and (0 <= new_y <= 3)  # Fill in the blanks
    
    if is_valid_move:
        print(f" George moved {direction} to ({new_x}, {new_y})")
    else:
        print(f" Can't move {direction} - out of bounds!")

# Run the algorithms
george_movement_algorithm()
interactive_movement()
```

## Hack 3: Peppa's Pathfinding Adventure

Create a pathfinding algorithm to help Peppa navigate through different terrains to reach her friends! This combines Boolean logic, conditional statements, and algorithm design.

**Your task:** Implement different pathfinding strategies and compare their effectiveness using interactive visualizations.


```python
# Simple maze pathfinding
def peppa_maze_pathfinder():
    """Help Peppa find her way through a simple maze"""
    
    # Simple 5x5 maze: 0=path, 1=wall, 2=start, 3=goal
    maze = [
        [2, 0, 1, 0, 0],
        [0, 0, 1, 0, 1], 
        [0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0],
        [1, 0, 0, 0, 3]
    ]
    
    def display_maze(path=None):
        symbols = {0: "", 1: "", 2: "", 3: ""}
        print("\nPeppa's Maze:")
        for r in range(5):
            row = ""
            for c in range(5):
                if path and (r, c) in path:
                    row += ""  # Yellow path
                else:
                    row += symbols[maze[r][c]]
            print(row)
    
    def is_valid_move(row, col):
        """
        TODO: Complete this function
        Return True if move is valid, False if not
        """
        # YOUR CODE HERE - Check if position is valid
        # Hint: Check bounds (0-4) and walls (maze[row][col] != 1)
        if 0 <= row < 5 and 0 <= col < 5 and maze[row][col] != 1:
            return True
        return False
    
    def find_path():
        """Simple pathfinding from start to goal"""
        start = (0, 0)  # Peppa's position
        goal = (4, 4)   # Friend's position
        
        # TODO: Complete this simple pathfinding
        # For now, just return a sample path
        sample_path = [(0,0), (1,0), (1,1), (2,1), (2,2), (3,2), (4,2), (4,3), (4,4)]
        
        # Check if path is valid
        valid_path = []
        for pos in sample_path:
            if is_valid_move(pos[0], pos[1]):
                valid_path.append(pos)
        
        return valid_path if len(valid_path) > 5 else None
    
    # Run the pathfinder
    print("Welcome to Peppa's Mini Maze!")
    display_maze()
    
    path = find_path()
    if path:
        print(" Path found!")
        display_maze(path)
        print(f"Path length: {len(path)} steps")
    else:
        print(" Complete the is_valid_move function to find the path!")

# Run the maze solver
peppa_maze_pathfinder()
```

## What You Should Complete

**After finishing the lesson, you should be able to:**

1. **Hack 1**: Fill in the Boolean comparison operators (`<=`, `>=`, `<`, `>`) to make the muddy puddle validator work
2. **Hack 2**: Complete the `if/elif/else` statements for George's number comparison
3. **Hack 3**: Fill in the boundary conditions for George's movement algorithm



