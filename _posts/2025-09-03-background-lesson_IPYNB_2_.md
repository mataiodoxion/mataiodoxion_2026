---
title: JS Background lesson
description: Learn javascript and simple animation building a background with flying object.
comments: False
layout: post
permalink: /background/lesson
---

## Background 

A background and flying object start the brain thinking...

- Can we add keys movements?
- How about making another objects?
- How does something like Flappy Bird work?


## Code Overview

### Frontmatter Images
These are yml key value pairs that are used to define the images for the center object and the moving background.

```yml
sprite: /images/platformer/sprites/flying-ufo.png
background: /images/platformer/backgrounds/alien_planet1.jpg
```

Obtains assets for project using VSCode terminal

```bash
mkdir -p hacks
wget https://raw.githubusercontent.com/Open-Coding-Society/pages/refs/heads/main/hacks/background.md -O hacks/background.md
```


```bash
mkdir -p images/platformer/sprites
wget https://raw.githubusercontent.com/Open-Coding-Society/pages/refs/heads/main/images/platformer/sprites/flying-ufo.png -O images/platformer/sprites/flying-ufo.png
```

```bash
mkdir -p images/platformer/backgrounds 
wget https://raw.githubusercontent.com/Open-Coding-Society/pages/refs/heads/main/images/platformer/backgrounds/alien_planet1.jpg -O images/platformer/backgrounds/alien_planet1.jpg
```

Open these files in VSCode

- hacks/background.md
  - frontmatter changes
    - change "opencs" to "base"
    - remove forward slashes on image names
- images/platformer/backgrounds/flying-ufo.png
- images/platformer/backgrounds/alien_planet.jpg

### Canvas 
<br>
In order for anything to show up on your screen in the first place, we have to create a canvas for everything to be drawn on.

We're going to need to **declare a constant variable** using the keyword ``const``

```js
const canvas = document.getElementById("world");
// document.getElementById("world") finds the <canvas id="world"> element in the DOM.

const ctx = canvas.getContext('2d');
// every canvas has a drawing context. for example, getContext('2d') returns the 2D drawing API (methods like drawImage, fillRect, clearRect).
// canvas is the DOM element; ctx is what draws pixels
```

#### Sizing the Canvas 

We need to setup the `GameWorld`
- The `canvas` is setup things being drawn are'nt too small or too big for your window screen.  Look at all the `this.canvas` commands.
- The `ctx` is used to draw into the canvas
- THe `objects` are assigned to `this.objects` these are the the game pieces placed into the game world.


```js
class GameWorld {
  static gameSpeed = 5;
  constructor(backgroundImg, spriteImg) {
    this.canvas = document.getElementById("world");
    this.ctx = this.canvas.getContext('2d');
    this.width = window.innerWidth;
    this.height = window.innerHeight;
    this.canvas.width = this.width;
    this.canvas.height = this.height;
    this.canvas.style.width = `${this.width}px`;
    this.canvas.style.height = `${this.height}px`;
    this.canvas.style.position = 'absolute';
    this.canvas.style.left = `0px`;
    this.canvas.style.top = `${(window.innerHeight - this.height) / 2}px`;

    this.objects = [
      new Background(backgroundImg, this),
      new Player(spriteImg, this)
    ];
```

Here are a couple of definitions

`canvas.width`, `canvas.height`: set the drawing buffer size (pixel resolution of the canvas).

`canvas.style.width`, `canvas.style.height`: set the **CSS** size (how big it appears on screen).
They set both equal so the drawing area matches the visible size.

### Game Objects
<br> In our game, everything that shows up on the screen (like the background or the character sprite) can be thought of as a **game objects**. Instead of writing separate code for each image, we create a **class** that acts like a blueprint.

```js
class GameObject {
  constructor(image, width, height, x = 0, y = 0, speedRatio = 0) {
    this.image = image;       // what picture to draw
    this.width = width;       // how wide to draw it
    this.height = height;     // how tall to draw it
    this.x = x;               // where it is horizontally
    this.y = y;               // where it is vertically
    this.speedRatio = speedRatio;
    this.speed = gameSpeed * this.speedRatio; // how fast it moves
  }
  update() {
    // gets filled in by subclasses (like Background)
  }
  draw(ctx) {
    ctx.drawImage(this.image, this.x, this.y, this.width, this.height);
  }
}
```

Instead of hardcoding the background or sprite separately, we can now make them instances of this `GameObject` class.  Looks for `Background` and `Player` class.

#### Background Object

Our background is special: it has to **move sideways** forever to look like we’re running. To make it seamless, we draw **two copies** of the background image side-by-side. As one scrolls off the screen, the other one takes its place.

``` js
class Background extends GameObject {
  update() {
    this.x = (this.x - this.speed) % this.width;
  }
  draw(ctx) {
    ctx.drawImage(this.image, this.x, this.y, this.width, this.height);
    ctx.drawImage(this.image, this.x + this.width, this.y, this.width, this.height);
  }
}
```

`update()` moves the background a little bit every frame.

The % this.width (modulo) makes sure it “wraps around” so it never disappears.

`draw()` paints two backgrounds so there’s no empty gap.

### The Game Loop (Animation)

The most important piece of a game is the **loop** that keeps running forever. This is where things get updated and redrawn, frame after frame.

```js
gameLoop() {
  this.ctx.clearRect(0, 0, this.width, this.height);
  for (const obj of this.objects) {
    obj.update();
    obj.draw(this.ctx);
  }
  requestAnimationFrame(this.gameLoop.bind(this));
}
start() {
  this.gameLoop();
}
```

`clearRect` wipes the screen so old frames so they don’t overlap.

`for` cycles through every object we defined with the GameWorld constructor.

`update` changes typically changes object positions.

`draw` puts the image onto the canvas according to recent updates.

`requestAnimationFrame` tells the browser: “do this again on the next frame.”

That’s what makes the background look alive — it’s being redrawn 60 times per second!



## Hacks

The Player object is distinct on the canvas.

### Make Player Object update

Goal. Make the player oscilate up and down during gaemLoop loop.

```javascript
class Player extends GameObject {
      constructor(image) {
        // Sets up object specifics and calls super
      }
      update() {
        // We need to change y value to go up a down
      }
    }
```

### Make comments in the project

Commenting is key to our understanding.  The code has been spaced into sections seperated by space.   Make comments in every section.  Open Chat to help, but Teacher will expect that you can explain and change.

Single line or end of line comment `//`
Multi line commets starting line `/*` ending line `*/`

### Make your own scene

There are backgrounds and sprites in images/platform.  Locate and make a change.
