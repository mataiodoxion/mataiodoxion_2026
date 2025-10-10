---
title: Python Homework Iterations
description: Homework for Iterations lesson in Python
permalink: /csp/big-idea-3/iterations/p4/hw-py
author_profile: False
hidden: True
---

## 3.8 Iterations Homework in Python

Complete the 4 hacks down below as homework:
1. Uncomment the base code so that you can work on adding what you need to produce the correct outputs
2. Make sure you connect too the python kernal to display an output
3. Some questions require you to fill in the blanks. Beware of the '_' that you need to delete and replace with numbers or words
3. Make sure everything is re-indented properly before you run the code in the notebook
4. Have fun!

### Hack #1: Write a for loop that prints the numbers 1 through 10.


```python
for i in range(1,11):
  print(i)
```

    1
    2
    3
    4
    5
    6
    7
    8
    9
    10


### Hack #2: Write a while loop that prints the even numbers between 2 and 20.

Hint 1 for Hack #2: You need to remember your work with math equations. (What goes before the equal sign?)

Hint 2 for Hack #2: You need to remember variables. (What is the initial condition?)


```python
num = 2
while num <= 20:
  print(num)
  num += 2
```

    2
    4
    6
    8
    10
    12
    14
    16
    18
    20


### Hack #3: Shapes! Make the following shapes using iteration loops with the symbol `*`

This might be a little challenging but think about how you need your outputs to look.


```python
# 1. Right triangle (Don't uncomment this, they are instructions)
for i in range(1, 6):
   print("*" * i)

# 2. Square (5x5) (Don't uncomment this, they are instructions)
for i in range(5):
   print("*" * 5)

```

    *
    **
    ***
    ****
    *****
    *****
    *****
    *****
    *****
    *****


### Hack #4: Fruits! Choose your favorate fruits and list them using iterations

Pick you 3 favorte fruits and list them in the blanks


```python
fruits = ["banana", "papaya", "ahhhhhh"]

for i in range(len(fruits)):
   print(fruits[i])
```

    banana
    papaya
    ahhhhhh



```python
fruits = ["apple", "banana", "cherry"]

i = 0
while i < len(fruits): # Hint: Loop continues as long as i is less than the length of the fruits list (The length is 3)
   print(fruits[i]) #` Print the fruit at index i
   i += 1
```

    apple
    banana
    cherry

