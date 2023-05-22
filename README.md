## minigrep

Rust version of the classic command line search tool `grep` (**g**lobally search a **r**egular **e**xpression and **p**rint).

In the simplest use case, `grep` searches a specified file for a specified string. 

To do so, `grep` takes as its arguments a file path and a string. Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.
