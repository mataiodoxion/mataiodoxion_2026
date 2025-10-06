---
title: Conditionals in Javascript and Python - Hacks
description: Apply your skills to basic algorithmic design with conditionals.
permalink: /csp/big-idea-3/conditionals/p4/hw-py
author_profile: False
published: False
---

# 🚀 CSP 3.6 Hacks — Conditionals 

You will complete **three** small programs that practice *selection* using `if`, `if/else`, and `if/elif/else`.  
Please do not use nested conditionals as that is for 3.7

## How to comeplete
1. **Read the directions** for the hack you’re on.
2. **Run the starter cell**, add your code where marked.
3. **Test your program** with at least the sample inputs provided.
4. **Make one or more small changes to the starter code** (This can be the message, number, range, etc). Please do not just turn in the starter code
5. When finished, **screenshot or record** a short demo and add it to your portfolio.

---
## What we’re assessing
- Correct use of `if`, `elif`, `else` .
- Correct boolean expressions with comparisons and `and`/`or`/`not`.
- Output matches the rules **for the given inputs**.
- Clean prompts and give some comments that show your understanding.


## 🟢 Hack 1 — Number Range Checker 

**Goal**: Ask for a number and **select** the correct output.

**Rules**
- If input is **between 0 and 10** (inclusive), print the number **then** print `"Goodbye"`.
- Else, **immediately** print `"Goodbye!"` only.

**Steps**
- [ ] Prompt: `"Enter a number: "` and convert to `int`.
- [ ] Write one condition using `>=` and `<=` with `and`.
- [ ] Print in the correct order for the true case.
- [ ] Print only `"Goodbye!"` for the false case.

**Sample tests**
- Input `5` → prints `5` then `Goodbye` on next line.
- Input `-2` → prints `Goodbye!`
- Input `10` → prints `10` then `Goodbye`





```python
num = int(input("Enter a number: "))

if num >= 0 and num <= 10:
    print(f"{num}\nGoodbye")
else:
    print("Yikes!")
```

    5
    Goodbye



## 🟡 Hack 2 — Grade Evaluator 

**Goal**: Print messages based on a passing threshold using **selection**.

**Rules**
- Ask for a grade `0–100` (int).
- if the grade is 90 or above, give them an A and tell them to have a good day
- If the grade is 87-90, they have a chance to get an A through the AP test, so tell them "Can be rounded to an A" and tell them to have a good day
- If their grade is lower, just tell them to have a good day

**Steps**
- [ ] Prompt: `"Enter your grade (0-100): "` and convert to `int`.
- [ ] Write the `if/else` using `>=`.
- [ ] Make sure the passing branch prints **both** lines, in order.

**Sample tests to try**
- `100` → two lines
- `88` → two lines
- `74` → one line

**Optional**
- Add an `elif grade < 0 or grade > 100` to warn about invalid input, else keep the same behavior.



```python
grade = int(input("Enter your grade (0-100): "))

if grade < 0 or grade > 100:
    print("Invalid grade entered.")
elif grade >= 90:
    print("A")
    print("Have a good day")
elif grade >= 87:
    print("Can be rounded to an A")
    print("Have a good day")
else:
    print("Have a good day. Yikess")
```

    Have a good day. Yikess



## 🔵 Hack 3 — Access Pass (Hard)

You’re programming the entrance logic for a concert venue.

**Inputs**
- `age` (int)
- `has_ticket` (string `"yes"`/`"no"` — case-insensitive)
- `vip` (string `"yes"`/`"no"` — case-insensitive)

**Rules**
- If `vip == "yes"` → print `"VIP Entrance"`
- Else if `has_ticket == "yes"` **and** `age >= 16` → `"General Entrance"`
- Else if `has_ticket == "yes"` **and** `age < 16` → `"Minor Entrance (with guardian)"`
- Else → `"No Entrance"`

