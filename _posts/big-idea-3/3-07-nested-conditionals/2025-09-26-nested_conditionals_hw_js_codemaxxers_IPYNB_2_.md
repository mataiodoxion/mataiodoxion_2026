---
title: Nested Conditionals in Javascript - Homework
description: Apply your skills of multilayered conditionals and combine all you've learned so far.
permalink: /csp/big-idea-3/nested-conditionals/p4/hw-js
author_profile: False
published: False
---

# Nested Conditionals Homework: Javascript

Complete the following three problems to practice working with nested conditionals. Each problem increases in difficulty. **All problems must be completed in JavaScript.**

## Problem 1: Complete the Shipping Cost Calculator (Easy)

An online store calculates shipping costs based on the order total and membership status. The partially completed code below needs to be finished.

**Rules:**
- If order total is $50 or more:
  - Free shipping for members
  - $5 shipping for non-members
- If order total is less than $50:
  - $3 shipping for members
  - $8 shipping for non-members

**Your task:** Complete the nested conditional so that when you run the code with the given initial values (`orderTotal = 45` and `isMember = true`), the output shows a shipping cost of **$3**.


```python
%%js
// Initial conditions - DO NOT CHANGE THESE
let orderTotal = 45;
let isMember = true;
let shippingCost = 0;

console.log(`Order Total: $${orderTotal}`);
console.log(`Member: ${isMember}`);

// YOUR CODE HERE: Complete the nested conditional
if (orderTotal >= 50) {
    if (isMember) {
        shippingCost = 0;
    } else {
        shippingCost = 5;
    }
} else {
    if (isMember) {
        shippingCost = 3;
    } else {
        shippingCost = 8;
    }
}

console.log(`Shipping Cost: $${shippingCost}`);

// Expected output with given values: Shipping Cost: $3
```

## Problem 2: Build a Restaurant Recommendation System (Medium)

Create a system that recommends a restaurant based on the user's budget and cuisine preference.

**Requirements:**
- If budget is "high" (over $30 per person):
  - If cuisine is "italian": recommend "Bella Notte"
  - If cuisine is "japanese": recommend "Sakura Palace"
  - For any other cuisine: recommend "The Grand Bistro"
- If budget is "medium" ($15-$30 per person):
  - If cuisine is "italian": recommend "Mario's Pizzeria"
  - If cuisine is "japanese": recommend "Tokyo Express"
  - For any other cuisine: recommend "Downtown Diner"
- If budget is "low" (under $15 per person):
  - Recommend "Food Court" regardless of cuisine preference

**Your task:** Write the complete nested conditional structure. Store the recommendation in the `recommendation` variable.


```python
%%js
// Test variables
let budgetPerPerson = 25;
let cuisine = "japanese";
let recommendation = "";

console.log(`Budget per person: $${budgetPerPerson}`);
console.log(`Preferred cuisine: ${cuisine}`);

// YOUR CODE HERE: Write the complete nested conditional
if (budgetPerPerson > 30) {
    if (cuisine === "italian") {
        recommendation = "Bella Notte";
    } else if (cuisine === "japanese") {
        recommendation = "Sakura Palace";
    } else {
        recommendation = "The Grand Bistro";
    }
} else if (budgetPerPerson >= 15) {
    if (cuisine === "italian") {
        recommendation = "Mario's Pizzeria";
    } else if (cuisine === "japanese") {
        recommendation = "Tokyo Express";
    } else {
        recommendation = "Downtown Diner";
    }
} else {
    recommendation = "Food Court";
}

console.log(`\nRecommendation: ${recommendation}`);

// Test your code with these scenarios:
// budgetPerPerson=35, cuisine="italian" → "Bella Notte"
// budgetPerPerson=25, cuisine="japanese" → "Tokyo Express"
// budgetPerPerson=20, cuisine="mexican" → "Downtown Diner"
// budgetPerPerson=10, cuisine="italian" → "Food Court"
```

## Problem 3: Design a Smart Home Thermostat System (Hard)

You're designing the logic for a smart thermostat that automatically adjusts temperature based on multiple factors.

