
# sponk

(eventually) A very small array-oriented language.

Sponk mostly follows the same philosophy of J, except it has a slightly more sane syntax.

## lil ideas

Sponk operates on n-dimensional arrays.

```
    x =. 1 2 3 4 5
    y =. 6 7 8 9 10
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
    x =. 1 2 3 4 5
    y =. 6 7 8 9 10
    z =. 11 12 13 14 15

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
    a =. x,.y,.z
    # a -- a has three elements
3
    $ a -- a is a 3x5 array
3 5
    # $ a -- a is 2-dimensional
2
```

### Functions and dfn's (verbs and dvns?)

```
    x =. 32

    {] + ]} x
64

    double =: ] + ]
    :double
:{]+]}

    double x
64

    avg =: +/] % #]
    :avg
:{+/]%#]}

    avg 42 8 15 4 16 23
18

    l =: ((avg[) < (avg])) ~ ([,.])
    :l
:{(avg[<avg])~([,.])}

    x =. 1 2 3 4 5
    y =. 6 7 8 9 10
    x l y
6 7 8 9 10
```

### Documentation

```
    db =: ] + ]
    :db doc. 'Doubles the right argument.'

    help. db
Doubles the right argument.
db =: ] + ]
```

### Quotes

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
    :{1+]} f 1 2 3 4 5
3 5 7 9 11
```

### Function combinators

r, s are verbs, and x, y are nouns, not with any particular value, just
representing left and right arguments

* one verb
  * `{r]} y` ← `r y`
  * `x{[r]}y` ← `x r y`
* two verbs
  * `r s`
    * `{r]}{s]}y` ← `r (s y)`
    * `{r]}x{[s]}y` ← `r (x s y)`
    * `x{[r]}{s]}y` ← `x r (s y)`
    * `x{[r]}x{[s]}y` ← `x r (x s y)`
  * `s r s`
    * `({s]}y) r {s]y}` ← `(s y) r (s y)`
    * `({s]}y) r x{[s]}y` ← `(s y) r (x s y)`
    * `(x{[s]}y) r {s]}y` ← `(x s y) r (s y)`
    * `(x{[s]}y) r x{[s]}y` ← `(x s y) r (x s y)`
  * `r s r` swap r and s above

how do we represent this in a sane way? adverbs, or higher-order functions.
→
`@`, compose
* `r@s`
  * r@s y → r (s y) → r {s]} y
  * x r@s y → x r (s y) → x r {s]} y
  * x r@.s y → r (x s y) → r x s y
  * x r@:s y → x r
  
```
amp =: [:[ ]: [:]
```

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
* able to browse the source code on github for mobile without scrolling horizontally

## syntax tree maybe

* `double =. ] + ]`
  add the right argument to the right argument
  `(+ right right)`
* `f =. [./ 2*]`
  multiply 2 times the right argument, then spread the monadic left argument through that result
  `(/ left. (* 2 right))`
* `avg =. (+/]) % #]`
  spread + through the right argument, then find the length of the right argument, then divide
  `(% (/ + right) (# right))`
* `l =. ([ avg&< ]) ~ ([,.])`
  average the right argument, then average the left argument, compare their results
  push the right argument to the left argument, pick
  `(~ (,. (left right)) (< (avg right) (avg left)))`
