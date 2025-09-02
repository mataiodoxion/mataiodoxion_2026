---
title: GitHub Pages Jokes
description: These Programming topics are focused on Jupyter Notebooks, and Managing Files.
permalink: /github/pages/jokes
author_profile: False
---

## Running Jupyter Notebooks

Let's have a laugh while we test out JavaScript in Jupyter Notebooks.

- In VSCode, go to **Help->Toggle Developer Tools**
- Click on **Console** in the newly opened Window
- Clear screen by pressing **Clear Console** (circle with line)
- Then press **Play button** to left of cells below
- Observe random output in the Console


### Programmer Jokes

An array of key-value pairs (JavaScript objects). Each joke has a "complexity" rating—if you understand the ones rated "3," your teacher will be impressed!


```python
%%html
<div>
    <h3>Random Computer Science Joke</h3>
    <p id="computer_joke"></p>
</div>

<script>
var compsci_joke_list = [
    { joke: "Why do programmers prefer dark mode? Because light attracts bugs.", complexity: "1" },
    { joke: "Why do Java developers wear glasses? Because they don't see sharp.", complexity: "2" },
    { joke: "How many programmers does it take to change a light bulb? None, that's a hardware problem.", complexity: "1" },
    { joke: "Why do Python programmers prefer snake_case? Because they can't C.", complexity: "2" },
    { joke: "Why was the JavaScript developer sad? Because he didn't know how to 'null' his feelings.", complexity: "3" },
    { joke: "Why do programmers always mix up Christmas and Halloween? Because Oct 31 == Dec 25.", complexity: "3" },
    { joke: "Why did the programmer quit his job? Because he didn't get arrays.", complexity: "O(n)" },
    { joke: "Why do Linux programmers prefer using the terminal? Because they don't like Windows.", complexity: "1" },
];
var randomIndex = Math.floor(Math.random() * compsci_joke_list.length);
var selectedJoke = compsci_joke_list[randomIndex];
var msg = "Joke #" + (randomIndex + 1) + ": " + selectedJoke.joke + " (Complexity: " + selectedJoke.complexity + ")";
console.log(msg);
document.getElementById("computer_joke").innerText = msg;
</script>
```


<div>
    <h3>Random Computer Science Joke</h3>
    <p id="computer_joke"></p>
</div>

<script>
var compsci_joke_list = [
    { joke: "Why do programmers prefer dark mode? Because light attracts bugs.", complexity: "1" },
    { joke: "Why do Java developers wear glasses? Because they don't see sharp.", complexity: "2" },
    { joke: "How many programmers does it take to change a light bulb? None, that's a hardware problem.", complexity: "1" },
    { joke: "Why do Python programmers prefer snake_case? Because they can't C.", complexity: "2" },
    { joke: "Why was the JavaScript developer sad? Because he didn't know how to 'null' his feelings.", complexity: "3" },
    { joke: "Why do programmers always mix up Christmas and Halloween? Because Oct 31 == Dec 25.", complexity: "3" },
    { joke: "Why did the programmer quit his job? Because he didn't get arrays.", complexity: "O(n)" },
    { joke: "Why do Linux programmers prefer using the terminal? Because they don't like Windows.", complexity: "1" },
];
var randomIndex = Math.floor(Math.random() * compsci_joke_list.length);
var selectedJoke = compsci_joke_list[randomIndex];
var msg = "Joke #" + (randomIndex + 1) + ": " + selectedJoke.joke + " (Complexity: " + selectedJoke.complexity + ")";
console.log(msg);
document.getElementById("computer_joke").innerText = msg;
</script>



### Accountant Jokes

A simpler array of strings—these jokes are just plain text, not JavaScript objects.


