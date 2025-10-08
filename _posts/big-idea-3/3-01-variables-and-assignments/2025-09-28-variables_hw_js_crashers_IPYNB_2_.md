---
title: 3.1 Variables & Assignments hacks
description: Apply your understanding of Variables & Assignments with these hacks
permalink: /csp/big-idea-3/variables-and-assignments/p4/hacks-js
author_profile: False
hidden: True
---

# 🐷 Peppa Maze Variables & Assignments Hacks
Welcome to the Peppa Maze hacks! These challenges will test your understanding of variables, assignments, and logic in Javascript. Read each task, then write or modify code to solve it.

## Hack 1: JavaScript - Variable Assignment, Naming, and Operators
Create variables for Peppa's name, score, and level using good JavaScript naming conventions. Assign initial values, then use operators to update score (add 10) and level (multiply by 2). Print all results.


```javascript

// Write your code here
// Create variables for Peppa's name, score, and level
// Use operators to update score and level

// In the context of this code snippet, it would be bad convention to make hyperspecific vars here
let name = "Peppa";
let score = 0;
let level = 1;
console.log(`name: ${name}, score: ${score}, level: ${level}`);

// Ops
score += 10;
level *= 2;
console.log(`name: ${name}, score: ${score}, level: ${level}`);
```


    <IPython.core.display.Javascript object>


## Hack 2: JavaScript - Variable Declaration, Assignment, and Operators
Declare variables for Peppa and George's scores using good JavaScript naming conventions. Assign initial values, then use operators to update both scores (e.g., Peppa gets 15 points, George loses 5 points). Print both scores.


```javascript

// Write your code here
// Declare variables for Peppa and George's scores
// Use operators to update both scores

// Again, here it would be better convention to name the scores `peppa` and `george` given the lack of context
let peppaScore = 0;
let georgeScore = 0;
console.log(`peppaScore: ${peppaScore}, georgeScore: ${georgeScore}`);

// ops
peppaScore += 15;
georgeScore -= 5;
console.log(`peppaScore: ${peppaScore}, georgeScore: ${georgeScore}`);
```


    <IPython.core.display.Javascript object>


## Hack 3: JavaScript - Multiple Assignment and Math Operators
Peppa and George both start at level 1. Use a single line to assign both their levels to 5. Then, calculate a combined score by multiplying their levels together and multiplying by 10. Print all results.


```javascript

// Write your code here
// let peppaLevel = 1, georgeLevel = 1;
// Assign both levels to 5 in one line
// Calculate combinedScore = peppaLevel * georgeLevel * 10;
// Print all results

let peppaLevel = 1, georgeLevel = 1;
console.log(`peppaLevel: ${peppaLevel}, georgeLevel: ${georgeLevel}`);

peppaLevel = 5, georgeLevel = 5;
console.log(`peppaLevel: ${peppaLevel}, georgeLevel: ${georgeLevel}`);

let combinedScore = peppaLevel * georgeLevel * 10;
console.log(`peppaLevel: ${peppaLevel}, georgeLevel: ${georgeLevel}, combinedScore: ${combinedScore}`);
```


    <IPython.core.display.Javascript object>