**Steps**
- [ ] Read inputs and normalize strings with `.strip().lower()`.
- [ ] Use **one** `if/elif/elif/else` ladder (don't use nesting).
- [ ] Combine conditions with `and` where required.
- [ ] Print exactly one of the four messages.

**Sample tests**
- `age=12, ticket=no, vip=yes` → `VIP Entrance`
- `age=20, ticket=yes, vip=no` → `General Entrance`
- `age=15, ticket=yes, vip=no` → `Minor Entrance (with guardian)`
- `age=20, ticket=no, vip=no` → `No Entrance`

**Optional**
- Add an **OR**: Guests under 8 with a ticket may enter as `"Kid Entrance (with adult)"`.  
  



```python
age = int(input("Age: "))
has_ticket = input("Has ticket? (yes/no): ").strip().lower()
vip = input("VIP? (yes/no): ").strip().lower()

if vip == "yes":
    print("VIP Entrance")
elif has_ticket == "yes" and age < 8:
    print("Kid Entrance (with adult)")
elif has_ticket == "yes" and age >= 65:
    print("Senior Entrance (priority line)")
elif has_ticket == "yes" and age >= 16:
    print("General Entrance")
elif has_ticket == "yes" and age < 16:
    print("Minor Entrance (with guardian)")
else:
    print("No Entrance")
```

## Hack 1 — Number Range Checker

**Just try your best, we know this can be challenging but please do not use AI to do these problems.**



```python
num = int(input("Enter a number: "))
# TODO: Write an if/else that checks if num is between 0 and 10 (inclusive)

if num >= 0 and num <= 10:
    print(num)
    print("Goodbye")
else:
    print("Goodbye!")
```

    Goodbye!


## Hack 2 — Grade Evaluator

**Just try your best, we know this can be challenging but please do not use AI to do these problems.**



```python
# TODO: Write an if/else that checks if grade is 90 or above, 87-89, or lower

if grade >= 90:
    print("A")
    print("have a good day")
elif grade >= 87:
    print("can be rounded to an A")
    print("have a good day")
else:
    print("have a good day")
```

## Hack 3 — Access Pass (Advanced) 
**Just try your best, we know this can be challenging but please do not use AI to do these problems.**

You can always go back to the lesson to relearn the topics. If you are really stuck, just add comments explaining your thought process and where you are stuck on, we will grade it based on your understanding, not just completion.


```python
age = int(input("Age: "))
has_ticket = input("Has ticket? (yes/no): ").strip().lower()
vip = input("VIP? (yes/no): ").strip().lower()

# 1. If vip is "yes", print "VIP Entrance"
if vip == "yes":
    print("VIP Entrance")
# 2. Else if the person has a ticket ("yes") and is 16 or older, print "General Entrance"
elif has_ticket == "yes" and age >= 16:
    print("General Entrance")
# 3. Else if the person has a ticket ("yes") and is younger than 16, print "Minor Entrance (with guardian)"
elif has_ticket == "yes" and age < 16:
    print("Minor Entrance (with guardian)")
# 4. Otherwise, print "No Entrance"
else:
    print("No Entrance")
```


---
### Reflection: Think about the following question. Answer in 3-4 sentences
- Did any of the question trip you up? If yes, explain the problem and your solution. If no, give a brief, 1-2 sentence summary of conditionals.
- Can you think of real life examples where we would use conditionals? Please do NOT use one of the hacks as a real life example
- Which operator (`and`, `or`, `not`) do you think you will use the most, and why?

Ibid. See other hw file for answers


---
### 🧾 Turn-in checklist
- [ ] All three hacks run and match the rules above them. 
- [ ] You included at least one changed element for at least one hack, this can be change of message, the range, or the input information...
- [ ] You wrote a 2–3 sentence reflection in your portfolio:  
  - What conditional form did you use most?  
  - Where did you use `and`/`or`/`not`?  
  - What would you like to add to the lesson that would help you better understand the material?


Ibid.
