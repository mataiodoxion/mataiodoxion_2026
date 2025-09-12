---
title: A Brief Comment on LxD Init
description: A brief overview of certain
author_profile: False
---

## Common Issues

After debugging a bit for other people and tinkering on my own Arch installation, I've ran into a bunch of problems.

The majority of these problems can simply be resolved by running in `venv`:

```console
➜  pages git:(main) make
Stopping server...
Stopping logging process...
Traceback (most recent call last):
  File "<string>", line 1, in <module>
    from scripts.convert_notebooks import convert_notebooks; convert_notebooks()
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/parallaxis/Programming/School/opencs/pages/scripts/convert_notebooks.py", line 15, in <module>
    from scripts.progress_bar import ProgressBar
  File "/home/parallaxis/Programming/School/opencs/pages/scripts/progress_bar.py", line 1, in <module>
    from progress.bar import ChargingBar
ModuleNotFoundError: No module named 'progress'
make: *** [Makefile:146: _posts/Foundation/C-github_pages/2025-08-21-github_pages_jokes_IPYNB_2_.md] Error 1

➜  pages git:(main) source venv/bin/activate
(venv) ➜  pages git:(main) make
Stopping server...
Stopping logging process...
Notebook conversion progress: 1/261 ∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙∙ 0.4% 
...
```

Sometimes, this also doesn't work, and errors due to a lack of dependencies. The most common missing dependency I've seen is the Ruby gem `erb`.

This can simply be installed by adding this to the `Gemfile`:
```
gem "erb"
```
and upon running `bundle install` and `make`, everything should be resolved.


## Running Arch
Surprisingly, there were very few problems I ran into while using Arch for AP CSP. There were a few problems with installing a Ruby toolchain, Bundler, and the like, but other than that, very few problems. The main point of speculation is how I'll run Ditto... 

## Running a Custom Site

