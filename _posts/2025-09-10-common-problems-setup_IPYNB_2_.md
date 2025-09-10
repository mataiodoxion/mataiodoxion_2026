---
title: A Brief Comment of LxD Init
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
