# Cash Register (Common Lisp)

## Running the project

1. Build the docker container

```
docker build -t cash-register .
```

2. Enter the docker container shell

```
docker run -it cash-register sh
```

3. From the shell, run `sbcl`.

4. Run the followign commands in sbcl

```sbcl
(ql:quickload :cash-register)
(in-package :cash-register)
(calculate-change 15.94 16.00)
```

---

## Running the tests

1. Build the docker container

```
docker build -t cash-register .
```

2. Enter the docker container shell

```
docker run -it cash-register sh
```

3. From the shell, run `sbcl`.

4. Run the followign commands in sbcl

``` sbcl
(ql:quickload :fiveam)
(ql:quickload :cash-register/tests)
(in-package :cash-register-tests)
(run! 'all-tests)
```


---

## Useful Resources

- [Getting Started](https://lispcookbook.github.io/cl-cookbook/getting-started.html)
- [Beginner's guide to testing](https://turtleware.eu/posts/Tutorial-Working-with-FiveAM.html)
- [Creating an executable](https://lispcookbook.github.io/cl-cookbook/scripting.html)