I didn't want to use the given templates because they were really bloated and the load times were abysmal. I figured I could just build my own from a different Jekyll theme and only pull what I needed (which would require a bit tinkering because they aren't so compatible). I ended up using [minimal-mistakes](https://mmistakes.github.io/minimal-mistakes/) with a dark theme. The notebooks tended to be incompatible with their front matters, but I fixed those by removing a few fields. 

Needless to say, I had to do a bunch of tinkering to get my site (mainly with having the blog pages display wider and without my author profile which was not necessary). But, everything's been working nicely and it's quite literally 3x as fast. I also implemented my own Makefile with a method to refresh notebooks which I made an issue on [here](https://github.com/mataiodoxion/lxd-apis/issues/2).

## Adding Rust support to Jekyll

I'll be doing this via WASM.

### Set Up
First course of action is to start a Rust project in the root using `cargo`:
```rs
cargo new wasm-module
```

```console
wasm-module/
├── Cargo.toml
└── src/
    └── lib.rs
```

Then add our `lib.rs`:
```rs
// filepath: wasm-module/src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

and our `Cargo.toml`:
```rs
// filepath: wasm-module/Cargo.toml
[package]
name = "wasm-module"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"

[dependencies.web-sys]
version = "0.3"
features = [
  "console",
  "Document",
  "Element",
  "HtmlElement",
  "Window",
]
```

### Build and Deploy WASM

Install `wasm-pack` using `cargo install wasm-pack`.

Then run (in the `wasm-module` directory)
```
wasm-pack build --target web --out-dir ../assets/wasm
```

### Integrating into Jekyll Pages

```md
---
layout: default
title: WASM Rust Demo
description: Running Rust WASM code in Jekyll
permalink: /wasm-demo
---

<div id="wasm-output"></div>
<button id="run-rust">Run Rust Code</button>

<script type="module">
    import init, { greet, add } from '/mataiodoxion_2026/assets/wasm/wasm_module.js';
    
    async function run() {
        // Initialize the WASM module
        await init();
        
        document.getElementById('run-rust').addEventListener('click', () => {
            const result = add(5, 3);
            document.getElementById('wasm-output').innerText = `Rust says: 5 + 3 = ${result}`;
            greet('WASM');
        });
    }
    
    run();
</script>
```

### Integrating Jupyter Notebooks

```html
%%html
<div id="rust-demo"></div>
<script type="module">
    import init, { fibonacci } from '/mataiodoxion_2026/assets/wasm/math_module.js';
    
    async function runDemo() {
        await init();
        const result = fibonacci(10);
        document.getElementById('rust-demo').innerHTML = `Fibonacci(10) = ${result}`;
    }
    
    runDemo();
</script>
```

### Update the Makefile
```Makefile
# ...existing content...

# Build WASM modules
wasm:
    @echo "Building WASM modules..."
    @if [ -d "wasm-module" ]; then \
        cd wasm-module && wasm-pack build --target web --out-dir ../assets/wasm; \
    fi

# Modified server target to include WASM
server: stop convert wasm
    @echo "Starting server incrementally..."
    @nohup bundle exec jekyll serve -H 127.0.0.1 -P $(PORT) --livereload > $(LOG_FILE) 2>&1 & \
        PID=$$!; \
        echo "Server PID: $$PID"
    @until [ -f $(LOG_FILE) ]; do sleep 1; done
```

### Add WASM Target

```console
rustup target add wasm32-unknown-unknown
```

### Update Github Workflow

```yml
# This workflow uses actions that are not certified by GitHub.
# They are provided by a third-party and are governed by
# separate terms of service, privacy policy, and support
# documentation.

# Sample workflow for building and deploying a Jekyll site to GitHub Pages
name: Deploy Jekyll site to Pages

on:
  # Runs on pushes targeting the default branch
  push:
    branches: ["master"]

  # Allows you to run this workflow manually from the Actions tab
  workflow_dispatch:

# Sets permissions of the GITHUB_TOKEN to allow deployment to GitHub Pages
permissions:
  contents: read
  pages: write
  id-token: write

# Allow only one concurrent deployment, skipping runs queued between the run in-progress and latest queued.
# However, do NOT cancel in-progress runs as we want to allow these production deployments to complete.
concurrency:
  group: "pages"
  cancel-in-progress: false

jobs:
  # Build job
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Setup Ruby
        # https://github.com/ruby/setup-ruby/releases/tag/v1.207.0
        uses: ruby/setup-ruby@4a9ddd6f338a97768b8006bf671dfbad383215f4
        with:
          ruby-version: '3.1' # Not needed with a .ruby-version file
          bundler-cache: true # runs 'bundle install' and caches installed gems automatically
          cache-version: 0 # Increment this number if you need to re-download cached gems
      
      # Add Rust toolchain setup
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
          override: true
      
      # Install wasm-pack
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      
      # Build WASM module
      - name: Build WASM module
        run: |
          if [ -d "wasm-module" ]; then
            cd wasm-module
            wasm-pack build --target web --out-dir ../assets/wasm
            cd ..
          else
            echo "No wasm-module directory found, skipping WASM build"
          fi
      
      - name: Install Python dependencies
        run: |
          python -m venv venv
          source venv/bin/activate
          pip install -r requirements.txt

      - name: Execute conversion script
        run: |
          source venv/bin/activate
          python scripts/convert_notebooks.py
      - name: Setup Pages
        id: pages
        uses: actions/configure-pages@v5
      - name: Build with Jekyll
        # Outputs to the './_site' directory by default
        run: bundle exec jekyll build --baseurl "${{ steps.pages.outputs.base_path }}"
        env:
          JEKYLL_ENV: production
      - name: Upload artifact
        # Automatically uploads an artifact from the './_site' directory by default
        uses: actions/upload-pages-artifact@v3

  # Deployment job
  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    needs: build
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

### Result?

Check out the (result)[https://mataiodoxion.github.io/mataiodoxion_2026/wasm-demo].