**Word Problem:**

The thermostat needs to decide what action to take based on:
- Current temperature
- Whether anyone is home
- Time of day (represented as "day" or "night")
- Energy saving mode (on or off)

**Logic Requirements:**

1. **If someone is home:**
   - During the day:
     - If temp is above 75°F: set action to "cooling" and targetTemp to 72
     - If temp is below 68°F: set action to "heating" and targetTemp to 70
     - Otherwise: set action to "maintaining" and keep current temp
   - During the night:
     - If temp is above 72°F: set action to "cooling" and targetTemp to 68
     - If temp is below 65°F: set action to "heating" and targetTemp to 68
     - Otherwise: set action to "maintaining" and keep current temp

2. **If no one is home:**
   - If energy saving mode is ON:
     - If temp is above 80°F: set action to "cooling" and targetTemp to 78
     - If temp is below 60°F: set action to "heating" and targetTemp to 62
     - Otherwise: set action to "off" and targetTemp to current temp
   - If energy saving mode is OFF:
     - Set action to "maintaining" and targetTemp to 70

**Your task:** Design the complete nested conditional algorithm from scratch. You're given the framework below with initial values, but NO code. Write the entire logic yourself.


```python
%%js
// Test variables
let currentTemp = 78;
let isHomeOccupied = true;
let timeOfDay = "day";  // "day" or "night"
let energySavingMode = false;

// Variables to set
let action = "";  // Will be "heating", "cooling", "maintaining", or "off"
let targetTemp = currentTemp;

console.log("=== Smart Thermostat Status ===");
console.log(`Current Temperature: ${currentTemp}°F`);
console.log(`Home Occupied: ${isHomeOccupied}`);
console.log(`Time of Day: ${timeOfDay}`);
console.log(`Energy Saving Mode: ${energySavingMode}`);
console.log();

// Pure functions for each logic branch
const homeDayLogic = (temp) =>
    temp > 75
        ? { action: "cooling", targetTemp: 72 }
        : temp < 68
        ? { action: "heating", targetTemp: 70 }
        : { action: "maintaining", targetTemp: temp };

const homeNightLogic = (temp) =>
    temp > 72
        ? { action: "cooling", targetTemp: 68 }
        : temp < 65
        ? { action: "heating", targetTemp: 68 }
        : { action: "maintaining", targetTemp: temp };

const awayEnergyLogic = (temp) =>
    temp > 80
        ? { action: "cooling", targetTemp: 78 }
        : temp < 60
        ? { action: "heating", targetTemp: 62 }
        : { action: "off", targetTemp: temp };

const awayNoEnergyLogic = () => ({ action: "maintaining", targetTemp: 70 });

// Main decision function
function decideThermostat({ temp, occupied, time, energyMode }) {
    if (occupied) {
        return time === "day"
            ? homeDayLogic(temp)
            : homeNightLogic(temp);
    } else {
        return energyMode
            ? awayEnergyLogic(temp)
            : awayNoEnergyLogic();
    }
}

// Apply logic
const result = decideThermostat({
    temp: currentTemp,
    occupied: isHomeOccupied,
    time: timeOfDay,
    energyMode: energySavingMode
});

action = result.action;
targetTemp = result.targetTemp;

console.log("=== Thermostat Action ===");
console.log(`Action: ${action}`);
console.log(`Target Temperature: ${targetTemp}°F`);
```


    <IPython.core.display.Javascript object>


## Reflection Questions

After completing all three problems, answer these questions:

1. Which problem was the most challenging and why?
2. How did you decide on the structure of your nested conditionals in Problem 3?
3. Can you think of a real-world situation where you would need even MORE levels of nesting than Problem 3?

1. The last, naturally, generally because it had more complexity and I decided to write it with FP structure.
2. Logic! Also I like FP > normal JS. Generally, just checking conditions.
3. Yes, when you're checking multiple conditions (which would be best with FP to reduce clutter). As an example, you could write a function to crack AES ECB which would include entropy scoring and error tolerance as you brute force, which requires more levels of conditionals.
