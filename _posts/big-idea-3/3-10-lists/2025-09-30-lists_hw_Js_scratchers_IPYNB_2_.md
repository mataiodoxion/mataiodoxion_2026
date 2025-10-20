---
title: Lists Hacks Javascript
description: This is a CSP Hacks notebook based on the lists lesson
permalink: /csp/big-idea-3/lists/p4/hw-js
author_profile: False
hidden: True
---

# Javascript Lists Homework

- After going through the lists lesson work on these hacks in your own repository

### Hack 1 – Add Up Numbers

Make a list of numbers. Write code to:
1. Find the total sum.
2. Find the average.


```javascript
// Hack 1 – Add Up Numbers
let numbers = [4, 7, 1, 9, 6, 7, 10];

// Write your code here:
let sum = numbers.reduce((acc, val) => acc + val, 0);
console.log(sum);

let average = numbers / numbers.length();
console.log(numbers);
```


    <IPython.core.display.Javascript object>


### Hack 2 – Count Repeats

Make a list with repeated items. Write code to count how many times each item appears.


```javascript
// Hack 2 – Count Repeats
let items = ["cat", "dog", "cat", "bird", "bird", "bird"];

// Write your code here:
let counts = {};
items.forEach(item => {
    counts[item] = (counts[item] || 0) + 1;
});
console.log(counts);
```


    <IPython.core.display.Javascript object>


### Hack 3 – Keep Only Evens

Make a list of numbers. Write code to create a new list with only even numbers.


```javascript
// Hack 3 – Keep Only Evens
let numbers = [3, 8, 5, 12, 7, 9, 13, 31, 66, 18];

// Write your code here:
let evens = numbers.filter(num => num % 2 === 0);
console.log(evens);
```


    <IPython.core.display.Javascript object>

