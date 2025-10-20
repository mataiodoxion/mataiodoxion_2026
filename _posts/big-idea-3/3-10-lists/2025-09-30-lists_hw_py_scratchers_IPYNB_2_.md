---
title: 3.10 Lists Hacks Python
description: This is a CSP Hacks notebook based on the lists lesson
permalink: /csp/big-idea-3/lists/p4/hw-py
author_profile: False
hidden: True
---

# Python Lists Homework

- After going through the lists lesson work on these hacks in your own repository

### Hack 1 – Add Up Numbers

Make a list of numbers. Write code to:
1. Find the total sum.
2. Find the average.


```python
# Hack 1 – Add Up Numbers
numbers = [4, 7, 1, 9, 6, 7, 10]

# Write your code here:
sum = 0
for number in numbers:
    sum += number
print(sum)

average = sum / len(numbers)
print(average)
```

    44
    6.285714285714286


### Hack 2 – Count Repeats

Make a list with repeated items. Write code to count how many times each item appears.


```python
# Hack 2 – Count Repeats
items = ["cat", "dog", "cat", "bird", "bird", "bird"]

# Write your code here:
counts = {}
for item in items:
    if item in counts:
        counts[item] += 1
    else:
        counts[item] = 1

print(counts)
```

    {'cat': 2, 'dog': 1, 'bird': 3}


### Hack 3 – Keep Only Evens

Make a list of numbers. Write code to create a new list with only even numbers.


```python
# Hack 3 – Keep Only Evens
numbers = [3, 8, 5, 12, 7, 9, 13, 31, 66, 18]

# Write your code here:
for number in numbers:
    if number & 1:
        numbers.remove(number)
print(numbers)
```

    [8, 12, 9, 31, 66, 18]

