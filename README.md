
# sponk

(eventually) A very small array-oriented language.

Sponk mostly follows the same philosophy of J, except it has a slightly more sane syntax.

## lil ideas

Sponk operates on n-dimensional arrays.

```
    x = 1 2 3 4 5
    y = 6 7 8 9 10
    x + y
7 9 11 13 15

    x * y
6 14 24 36 50

    # $ x
1
```

You can manipulate arrays with several operators.

```
    -- list manipulation
    x = 1 2 3 4 5
    y = 6 7 8 9 10
    z = 11 12 13 14 15

    -- append
    x,y,z
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15

    -- push
    x,.y
1 2 3 4 5
6 7 8 9 10

    x,.y,.z
 1  2  3  4  5
 6  7  8  9 10
11 12 13 14 15

    -- shape, length, rank of arrays
    a = x,.y,.z
    # a -- a has three elements
3
    $ a -- a is a 3x5 array
3 5
    # $ a -- a is 2-dimensional
2
```

Lambdas/anonymous functions/dfns

```
    x = 32

    {] + ]} x
64

    double =. ] + ]
    def. double
.{]+]}

    double x
64

    avg =. +/] % #]
    def. avg
.{+/]%#]}

    avg 42 8 15 4 16 23
18

    l =: (avg[ < avg]) ~ ([,.])
    def. l
:{(avg[<avg])~([,.])}

    x = 1 2 3 4 5
    y = 6 7 8 9 10
    x l y
6 7 8 9 10
```

Documentation

```
    db =. ] + ]
    :db doc: 'Doubles the right argument.'

    help. db
Doubles the right argument.
db =. ] + ]
```

Quotes

```
    -- f expects its left argument to be a monadic function
    f =: [./ 2*]

    -- but this snippet doesn't really make sense
    {1+]} f 1 2 3 4 5
error: the expression is parsed as
    {1+]} (f 1 2 3 4 5)
but f has no monadic form
    f=:[./2*]

    -- so you need to quote the dfn in order to pass it to f
    .{1+]} f 1 2 3 4 5
3 5 7 9 11
```

Function combinators

## goals

* general sanity
* syntax
  * more of a blend between J and APL
* types
  * copy/paste from J
* functions
  * lambda
  * left and right arguments
* errors
  * report early and often, especially syntax

## syntax tree maybe

* `double =. ] + ]`
  add the right argument to the right argument
  `(+ right right)`
* `f =: [./ 2*]`
  multiply 2 times the right argument, then spread the monadic left argument through that result
  `(/ left. (* 2 right))`
* `avg =. (+/]) % #]`
  spread + through the right argument, then find the length of the right argument, then divide
  `(% (/ + right) (# right))`
* `l =: ([ avg&< ]) ~ ([,.])`
  average the right argument, then average the left argument, compare their results
  push the right argument to the left argument, pick
  `(~ (,. (left right)) (< (avg right) (avg left)))`
