---
title: Troubleshooting Guide
description: Recovery guide for common issues with GitHub, cloning, virtual environments, and running your website.
permalink: /tools/trouble
author_profile: False
---

## 🔧 Tool Setup Troubleshooting Guide

Use this page if something is not working.  
Each section is independent — jump directly to the area you are stuck.  

---

## GitHub Commit / Config Recovery

Use these commands if git commit is failing

✅ **Expectation**  
You have a GitHub username + email

```bash
git config --list` # shows your GitHub username + email.  
```

❌ **If not personalized, run to match your credentials:**  

```bash
git config --global user.name "jm1021"        # change to your GitHub ID
git config --global user.email "jm1021@gmail.com"  # change to your Email
```

---


```python
%%script bash

git config --list | grep -vE "user.email|Pa*"
```

    core.repositoryformatversion=0
    core.filemode=true
    core.bare=false
    core.logallrefupdates=true
    remote.origin.url=https://mataiodoxion@github.com/mataiodoxion/mataiodoxion_2026.git
    remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
    branch.master.remote=origin
    branch.master.merge=refs/heads/master
    branch.master.vscode-merge-base=origin/master
    user.name=mataiodoxion
    credential.helper=store


## Directory + Clone Recovery

✅ Expectation
You can cd into your personal directory, and an ls shows your repo folder (ex: student).

### Navigate

```bash
cd ~/jm1021 # change jm1021 to your user directory name
```

❌ If cd fails, run:

```bash
mkdir ~/jm1021
cd ~/jm1021
```

### Check for repo folder

```bash
ls # lshould show "student"
```

❌ If missing, run:

```bash
git clone https://github.com/jm1021/student.git # change to personal location of repo
```

---


```python
%%script bash

cd ../../
ls -a | grep "mataiodoxion_2026"
```

    mataiodoxion_2026


## Virtual Environment Recovery

✅ Expectation
Your terminal prompt shows (venv) prefix.

### Run Vitual Environment

```bash
source venv/bin/activate
```

❌ If it fails

```bash
./scripts/venv.sh
source venv/bin/activate
```

### VSCode Launch and Memories

✅ Satisfying the pre-requisites

- In project directory of your repo `pwd`
- Sourcing virtual environment `source venv/bin/activate`
- Ensure your terminal prompt shows the active virtual environment `(venv)`.

You are now ready to load VSCode and build a proper memory to open your project.

```bash
code .
```

✅ Verify VSCode launch

- Terminal and presence of `(venv)` prompt
- Open your Jokes IPYNB notebook and select the Python kernel with the `venv` prefix.

❌ If you fail verification

You may have opened your repo project without activating the proper `(venv)` environment.

Check the `Recent` listings. If there are entries that look incorrect or outdated (bad memories), remove them all.

- Shift-Cmd-P (Mac) or Shift-Ctl-P (Windows, KASM)
- type: Clear Recently Open -- select and confirm
- close VSCode
- Repeat VSCode Launch and Memories

---


```python
%%script bash

cd ../
source venv/bin/activate
echo "$PS1"
```

    (venv) 


## Version Checks

✅ Expectation

Run the bash script below
- Output is expected for each `### Command`
- Version may be slightly different, but ask if you are not sure
* Java kernels are required for CSA only

❌ If it fails

Best course of action is to run OS specific activate scripts from `pages` project directory

```bash
./scripts/activate_ubuntu.sh # windows ubuntu
./scripts/activate_macos.sh # macos
./scripts/activate.sh # help setup git config options
```



```python
%%script bash

# Define an array of commands
commands=("python --version" "pip --version" "ruby -v" "bundle -v" "gem -v" "jupyter --version" "jupyter kernelspec list" "git config --local user.name" "git config --local user.email")


for cmd in "${commands[@]}"; do
  echo "### Command: $cmd"
  bash -c "$cmd" 2>/dev/null | grep -vE ".com" || true
done
```

    ### Command: python --version
    Python 3.13.5
    ### Command: pip --version
    pip 25.2 from /usr/lib/python3.13/site-packages/pip (python 3.13)
    ### Command: ruby -v
    ruby 3.4.4 (2025-05-14 revision a38531fd3f) +PRISM [x86_64-linux]
    ### Command: bundle -v
    Bundler version 2.7.1
    ### Command: gem -v
    3.6.7
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
    ### Command: git config --local user.name
    mataiodoxion
    ### Command: git config --local user.email

