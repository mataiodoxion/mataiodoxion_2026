---
title: JavaScript Homework Iterations
description: Homework for Iterations lesson in Python
permalink: /csp/big-idea-3/scratchers/iterations/hw-js
author_profile: False
hidden: True
---

# 3.8 Iterations Homework in JavaScript

Complete the 4 tasks below as homework:

1. Uncomment the base code so you can work on adding what you need to produce the correct outputs  
2. Use `console.log()` to display outputs  
3. Some blanks (`_`) need to be replaced with numbers or code  
4. Important!!! Under each JavaScript hack, be sure to add an image or screenshot of your outputs in console 
4. Have fun!



### 🧩 Task #1
Write a `for` loop that prints the numbers **1 through 10**.


```javascript
for (let i = 1; i <= 10; i++) {
    console.log(i);
}

```

### 🧩 Task #2
Write a `while` loop that prints the even numbers between **2 and 20**.


```javascript
let num = 2;
while (num <= 20) {
    console.log(num);
    num += 2;
}

```

### 🧩 Task #3
Make the following shape using iteration loops with the symbol `*`:
```
*
**
***
****
*****
```


```javascript
let stars = "";
for (let i = 1; i <= 5; i++) {
    stars += "*";
    console.log(stars);
}

```


    <IPython.core.display.Javascript object>


### 🧩 Task #4
Use a loop to calculate the **sum of all numbers from 1 to 100**, and print the result.


```javascript
let sum = 0;
for (let i = 1; i <= 100; i++) {
    sum += i;
}
console.log("The sum is:", sum);

```


    <IPython.core.display.Javascript object>

