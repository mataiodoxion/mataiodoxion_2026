---
title: Remakers - 3.13 Developing Procedures JS Hacks
author: Krishna Visvanath, Sloane Sommers
description: Apply your skills to basic procedure development in Javascript.
permalink: /csp/big-idea-3/developing-procedures/p4/hw-js
author_profile: False
hidden: True
---

### Q1 (Easy)
Which of these procedures is named wrong, provide a **short** explanation of justification


```javascript

function mixIngredients()

function doIt()

function makeLeftTurn()
```

**Explanation Here:**
`doIt()` is wrong because its naming is vague and generally does not abide my good naming conventions.

### Q2 (Medium)
Finish the code to have a correctly named procedure


```javascript
function makeLeftTurn() {
    moveForward() ;
    rotateLeft() ;
    moveForwardAgain() ;
}
//Todo: add corresponding procedure definitions
//here- replace this line with your code {
function moveForward() {
    console.log("Moving forward.");
}

//here- replace this line with your code {
function rotateLeft() {
    console.log("Turning left");
}

function moveForwardAgain() {
    console.log("Moving forward again to complete left turn.");
}

//Run the procedure
makeLeftTurn()
```

### Q3 (Hard)
Write code to fulfill the requirements
Doing a dance! 🕺💃
Must have
1. A shimmy left procedure
- Print `super cool left slide`
2. A shimmy right procedure, print `even cooler right slide`
3. Doing a bow to the crowd, print `Great dance!`, `the audience claps at your bow!`


```javascript

function sillyDance() {
    leftSlide();
    rightSlide();
    bow();
}

function leftSlide() {
    console.log("Super cool left slide");
}

function rightSlide() {
    console.log("Even cooler right slide");
}

function bow() {
    console.log("Great dance!\nThe audience claps and at your bow!")
}
```

![silly dance](https://i.redd.it/7bb0v1q2j9ce1.gif)
