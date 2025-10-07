---
title: Nested Conditionals in Python - Homework
description: Apply your skills of multilayered conditionals and combine all you've learned so far.
permalink: /csp/big-idea-3/nested-conditionals/p4/hw-js
author_profile: False
hidden: True
---

# Nested Conditionals Homework: Python

Complete the following three problems to practice working with nested conditionals. Each problem increases in difficulty. **All problems must be completed in Python.**

## Problem 1: Complete the Shipping Cost Calculator (Easy)

An online store calculates shipping costs based on the order total and membership status. The partially completed code below needs to be finished.

**Rules:**
- If order total is $50 or more:
  - Free shipping for members
  - $5 shipping for non-members
- If order total is less than $50:
  - $3 shipping for members
  - $8 shipping for non-members

**Your task:** Complete the nested conditional so that when you run the code with the given initial values (`order_total = 45` and `is_member = True`), the output shows a shipping cost of **$3**.


```python
# Initial conditions - DO NOT CHANGE THESE
order_total = 45
is_member = True
shipping_cost = 0

print(f"Order Total: ${order_total}")
print(f"Member: {is_member}")

# YOUR CODE HERE: Complete the nested conditional
if order_total >= 50:
    if is_member:
        shipping_cost = 0
    else:
        shipping_cost = 5
else:
    if is_member:
        shipping_cost = 3
    else:
        shipping_cost = 8

print(f"Shipping Cost: ${shipping_cost}")

# Expected output with given values: Shipping Cost: $3
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
# Test variables
budget_per_person = 25
cuisine = "japanese"
recommendation = ""

print(f"Budget per person: ${budget_per_person}")
print(f"Preferred cuisine: {cuisine}")

# YOUR CODE HERE: Write the complete nested conditional
if budget_per_person > 30:
    if cuisine == "italian":
        recommendation = "Bella Notte"
    elif cuisine == "japanese":
        recommendation = "Sakura Palace"
    else:
        recommendation = "The Grand Bistro"
elif budget_per_person >= 15:
    if cuisine == "italian":
        recommendation = "Mario's Pizzeria"
    elif cuisine == "japanese":
        recommendation = "Tokyo Express"
    else:
        recommendation = "Downtown Diner"
else:
    recommendation = "Food Court"

print(f"\nRecommendation: {recommendation}")

# Test your code with these scenarios:
# budget_per_person=35, cuisine="italian" → "Bella Notte"
# budget_per_person=25, cuisine="japanese" → "Tokyo Express"
# budget_per_person=20, cuisine="mexican" → "Downtown Diner"
# budget_per_person=10, cuisine="italian" → "Food Court"
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
     - If temp is above 75°F: set action to "cooling" and target_temp to 72
     - If temp is below 68°F: set action to "heating" and target_temp to 70
     - Otherwise: set action to "maintaining" and keep current temp
   - During the night:
     - If temp is above 72°F: set action to "cooling" and target_temp to 68
     - If temp is below 65°F: set action to "heating" and target_temp to 68
     - Otherwise: set action to "maintaining" and keep current temp

2. **If no one is home:**
   - If energy saving mode is ON:
     - If temp is above 80°F: set action to "cooling" and target_temp to 78
     - If temp is below 60°F: set action to "heating" and target_temp to 62
     - Otherwise: set action to "off" and target_temp to current temp
   - If energy saving mode is OFF:
     - Set action to "maintaining" and target_temp to 70

**Your task:** Design the complete nested conditional algorithm from scratch. You're given the framework below with initial values, but NO code. Write the entire logic yourself.


```python
# Test variables
current_temp = 78
is_home_occupied = True
time_of_day = "day"  # "day" or "night"
energy_saving_mode = False

# Variables to set
action = ""  # Will be "heating", "cooling", "maintaining", or "off"
target_temp = current_temp

print("=== Smart Thermostat Status ===")
print(f"Current Temperature: {current_temp}°F")
print(f"Home Occupied: {is_home_occupied}")
print(f"Time of Day: {time_of_day}")
print(f"Energy Saving Mode: {energy_saving_mode}")
print()

# YOUR CODE HERE: Write the complete nested conditional algorithm

if is_home_occupied:
    if time_of_day == "day":
        if current_temp > 75:
            action = "cooling"
            target_temp = 72
        elif current_temp < 68:
            action = "heating"
            target_temp = 70
        else:
            action = "maintaining"
            target_temp = current_temp
    else:  # night
        if current_temp > 72:
            action = "cooling"
            target_temp = 68
        elif current_temp < 65:
            action = "heating"
            target_temp = 68
        else:
            action = "maintaining"
            target_temp = current_temp
else:
    if energy_saving_mode:
        if current_temp > 80:
            action = "cooling"
            target_temp = 78
        elif current_temp < 60:
            action = "heating"
            target_temp = 62
        else:
            action = "off"
            target_temp = current_temp
    else:
        action = "maintaining"
        target_temp = 70

print("=== Thermostat Action ===")
print(f"Action: {action}")
print(f"Target Temperature: {target_temp}°F")

# Test your code with these scenarios:
# Scenario 1: current_temp=78, is_home_occupied=True, time_of_day="day", energy_saving_mode=False
#   → action="cooling", target_temp=72
# Scenario 2: current_temp=66, is_home_occupied=True, time_of_day="night", energy_saving_mode=True
#   → action="maintaining", target_temp=66
# Scenario 3: current_temp=85, is_home_occupied=False, time_of_day="day", energy_saving_mode=True
#   → action="cooling", target_temp=78
# Scenario 4: current_temp=70, is_home_occupied=False, time_of_day="night", energy_saving_mode=False
#   → action="maintaining", target_temp=70
```

## Reflection Questions

After completing all three problems, answer these questions:

1. Which problem was the most challenging and why?
2. How did you decide on the structure of your nested conditionals in Problem 3?
3. Can you think of a real-world situation where you would need even MORE levels of nesting than Problem 3?

1. Ibid (js hw)
2. Ibid (js hw)
3. Ibid (js hw)
