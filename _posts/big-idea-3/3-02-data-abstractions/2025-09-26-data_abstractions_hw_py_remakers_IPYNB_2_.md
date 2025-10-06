---
title: 3.2 Data Abstractions Python Hack
description: Hack(s) for intro to data abstractions in Python.
permalink: /csp/big-idea-3/data-abstractions/p4/hacks-py
author_profile: False
<<<<<<< HEAD
hidden: True
=======
published: False
>>>>>>> 2b1cf4b (Add lesson files, fix archiving)
---

## Python Lab: Simple DBs

In this lab, you'll be working on a simple "database" system consisting of dictionaries. The idea here is to understand some basic CRUD actions and how you can use data abstractions (dictionaries in this case) to represent redundant, similar data under a unified structure.

You'll have to do some research about some Python syntax for this!

You can complete the Python lab by simply running your code and getting your outputs in the Jupyter notebook.


```python
# Our "database" is a list of dictionaries, each representing a record (e.g., a student)
db = [
    {"name": "Alice", "age": 16, "grade": "A"},
    {"name": "Bob", "age": 17, "grade": "B"},
    {"name": "Charlie", "age": 16, "grade": "C"}
]

def display_db(database):
    print("All records in the list:")
    for i, record in enumerate(database):
        print(f"Index {i}: {record}")

def add_record(database, name, age, grade):
    new_record = {"name": name, "age": age, "grade": grade}
    database.append(new_record)
    print(f"Added: {new_record}")

def find_record(database, search_name):
    for record in database:
        if record["name"] == search_name:
            print(f"Found: {record}")
            return record
    print(f"No record found for name: {search_name}")
    return None

def update_record(database, search_name, new_age=None, new_grade=None):
    for record in database:
        if record["name"] == search_name:
            if new_age is not None:
                record["age"] = new_age
            if new_grade is not None:
                record["grade"] = new_grade
            print(f"Updated: {record}")
            return
    print(f"No record found for name: {search_name}")

def delete_record(database, search_name):
    for i, record in enumerate(database):
        if record["name"] == search_name:
            del database[i]
            print(f"Deleted record for name: {search_name}")
            return
    print(f"No record found for name: {search_name}")

# Example usage/tests
display_db(db)
add_record(db, "David", 18, "B")
find_record(db, "Alice")
update_record(db, "Bob", new_age=18, new_grade="A")
delete_record(db, "Charlie")
display_db(db)
```

    All records in the list:
    Index 0: {'name': 'Alice', 'age': 16, 'grade': 'A'}
    Index 1: {'name': 'Bob', 'age': 17, 'grade': 'B'}
    Index 2: {'name': 'Charlie', 'age': 16, 'grade': 'C'}
    Added: {'name': 'David', 'age': 18, 'grade': 'B'}
    Found: {'name': 'Alice', 'age': 16, 'grade': 'A'}
    Updated: {'name': 'Bob', 'age': 18, 'grade': 'A'}
    Deleted record for name: Charlie
    All records in the list:
    Index 0: {'name': 'Alice', 'age': 16, 'grade': 'A'}
    Index 1: {'name': 'Bob', 'age': 18, 'grade': 'A'}
    Index 2: {'name': 'David', 'age': 18, 'grade': 'B'}

