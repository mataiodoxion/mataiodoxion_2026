---
title: 3.2 Data Abstractions
description: Intro to data abstractions in Python and JS.
permalink: /csp/big-idea-3/data-abstractions/p4/introduction
author_profile: False
---

## Learning Objectives

> 3.A Generalize data sources through variables.  
> 
> 3.B Use abstraction to manage complexity in a program.
> 
> 3.C Explain how abstraction manages complexity.

## Data Abstractions

Data abstraction means *treating collections of values as a single unit* (like a list, array, or string), so you don’t have to think about all the small details every time you use them. This manages complexity by letting us use a name (like `numbers`) instead of repeatedly referencing each individual element.

For example, in Python:

```py
# A list in Python
numbers = [10, 20, 30, 40]

# Access by index (0-based in Python)
print(numbers[0]) # 10
print(numbers[2]) # 30

# Strings work similarly
word = "hello"
print(word[1]) # 'e'
```

or in JavaScript:

```js
// An array in JavaScript
let numbers = [10, 20, 30, 40];

console.log(numbers[0]); // 10
console.log(numbers[2]); // 30

// Strings are also indexable
let word = "hello";
console.log(word[1]); // 'e'
```

## Using Abstractions to Manage Complexity

Instead of writing code for each item, we use loops and functions to work with lists.

For example, in Python, we could iterate over a list and find the average of its elements:

```py
scores = [85, 92, 78, 90]

# Sum with a loop
total = 0
for score in scores:
    total += score
print("Average:", total / len(scores)) # 86.25
```

and in JS:

```js
let scores = [85, 92, 78, 90];

// Using a loop
let total = 0;
for (let score of scores) {
    total += score;
}
console.log("Average:", total / scores.length);
```

## How Abstraction Manages Complexity

Instead of thinking about *every score individually*, you can use the abstraction "list of scores."
- With a name (`scores`), you manipulate the entire group.
- You don't need to worry about how the list is stored in memory.
- Functions and loops let you work with the *concept* of a collection, not every detail.

What we mean by the last part may be a bit more understandable in some examples of benefits.
- Easier maintenance (add/remove scores without rewriting logic over and over again).
- Reusable functions (average can work for any list of numbers).
- Clearer code (you read "average of scores" instead of `score1 + score2 + score3 + ...`).

## Nuances

Remember that Python and JS both use 0-based indexing, which means that the first element of a list/array/string is at index 0 (like we showed before).

## Quick Facts (From AP)

- A *list* is an ordered sequence of elements. For example, `[value1, value2, value3, ...]` describes a list where `value1` is the first element, `value2` is the second element, and `value3` is the third element, and so on.
- An *element* is an individual value in a list that is assigned a unique index.
- An *index* is a common method for referencing the elements ina  list or string using natural numbers.
- A *string* is an ordered sequence of characters.
- Data abstraction provides a separation between the abstract properties of a data type and the concrete details of its representation.
- Data abstractions manage complexity in programs by giving a collection of data a name without referencing the specific details of the  representation.
- Data abstractions can be created using lists.
- Developing a data abstraction to implement in a program can result in a program that is easier to develop and maintain.
- Data abstractions often contain different types of elements.
- The use of lists allows multiple related items to be treated as a single value. Lists are referred to by different names such as array, depending on the programming language.
- **For AP, lists are 1-indexed!**
