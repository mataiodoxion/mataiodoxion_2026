---
title: Mathematical Operations in Javascript and Python - Hacks
description: Apply your skills of math, logic, and coding.
permalink: /csp/big-idea-3/mathematical-expressions/p4/hacks-py
author_profile: False
---

# Basic Algebraic Math hacks

## Q1 (Exponents):
### A cube has a side length of 4 units. What is its volume?


```python
side_len = 4
vol = side_len ** 4
print(vol)
```

    256


## Q2 (PEMDAS):
### Evaluate the expression: 

 (12+8)/2+(3^2)


```python
expr = (12 + 8) / 2 + (3 ^ 2)
print(expr)
```

    11.0


## Q3 (Algorithm): 

Write Python  code where you define variables and run commands that find the values of operations you apply onto them


```python
def gcd(a, b):
    if (a == 0):
        return b
    return gcd(b % a, a)
 

def phi(n):
    count = 0
    for i in range(1, n):
        if gcd(i, n) == 1:
            count += 1
    return count

n = 126
print(phi(n))
```

    36


<img src="/images/3_3diagram.png" alt="Diagram showing mathematical operations" style="max-width:300px; margin:10px;">

