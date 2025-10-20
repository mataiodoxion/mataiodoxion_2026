---
title: Escape Room 3.15 - Hacks & Extensions
description: Extended Python challenges and hacks for CSP 3.15 Escape Room - Random Values
permalink: /csp/big-idea-3/random-values/p4/hw-py
author_profile: False
hidden: True
---

# 🚀 Escape Room Python Hacks


1. Simulate rolling two dice and show all possible sums of their outcomes.


```python
# 🎲 Die faces: 1, 2, 3, 4, 5, 6
# TODO: Write code to calculate and display all possible sums
# Hint: You may need two nested loops
for i in range(1, 7):
    for j in range(1, 7):
        print(f"({i}, {j})")

```

2. Create a fortune teller program. The response "Try Again" should appear 40% of the time, while the other possible responses are "Yes", "No", and "Maybe".


```python
import random

num = random.randint(1, 100)
# 🔮 Fortune Teller
# TODO: Generate a random number between 1 and 100

if num <= 40:
    print("Try Again")
elif num <= 60:
    print("Yes")
elif num <= 80:
    print("No")
else:
    print("Maybe")

```

    Try Again


3. Create a program that randomly selects a meal from a menu list. For example: "Pizza", "Burger", "Salad", "Pasta", "Sushi".


```python
import random

# 🍔 Menu list of meals
meals = ["Banana", "Papaya", "Mango", "Pineapple"]

# TODO: Pick one random meal from the list
index = random.randint(0, len(meals) - 1)
print(meals[index])
```

    Banana


4. Practice using random.choice() and random.shuffle().

- Use random.choice() to select one random card from a deck.

- Use random.shuffle() to shuffle the entire deck.


```python
import random

# 🃏 Deck of cards (simplified as numbers 1–10 for this example)
deck = [1,2,3,4,5,6,7,8,9,10]

# TODO: Use random.choice() to pick one card
print(random.choice(deck))
# TODO: Use random.shuffle() to shuffle the deck and print it
random.shuffle(deck)
print(deck)
```

    9
    [9, 3, 1, 10, 4, 5, 7, 8, 2, 6]

