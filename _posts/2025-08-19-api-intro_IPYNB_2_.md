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

            // Replace 'container' with your actual container id
            const container = document.getElementById('container');
            container.append(image);
        } else {
            console.log("HTTP Error: " + response.status);
        }
    }
</script>
