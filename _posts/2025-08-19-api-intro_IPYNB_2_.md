---
title: Intro to APIs
categories: ['APIs']
---

## APIs? What are those?
![by the nine](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fmedia.tenor.com%2FW8Te5fvAQZcAAAAM%2Ftweaking.gif&f=1&nofb=1&ipt=5baade81efe47ccdab7bdf2ea4581fa7b81173c33b8f786d7877e2b16f13d8ed)

You've probably heard the word API before, but never actually came to know what it meant. Well, we'll tell you now: APIs are **Application Programming Interfaces**.

Now that we know what the acronym means, we still don't know what an API actually is. Ergo, here's an analogy. 

Think about how power outlets work. If you want to use some electrical appliance (like a toaster, a microwave, a charger, etc.), you plug the power cord into the socket and it works. Now notice a clear distinction here: you don't plug your cord directly into the power supply because doing so would be highly inefficient and frankly dangerous. Power outlets are APIs without the AP part. Truth to be told, you have had plethoras of experience with interfaces before this class.


### Web Audio

And wouldn't you be surprised to know that you've actually using APIs your entire life. Your browser (Chrome, Firefox, Edge, etc.) has native APIs which expose data from the browser. The audio you hear from watching videos on your browser comes from the APIs like the Web Audio API provided by JavaScript which contains pieces of code that allow the browser to manipulate audio. In the background, the browser is actually using some lower-level code (like C++ or Rust) to handle actual audio processing. In fact, any application or website you use at School is probably calling some API; Canvas will call APIs to fetch your user data, Synergy will call the grade server APIs to fetch your grades etc. APIs provide end points to access data in an easy way.

![](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi1.wp.com%2Fwww.ux-republic.com%2Fwp-content%2Fuploads%2F2016%2F01%2FRichardFerris-diagram.jpg%3Ffit%3D1920%252C1080%26ssl%3D1&f=1&nofb=1&ipt=d4d74080dbda6b7128257790da7c06555d814fd3297a2eb4278c53bddba314e3)
*Web Audio API schematic*


### Graphics Libraries

Here's another lower level related example: you've probably heard of the term "graphics libraries" before. Basically, languages like C/C++ or Python have *libraries* of code where people have already written the lower level "stuff" (like CPU scheduling, ray tracing, rendering, etc.), and when you use code provided by those libraries, you're using their APIs. The lib programmer intentionally exposed pieces of your code so that you can call them and they'll handle some of the work for you.

![](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi.ytimg.com%2Fvi%2FizPR5TY6z9Q%2Fmaxresdefault.jpg&f=1&nofb=1&ipt=f95cde13d90bc1dcc3d415c5c6ad9148cdd30ea0a8e71ef7c01733f75024240e)
*OpenGL (Mesa) running graphics program `glxgears`*

## Ok... But Why?

APIs do a lot of things to make your life easier. Do you want to write your own custom driver for `DOM` (Document Object Model)? I know I don't. It's a lot of work, and you're really just reinventing the wheel.

There's a few common categories for browser APIs:

### APIs for editing webpages
The most common would be HTML's DOM, which allows you to manipulate HTML and CSS dynamically. Every time you see some page or content display, that DOM.

### APIs that fetch data
Store webpages will use APIs that fetch data from a database, say, for product prices. This is done mainly with JavaScript's `fetch` API and sometimes the `XMLHttpRequest` API.

### APIs for drawing/graphics
The most popular are **Canvas** and **WebGL**, which allow you to update pixel data contained in HTML `<canvas>` elements. If you're using those libraries to draw things or render 3D scenes on your site, you're most likely using those graphics APIs. Some of these APIs are combined with animation APIs (like `window.requestAnimationFrame()`) to constantly update scenes like for games.

### Audio and Video APIs
APIs like `HTMLMediaElement`, the `Web Audio API`, and `WebRTC` allow you do a *lot* with multimedia. You can create custom UIs, display text tracks, etc.

