---
layout: opencs
title: Boolean Logic in Programming
description: Boolean Hacks For Students To Try In Java
comments: True
permalink: /csp/big-idea-3/boolean/p4/javahacks
---

# 🔑 Boolean Hacks in Java: Whack-A-Mole 

This notebook has short Boolean challenges. Edit the code where it says `TODO` to make it work.

## Challenge 1: Mole Hit
Set hit to true if the player hits the mole. Print "Hit!" if hit is true, otherwise print "Miss!".


```python
%%js

boolean hit = false; // Change to true if you hit the mole

// TODO: check if hit is true
if (hit) {
    System.out.println("Hit!");
} else {
    System.out.println("Miss!");
}
```


    <IPython.core.display.Javascript object>


## Challenge 2: Out of Lives
Each time the player misses a mole, they lose a life. If lives reach 0, print "Game Over!".


```python
%%js
let lives = 3; // Decrease when a mole is missed
let missed = true; // Change to true if player misses a mole

// TODO: decrease lives when a mole is missed
if (missed) {
    lives -= 1;
}

if (lives === 0) {
    console.log("Game Over!");
} else {
    console.log("Keep playing! Lives left: " + lives);
}
```

## Challenge 3: High Score Boost
If the player’s score reaches 10 or more, give them a bonus of +5 points.


```python
%%js
let score = 8; // Increase when a mole is hit

// TODO: add 5 bonus points when score is 10 or higher
if (score >= 10) {
    score += 5;
    console.log("New High Score! Bonus awarded.");
} else {
    console.log("Try again!");
}

console.log("Final score: " + score);
```

## Challenge 4: Bonus Mole Condition
A golden mole should only give a bonus if both goldenMole and hit are true. Fix the condition.


```python
%%js
let goldenMole = true;   // Special mole appears
let hit = true;          // Change to true if the mole is hit

// TODO: fix the condition so bonus only happens when BOTH are true
if (goldenMole || hit) { // <-- wrong condition
    console.log("Bonus!");
} else {
    console.log("No bonus.");
}
```
