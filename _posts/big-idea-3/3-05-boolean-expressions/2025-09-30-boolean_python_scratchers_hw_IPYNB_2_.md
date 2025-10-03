---
title: Boolean Logic in Programming
description: Boolean Hacks For Students To Try In Python
permalink: /csp/big-idea-3/boolean-expressions/p4/hw-py
author_profile: False
published: False
---

# 🔑 Boolean Hacks in Python: A Few New Codes!

This notebook has short Boolean challenges. Edit the code where it says `TODO` to make it work.

## Challenge 1: Positive Number
Fix the condition so it prints `num is positive` if the number is greater than 0.


```python
num = 1000  # Try changing this number!

# TODO: change the condition
if num > 0:
    print(num, "is positive")
else:
    print(num, "is NOT positive")
```

    1000 is positive


## Challenge 2: Is Even?
Change the condition so it prints `num is even` when the number is divisible by 2.


```python
num = 7

# TODO: check if num is even
if not num & 1: # or `num % 2 == 0``, but `not num & 1` is a bit faster in py
    print(num, "is even")
else:
    print(num, "is odd")
```

    7 is odd


## Challenge 3: Teenager Check
Print `Teenager` if age is between 13 and 19 (inclusive). Otherwise print `Not Teenager`. Fix the condition.


```python
age = 15

# TODO: fix the condition
if age >= 13 and age <= 19:
    print("Teenager")
else:
    print("Not Teenager")
```

    Teenager


## Challenge 4: Lamp Logic
We have two switches: `a` and `b`. The lamp should turn ON if **at least one** is True.

Fix the condition.


```python
a, b = True, False

# TODO: fix condition for at least one True
if a or b: 
    print("Lamp is ON")
else:
    print("Lamp is OFF")
```

    Lamp is ON

