---
title: Conditionals in JavaScript - Hacks
description: Three scaffolded hacks to practice selection (if, else if, else) in JavaScript. No nesting.
permalink: /csp/big-idea-3/conditionals/p4/hw-js
author_profile: False
hidden: True
---

# 🌐 CSP 3.6 Hacks — Conditionals (JavaScript)

You will complete **three** small programs that use selection with `if`, `else if`, `else` 

Answers to common errors:
- Please select the JavaScript kernel when running the code, do NOT select python
- If the code reports random errors try pressing restart at the top bar
- If you get stuck on a problem, make comments about what you understand so far and what you are stuck on



## 🟢 Hack 1 — Number Range Checker (Beginner)

**Requirements (spec):**
- Ask for a number.
- If the number is **between 0 and 10** (inclusive), print the number **then** print `Goodbye`.
- Otherwise, print only `Goodbye!`.
- Use one `if / else` and combine comparisons with `&&`.

**Tips:** use `parseInt(..., 10)`; inclusive means `>=` and `<=`.



```javascript
// Starter code 
var num = 11;  //  Change this number to test different inputs

//Complete the code below according to the rules above
// Do it with ternaries!
(num >= 0 && num <= 10) ? (console.log(`${num}\nGoodbye`)) : console.log("Goodbye!");

```


    <IPython.core.display.Javascript object>



## 🟡 Hack 2 — Grade Evaluator (Intermediate)

**Requirements (spec):**
- Ask for a grade `0–100`.
- If `grade >= 75`, print two lines: `You get extra credit!` then `Have a good day`.
- Otherwise, print `Have a good day` only.





```javascript

// Starter code 
let grade = 76;  // Change this grade to test different cases

// TODO:
// If grade >= 90 print "A" then "Have a good day"
// If grade >= 87 but < 90 print "Can be rounded to an "A", then "Have a good day"
// Else → print "Have a good day"

// Mapping in FP with JS
[
    grade >= 90 ? () => { console.log("A"); console.log("Have a good day"); } :
    grade >= 87 ? () => { console.log('Can be rounded to an "A"'); console.log("Have a good day"); } :
    () => { console.log("Have a good day"); }
][0]();

//Complete the code below according to the rules above
```


    <IPython.core.display.Javascript object>



## 🔵 Hack 3 — Access Pass (Advanced)

**Inputs:** `age` (number), `has_ticket` (`"yes"/"no"`), `vip` (`"yes"/"no"`)

**Rules**
- If `vip === "yes"` → `VIP Entrance`
- Else if `has_ticket === "yes"` **and** `age >= 16` → `General Entrance`
- Else if `has_ticket === "yes"` **and** `age < 16` → `Minor Entrance (with guardian)`
- Else → `No Entrance`

**Tip:** normalize strings with `.trim().toLowerCase()`.



```javascript
// Starter code (student edits this cell)
let age = 16;           //  Change the age
let has_ticket = "yes"; //  Change to "yes" or "no"
let vip = "no";         //  Change to "yes" or "no"

// TODO: Implement ladder using if / else if / else
// Rules:
// - VIP = "yes" → "VIP Entrance"
// - Ticket = "yes" and age >= 16 → "General Entrance"
// - Ticket = "yes" and age < 16 → "Minor Entrance (with guardian)"
// - Otherwise → "No Entrance"
[
    vip === "yes" ? () => console.log("VIP Entrance") :
    has_ticket === "yes" && age >= 16 ? () => console.log("General Entrance") :
    has_ticket === "yes" && age < 16 ? () => console.log("Minor Entrance (with guardian)") :
    () => console.log("No Entrance")
][0]();

// Complete the code below according to the rules above
```


    <IPython.core.display.Javascript object>



---
### Reflection: Think about the following question. Answer in 3-4 sentences
- Did any of the question trip you up? If yes, explain the problem and your solution. If no, give a brief, 1-2 sentence summary of conditionals.
- Can you think of real life examples where we would use conditionals? Please do NOT use one of the hacks as a real life example
- How is javascript different from python in terms of conditionals?

1. No; basically, we're just checking for conditions (as the name suggests) and this could be in several forms (checking booleans, eqs, partial eqs, etc.).
2. Yes; basically any program you write will include conditionals. Heck, computers execute code using logic gates which are conditionals at the purest level. For example, you could check whether a certain driver is loaded, then load it if not.
3. Javascript and Python are more or less the same in context and nuance, but have slightly different syntax which is negligible.


---
### 🧾 Turn-in checklist
- [ ] All three hacks run and match the rules above them. 
- [ ] You included at least one changed element for at least one hack, this can be change of message, the range, or the input information (not the input, the prompt for the input). Do not just change the input and call that your personal change.
- [ ] You wrote a 2–3 sentence reflection in your portfolio:  
  - What conditional form did you use most?  
  - What would you like to add to the lesson that would help you better understand the material?


Nothing, good lesson.
