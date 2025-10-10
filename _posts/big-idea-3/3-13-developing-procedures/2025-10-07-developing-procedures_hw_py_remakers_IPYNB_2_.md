---
title: Remakers - 3.13 Developing Procedures Python Hacks
description: Apply your skills to basic procedure development in Python.
permalink: /csp/big-idea-3/developing-procedures/p4/hw-py
hidden: True
---

### Q1 (Easy)
Which of these procedures is named well, provide a **short** explanation of justification


```python
def mix_ingredients():

def do_it():

def make_move():
```

**Explanation Here:**
`do_it()` is wrong because its naming is vague

### Q2 (Medium)
Finish the code to have a correctly named procedure


```python
def make_left_turn():
    move_forward()
    rotate_left()
    move_forward_again()

##Todo: add corresponding procedure definitions
##here- replace this line with your code
def move_forward():
    print("Moving forward.")

##here- replace this line with your code
def rotate_left():
    print("Turning left")

def move_forward_again():
    print("Moving forward again to complete left turn.")

# Run the procedure
if __name__ == '__main__':
    make_left_turn()
```

    Moving forward.
    Turning left
    Moving forward again to complete left turn.


### Q3 (Hard)
Write code to fulfill the requirements
Doing a dance! 🕺💃
Must have
1. A shimmy left procedure
- Print `super cool left slide`
2. A shimmy right procedure, print `even cooler right slide`
3. Doing a bow to the crowd, print `Great dance!`, `the audience claps at your bow!`


```python
def sillyDance():
    leftSlide()
    rightSlide()
    bow()

def leftSlide():
    print("Super cool left slide")

def rightSlide():
    print("Even cooler right slide")

def bow():
    print("Great dance!\nThe audience claps and at your bow!")

sillyDance()
```

    Super cool left slide
    Even cooler right slide
    Great dance!
    The audience claps and at your bow!


![silly dance](https://i.redd.it/7bb0v1q2j9ce1.gif)
