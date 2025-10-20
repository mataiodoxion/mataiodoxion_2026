---
title: Crashers - 3.17 Algorithmic Efficiency Javascript Hacks
description: Learn about algorithms and how they can be more or less efficient
permalink: /csp/big-idea-3/algorithmic-efficiency/p4/hw-js
author_profile: False
hidden: True
---

## Algorithmic Efficiency Hacks: Javascript

Let's test your knowledge on algorithmic efficiency!

### Hack 1: How Much Time?

#### Objective: write the time complexity of the algorithm below using Big-O notation.
(don't worry about special cases such as n = 1 or n = 0).


```javascript
%%javascript
let n = 5; // change this value to test different outputs!

for (let i = 0; i < n * 2; i++) {
    console.log(i);
}

//TODO: print the above algorithm's time complexity
//BE CAREFUL - This one might trick some people. Remember that Big-O notation shows how much an algorithm's time complexity GROWS, so coefficients don't matter...
console.log("O(n)");
```


    <IPython.core.display.Javascript object>


### Hack 2: Your Turn!

#### Objective: <strong>write</strong> an algorithm with O(n^2) time complexity.


```javascript
%%javascript
const n = 10; // change this if you want.

//TODO: Write an algorithm with O(n^2) time complexity
//Hint: think about nested loops...
for (let i = 0; i < n; i++) {
    for (let j = 0; j< n; j++) {
        console.log(`(${i}, ${j})`);
    }
}
```


    <IPython.core.display.Javascript object>


### Hack 3: Gotta Go Fast!

#### Objective: Optimize this algorithm so that it has a lower time complexity <strong>without modifying the outer loop</strong>


```javascript
%%javascript
const n = 10; // change this
let count = 0;

for(let i = 0; i < n; i++){ //Outer loop, DO NOT MODIFY
    count += i;
}
console.log(count);

//TODO: Modify the algorithm so that it has a lower time complexity but same output, and keep the outer loop the same
//Hint: This algorithm has a time complexity of O(n^2).
```


    <IPython.core.display.Javascript object>


### Hack 4: Extra Challenge 

#### Objective: Write an algorithm that does <strong>NOT</strong> have a time complexity of O(1), O(n), or O(n^x) and identify the time complexity
##### (I will not accept O(n^3) or some other power, it needs to be more complex.)


```javascript

// TODO: Write an algorithm that has a more complicated time complexity than O(n^x).
function fib(n) {
    if (n <= 1) {
        return n;
    } else {
        return fib(n-1) + fib(n-2);
    }
}

console.log(fib(5))
```


    <IPython.core.display.Javascript object>