### Device APIs
The native `Geolocation` API allows browsers to interact with your device hardware for things like GPS.

### Client-side storage APIs
These are things like `Web Storage API` or `IndexedDB API` which allow you to store data on the client-side, so your website can save its state between page loads or even offline.

### And what if I need something else?
A lot of third parties provide external APIs (because they tend to make revenue some of the time -- like the OpenAI API) developed by other people. For example, Mapquest, Facebook, Telegram, Youtube, Pinterest, Twitter, Mastodon (a personal favorite; decentralized p2p social networking), etc.

## Now Let's Use an API

Let's start by defining *how* we call APIs using JavaScript. We'll try building a little code snipper that *fetches* some data from an API. The first command you'll need to know is `fetch()`.

Our structure will look something like this (trust me, we'll unpack this):
```js
fetch(ur)
    .then((response) => response.json())
    .then((json) => doSomething(json))
    .catch((error) => console.log("Error fetching data"));
```

This code runs `fetch(url)` on some URL to fetch data. The `.then()` statements come from Functional Programming nomenclature (you should learn [Haskell](https://www.haskell.org/)!), basically meaning "after doing this, do this." 

```js
    .then((response) => response.json())
```
Basically, if our `fetch()` does return a valid response, we get the response's JSON structure, which will look something like this:
```json
{
  "browsers": {
    "firefox": {
      "name": "Firefox",
      "pref_url": "about:config",
      "releases": {
        "1": {
          "release_date": "2004-11-09",
          "status": "retired",
          "engine": "Gecko",
          "engine_version": "1.7"
        }
      }
    }
  }
}
```

It may look a bit daunting, but the main idea is that it is a *data structure*, which, as the name suggests, is a structure that stores data.

```js
    .then((json) => doSomething(json))
```

Now, if we successfully get the response's JSON, we *then* run function `doSomething()` that takes the `json` parameter. 

You may be wondering: Wait! If we assigned the `response` variable to `response.json()`, how come we're using the different variable `json` to run `doSomething()` with? That, my friend, is the essence of Functional Programming.

Basically, we're running functions on the same piece of data to transform it. So, say our data is contained in some variable `x`, which is the result of `fetch(url)`. We then transform this data under an "alias" `response` to grab its JSON using `response.json()`. We have thus changed the data contained in `x`. Now, we run the next `.then()`, which, as we've seen, will return the the result from `doSomething()` and assign that returned data to that variable `x`. The data contained in `x` would look something like this:

```console
fetch(url) response -> response.json() -> doSomething()
```

Finally, the last piece of code:
```js
    .catch((error) => console.log("Error fetching data"));
```
The last piece of code basically says "if any of these fails, we `catch()` this error and return some error message." The reason we `catch()` the error is so that we containerize it. If we didn't catch the error, the variable `x` might just be an `Error` type, and so when we try to use that data under the assumption that it was correctly converted to JSON, we're just using the invalid `Error` data.


## No more tricks, I promise

Not let's write our own program to call an API!

For the sake of Jupyter notebooks, you'll need to have the code modify the HTML DOM, which I'll walk you through.

1. The first thing you'll want to do is make a new **markdown** cell.

2. In that **markdown** cell, we create write some HTML to create a container that we'll modify using the native DOM API (using APIs to write API code -- so cool, I know).

    ```html
    <div id="container"></div>
    <button onclick="fetchCat()">Click Me!</button>
    ```

    Here, we create a `<div></div>` with ID `"container"`, which is empty. We'll be modifying the content in that with our JavaScript code. Next, we create a simple button with an `onclick` field, which tells the code to run the function `fetchCat()` when the button is clicked.

3. Now for our actual script. We'll place this below the previous HTML code in a `<script></script>` environment, and we'll get to writing our actual logic.

    ```html
    <script>
        async function fetchCat() {
            const url = "https://cataas.com/cat?position=center";
            const options = { method: "GET" };

            let response = await fetch(url, options);

            if (response.status === 200) {
                const imageBlob = await response.blob();
                const imageObjectURL = URL.createObjectURL(imageBlob);

                const image = document.createElement('img');
                image.src = imageObjectURL;

                const container = document.getElementById('container');
                container.append(image);
            } else {
                console.log("HTTP Error: " + response.status);
            }
        }
    </script>
    ```

### Uhhhhh... What??

Ok, so what in the wold is going on here? In the first line, we define an `async` function (more on that later; just know that `async` is a type of function used for scheduling and time) `fetchCat()`.

We then define two `const` variables:
```js
const url = "https://cataas.com/cat?position=center";
const options = { method: "GET" };
```

- The first `const url` is assigned to a `string`, which is the API endpoint which we'll be calling. 

- The second `const options` is a JSON object with content defining a field `method:` with a string `"GET"`. We'll be using this later when sending our request.

- The idea here is to fetch some the data API endpoint (which would be some cat image).

Next, we construct our `response` variable:
```js
let response = await fetch(url, options);
```

We're using the native JavaScript `fetch()` function with parameters `url` and `options` to send a request to that API endpoint we defined previously in `const url`.

But what's with the `options` parameter? These are *headers* which we'll pass to the API telling it what we want to do. In this case, we want to perform an HTML `GET` request to retrieve data.

And what's `await`? `await` is a term we use in `async` functions that, in essence, says "wait for this task to finish before going forward." You'll be learning more about `async` later. 

If we try going forward in our code to try doing something with the data contained in `response`, there's no guarantee that we've received all the data. For example, we might have only received 20% of the data before we went forward and tried to do something with it, which would mean we were performing operations on mangled data.

Next, we have a control flow which looks a little large, but really isn't (it's mainly HTML stuff).

```js
if (response.status === 200) {
    const imageBlob = await response.blob();
    const imageObjectURL = URL.createObjectURL(imageBlob);

    const image = document.createElement('img');
    image.src = imageObjectURL;

    const container = document.getElementById('container');
    container.append(image);
} else {
    console.log("HTTP Error: " + response.status);
}
```

We first check if the `status` field of `response` is `200`. 

How does `response` even have that field though? The result of `fetch()` is a data structure called an `Object`. That `Object` will have specific data fields like `status`, which are filled in for us by `fetch()`.

And why `200`? `200` is the standard response code that HTTP returns when something goes correctly. So, as you may have realized, we're really just checking `if` the response we got was successful, we'll go forward, `else`, we simply run `console.log("HTTP Error: " + response.status)` to log the error in the console.

Let's take a look at the actual innards of this code now:

```js
    const imageBlob = await response.blob();
    const imageObjectURL = URL.createObjectURL(imageBlob);

    const image = document.createElement('img');
    image.src = imageObjectURL;

    const container = document.getElementById('container');
    container.append(image);
```

First, we begin by retrieving the `blob` of the `response` (with an `await` again). A blob is a data structure which tells you the file type and contains its data; we then assign this data to the `const imageBlob`. We then use `URL.createObjectURL(imageBlob)` to create a string containing the `blob`.

Next, we need to actually create an image using this data. To do this, we create an HTML `<img>` element and set its `src` to the blob URL string.

The next part is a more Jupyter/Jekyll sort of thing. To actually display this on the page, we refer back to the `container` div we created earlier. We then append our image to that container so it actually displays on the page.

Now let's try running it:

<div id="container"></div>
<button onclick="fetchCat()">Click Me!</button>

<script>
    async function fetchCat() {
        const url = "https://cataas.com/cat?position=center";
        const options = { method: "GET" };

        let response = await fetch(url, options);

        if (response.status === 200) {
            const imageBlob = await response.blob();
            const imageObjectURL = URL.createObjectURL(imageBlob);

            const image = document.createElement('img');
            image.src = imageObjectURL;

            const container = document.getElementById('container');
            container.append(image);
        } else {
            console.log("HTTP Error: " + response.status);
        }
    }
</script>
