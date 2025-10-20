---
title: Escape Room 3.15 - Hack
description: Extended Javascript challenges and hacks for CSP 3.15 Escape Room - Random Values
permalink: /csp/big-idea-3/random-values/p4/hw-js
author_profile: False
hidden: True
---

# 🚀 Escape Room Javascript Hacks

1. Simulate drawing two random cards, where each card has a value between 1 and 10. Show the two card values and their total.



```javascript
// 🎴 Simulate drawing two random cards between 1 and 10
const min = 1;
const max = 10;
// TODO: Generate a random value between 1 and 10 for the first card
let card1 = Math.floor(Math.random() * (max - min + 1) + min);

// TODO: Generate a random value between 1 and 10 for the second card
let card2 = Math.floor(Math.random() * (max - min + 1) + min);

// TODO: Add the two card values together
let total = card1 + card2;

// TODO: Print out the results
console.log("Card 1:", card1);
console.log("Card 2:", card2);
console.log("Total:", total);

```


    <IPython.core.display.Javascript object>


2. Create a decision maker where "Definitely" appears 30% of the time


```javascript
// 🎲 Decision Maker
// Goal: "Definitely" should appear about 30% of the time when you run this code

const min = 1;
const max = 100;
// TODO: Generate a random number between 1 and 100
let rand = Math.floor(Math.random() * (max - min + 1) + min);

// TODO: Use an if statement to make "Definitely" appear 30% of the time
if (rand <= 30) {
  console.log("Definitely");
} else {
  console.log("Not this time");
}

```


    <IPython.core.display.Javascript object>


3. Simulate one coin flip and one dice roll.

- The coin flip should be 0 = Heads or 1 = Tails.

- The dice roll should be a number between 1 and 6.

- Print out both results.


```javascript
// 🪙 Simulate one coin flip and one dice roll

// TODO: Generate a random value 0 or 1 for the coin
let coin = Math.round(Math.random());

// TODO: Print out the coin flip result
console.log("Coin flip:", coin);

const min = 1;
const max = 6;
// TODO: Generate a random value between 1 and 6 for the dice
let dice = Math.floor(Math.random() * (max - min + 1) + min);

// TODO: Print out the dice roll result
console.log("Dice roll:", dice);

```


    <IPython.core.display.Javascript object>


4. Write code that randomly picks one fortune from a list of 5 possible fortunes and prints it out.


```javascript
// 🔮 Random Fortune Generator

// TODO: Make a list (array) of fortunes
let fortunes = ["Run", "Behind you", "There's footsteps", "You're not crazy", "Boo"];

// TODO: Pick a random index
let index = Math.floor(Math.random() * 5);

// TODO: Print out the fortune at that index
console.log("Your fortune:", fortunes[index]);

```


    <IPython.core.display.Javascript object>