```python
%%html
<div>
    <h3>Random Accounting Joke</h3>
    <p id="accounting_joke"></p>
</div>

<script>
var accounting_joke_list = [
    "Why did the accountant cross the road? To bore the people on the other side.",
    "What do accountants do when they're constipated? They work it out with a pencil.",
    "How does an accountant stay out of debt? He learns to act his wage.",
    "Why did the accountant stare at his glass of orange juice for three hours? Because on the box it said 'concentrate'.",
    "Why did the accountant get promoted? Because he knew how to balance his work and play.",
    "Why did the accountant go broke? Because he lost his balance.",
    "Why did the accountant get a job at the bakery? Because he was good at making dough.",
    "Why did the accountant get a job at the zoo? Because he was good with cheetahs.",
    "Why did the accountant get a job at the library? Because he was good at keeping books.",
    "Why did the accountant get a job at the circus? Because he was good at juggling numbers.",
    "Why did the accountant get a job at the gym? Because he was good at working out the numbers.",
    "Why did the accountant get a job at the farm? Because he was good at counting the chickens before they hatched."
]
var randomIndex = Math.floor(Math.random() * accounting_joke_list.length);
var joke = "Joke #" + (randomIndex + 1) + ": " + accounting_joke_list[randomIndex];
console.log(joke);
document.getElementById("accounting_joke").innerText = joke;
</script>
```


<div>
    <h3>Random Accounting Joke</h3>
    <p id="accounting_joke"></p>
</div>

<script>
var accounting_joke_list = [
    "Why did the accountant cross the road? To bore the people on the other side.",
    "What do accountants do when they're constipated? They work it out with a pencil.",
    "How does an accountant stay out of debt? He learns to act his wage.",
    "Why did the accountant stare at his glass of orange juice for three hours? Because on the box it said 'concentrate'.",
    "Why did the accountant get promoted? Because he knew how to balance his work and play.",
    "Why did the accountant go broke? Because he lost his balance.",
    "Why did the accountant get a job at the bakery? Because he was good at making dough.",
    "Why did the accountant get a job at the zoo? Because he was good with cheetahs.",
    "Why did the accountant get a job at the library? Because he was good at keeping books.",
    "Why did the accountant get a job at the circus? Because he was good at juggling numbers.",
    "Why did the accountant get a job at the gym? Because he was good at working out the numbers.",
    "Why did the accountant get a job at the farm? Because he was good at counting the chickens before they hatched."
]
var randomIndex = Math.floor(Math.random() * accounting_joke_list.length);
var joke = "Joke #" + (randomIndex + 1) + ": " + accounting_joke_list[randomIndex];
console.log(joke);
document.getElementById("accounting_joke").innerText = joke;
</script>



## Check out Tools for Jupyter Notebooks

Here are some diagnostics if notebooks above are not working.


```python
%%script bash

# Define an array of commands
commands=("python --version" "jupyter --version" "jupyter kernelspec list")

for cmd in "${commands[@]}"; do
  echo "### Command: $cmd"
  bash -c "$cmd"
done
```

    ### Command: python --version
    Python 3.13.5
    ### Command: jupyter --version
    Selected Jupyter core packages...
    IPython          : 9.4.0
    ipykernel        : 6.30.0
    ipywidgets       : not installed
    jupyter_client   : 8.6.3
    jupyter_core     : 5.8.1
    jupyter_server   : 2.16.0
    jupyterlab       : 4.4.5
    nbclient         : 0.10.2
    nbconvert        : 7.16.6
    nbformat         : 5.10.4
    notebook         : 7.4.4
    qtconsole        : not installed
    traitlets        : 5.14.3
    ### Command: jupyter kernelspec list
    Available kernels:
      python3    /usr/share/jupyter/kernels/python3


## Hack 

If you love these jokes, you’ll probably want to have them in your own repository. Learning how to manage files in GitHub Pages is a key skill. This class will continually share files and offer challenges using GitHub Pages and Jupyter Notebooks.


- How to copy files between two GitHub repositories
  - **git clone** pages repository to your machine
  - **git pull** if you already have it, un git pull to make sure your repository is up to date
  - **code .** in directory of the project directory where you cloned the repo 
  - **drag and drop** files between repositories, be sure to put them in the right type of folder 

### Tip

- The **_notebooks** directory is the default location to place all **.ipynb** files.  
  - To get started, you can copy **_notebooks/Foundation/C-github_pages** to your project.
  - Then, rename or modify these files as part of the hack.

- The **_notebooks/Foundations/C-github_pages** contains the files that use an InfoGraph for a menu
  - In **frontmatter** of each file you will find data that describes its prooperties. 
    - **permalink: /github/pages/intro** 
    - [Describe some other properties]

- Create a new cell with your own (PG-13 rated) jokes
  - Pick your own topic
  - Jokes could be a great warmup at next live review
