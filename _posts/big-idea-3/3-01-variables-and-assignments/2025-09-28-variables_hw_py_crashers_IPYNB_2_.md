---
title: 3.1 Variables & Assignments hacks - Python
description: Apply your understanding of Variables & Assignments with these Python hacks
permalink: /csp/big-idea-3/variables-and-assignments/p4/hacks-py
author_profile: False
hidden: True
---

# 🐷 Peppa Maze Variables & Assignments Hacks (Python)
Welcome to the Peppa Maze hacks! These challenges will test your understanding of variables, assignments, and logic in Python. Read each task, then write or modify code to solve it.

## Hack 1: Python - Variable Assignment, Naming, and Operators
Create variables for Peppa's name, score, and level using good Python naming conventions. Assign initial values, then use operators to update score (add 10) and level (multiply by 2). Print all results.


```python
# Write your code here
# Create variables for Peppa's name, score, and level
# Use operators to update score and level

# In the context of this code snippet, it would be bad convention to make hyperspecific vars here
name = "Peppa"
score = 0
level = 1
print(f'name: {name}, score: {score}, level: {level}')

score += 10;
level *= 2;
print(f'name: {name}, score: {score}, level: {level}')
```

    name: Peppa, score: 0, level: 1
    name: Peppa, score: 10, level: 2


## Hack 2: Python - Variable Declaration, Assignment, and Operators
Declare variables for Peppa and George's scores using good Python naming conventions. Assign initial values, then use operators to update both scores (e.g., Peppa gets 15 points, George loses 5 points). Print both scores.


```python
# Write your code here
# Declare variables for Peppa and George's scores
# Use operators to update both scores
peppa_score = 0
george_score = 0
print(f'peppa_score: {peppa_score}, george_score: {george_score}')

peppa_score += 15
george_score -= 5
print(f'peppa_score: {peppa_score}, george_score: {george_score}')
```

    peppaScore: 0, georgeScore: 0
    peppaScore: 15, georgeScore: -5


## Hack 3: Python - Multiple Assignment and Math Operators
Peppa and George both start at level 1. Use Python's multiple assignment feature to assign both their levels to 5 in one line. Then, calculate a combined score by multiplying their levels together and multiplying by 10. Print all results.


```python
# Write your code here
# assign peppa level
# assign george level
# Assign both levels to 5 in one line using multiple assignment
# combined_score = peppa_level * george_level * 10
# Print all results
peppa_level, george_level = 1, 1
print(f'peppa_level: {peppa_level}, george_level: {george_level}')

peppa_level, george_level = 5, 5
print(f'peppaScore: {peppaScore}, georgeScore: {georgeScore}')

combined_score = peppa_level * george_level * 10
print(f'peppaScore: {peppaScore}, georgeScore: {georgeScore}, combined_score = {combined_score}')
```

    peppaLevel: 1, georgeLevel: 1

