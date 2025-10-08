---
title: Mathematical Operations in Javascript and Python - Hacks
description: Apply your skills of math, logic, and coding.
permalink: /csp/big-idea-3/mathematical-expressions/p4/hw-js
author_profile: False
hidden: True
---

# Basic Algebraic Math hacks

## Q1 (Exponents):
### A cube has a side length of 6 units. What is its volume?


```javascript

let side_len = 6;
let volume = side_len ** 3;

console.log(volume);
```


    <IPython.core.display.Javascript object>


## Q2 (PEMDAS):
### Evaluate the expression: 

(7+14)*5/12 + 2


```javascript

let expr = (7 + 14) * 5/12 + 2;
console.log(expr);
```


    <IPython.core.display.Javascript object>


## Q3 (Algorithm): 

Write JavaScript  code where you define variables and run commands that find the values of operations you apply onto them


```javascript

// Quick Fn for Euler's Totient (phi)
function phi(n) {
    // Function embedding! FP
    function gcd(a, b) {
        if (a === 0) {
            return b;
        }

        return gcd(b % a, a);
    }

    var result = 1;
    for (let i = 2; i < n; i++) {
        if (gcd(i, n) === 1) {
            result++;
        }
    }

    return result;
}

let n = 126;
console.log(phi(n));
```


    <IPython.core.display.Javascript object>


<img src="/images/3_3diagram.png" alt="Diagram showing mathematical operations" style="max-width:300px; margin:10px;">

