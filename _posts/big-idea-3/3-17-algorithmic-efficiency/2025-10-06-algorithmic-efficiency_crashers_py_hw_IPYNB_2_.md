---
title: Crashers - 3.17 Algorithmic Efficiency Python Hacks
description: Learn about algorithms and how they can be more or less efficient
permalink: /csp/big-idea-3/algorithmic-efficiency/p4/hw-py
author_profile: False
hidden: True
---

## Algorithmic Efficiency Hacks: Python

Let's test your knowledge on algorithmic efficiency!

### Hack 1: How Much Time?

#### Objective: write the time complexity of the algorithm below using Big-O notation.
(don't worry about special cases such as n = 1 or n = 0).


```python
n = int(input()) # remember what O(n) means? This is a good way of visualizing n.

for i in range(n):
    print(i)

#TODO: print the above algorithm's time complexity
print("O(n)")
```

    0
    1
    2
    O(n)


### Hack 2: Your Turn!

#### Objective: <strong>write</strong> an algorithm with O(n^2) time complexity.


```python
n = int(input())

#TODO: Write an algorithm with O(n^2) time complexity
#Hint: think about nested loops...
for i in range(n):
    for j in range(n):
        print(i)
```

    0
    0
    1
    1


### Hack 3: Gotta Go Fast!

#### Objective: Optimize this algorithm so that it has a lower time complexity <strong>without modifying the outer loop</strong>


```python
import math
n = int(input())
count = 0

for i in range(n):
    count += n * math.ceil(math.sqrt(n)*2)

print(count)

#TODO: make this algorithm more efficient, but keep the outer loop and make sure the output is still the same!
#Hint: how does the inner loop affect time complexity?
```

    36


### Hack 4: Extra Challenge 

#### Objective: Write an algorithm that does <strong>NOT</strong> have a time complexity of O(1), O(n), or O(n^x) and identify the time complexity
##### (I will not accept O(n^3) or some other power, it needs to be more complex.)


```python
n = int(input())

#TODO: Write an algorithm that has a more complicated time complexity than O(n^x).
# Calculate the Nth Fibonacci number recursively
# O(2^n)
def fib(n):
    if n <= 1:
        return n
    else:
        return fib(n-1) + fib(n-2)

print(fib(n))
```

    8

