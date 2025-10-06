---
title: 3.2 Data Abstractions JS Hack
description: Hack(s) for intro to data abstractions in JS.
permalink: /csp/big-idea-3/data-abstractions/p4/hacks-js
author_profile: False
<<<<<<< HEAD
hidden: True
=======
published: False
>>>>>>> 2b1cf4b (Add lesson files, fix archiving)
---

## JS Lab: Library

In this lab, similarly to the Python lab, you'll be working on a simple "database" for a library to understand CRUD operations in relation to representing redundant, similar data under one structure -- an abstraction.

For JavaScript, you'll have to open the web console from Developer Tools (`ctrl + shift + p` -> `Developer: Toggle developer tools`).


```python
%%js

let library = [
    { title: "1984", author: "George Orwell", checkedOut: false },
    { title: "To Kill a Mockingbird", author: "Harper Lee", checkedOut: true },
    { title: "The Great Gatsby", author: "F. Scott Fitzgerald", checkedOut: false }
];

function displayLibrary(lib) {
    console.log("All books in the library:");
    lib.forEach((book, i) => {
        console.log(`Index ${i}:`, book);
    });
}

function addBook(lib, { title, author, checkedOut = false }) {
    lib.push({ title, author, checkedOut });
}

function findBook(lib, searchTitle) {
    const book = lib.find(b => b.title.toLowerCase() === searchTitle.toLowerCase());
    if (book) {
        console.log("Book found:", book);
        return book;
    } else {
        console.log("Book not found.");
        return null;
    }
}

function updateBook(lib, searchTitle, newCheckedOut) {
    const idx = lib.findIndex(b => b.title.toLowerCase() === searchTitle.toLowerCase());
    if (idx !== -1) {
        lib[idx] = { ...lib[idx], checkedOut: newCheckedOut };
        console.log(`Updated "${searchTitle}":`, lib[idx]);
    } else {
        console.log("Book not found.");
    }
}

function deleteBook(lib, searchTitle) {
    const idx = lib.findIndex(b => b.title.toLowerCase() === searchTitle.toLowerCase());
    if (idx !== -1) {
        const removed = lib.splice(idx, 1)[0];
        console.log(`Deleted "${removed.title}" from library.`);
    } else {
        console.log("Book not found.");
    }
}

// Example usage
displayLibrary(library);
addBook(library, { title: "Brave New World", author: "Aldous Huxley", checkedOut: false });
findBook(library, "1984");
updateBook(library, "To Kill a Mockingbird", false);
deleteBook(library, "The Great Gatsby");
displayLibrary(library);
```


    <IPython.core.display.Javascript object>

