
# sponk

(eventually) A very small array-oriented language.

Sponk mostly follows the same philosophy of J, except it has a slightly more
sane syntax.

## todo
* [ ] specification
* [ ] parser
* [ ] interpreter
* [ ] compiler to shader language????

## lil ideas

Sponk operates on n-dimensional arrays.

```
    x = 1 2 3 4 5
    y = 6 7 8 9 10
    x + y
7 9 11 13 15

    x * y
6 14 24 36 50

    # x
5
    $ x
5
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
    # $ a -- a is rank 2 (2-dimensional)
2

    -- pick
    0 ~ 1 2 3 4
1
    8 ~ 1 2 3 4
╭ index out of bounds
│   8 ~ 1 2 3 4
╰ wanted 8, but array is length 4

    1 ~ a
6 7 8 9 10

    3 1 ~ a
9

    0 1 ~ 1 2 3 4
╭ rank mismatch
│   0 1 ~ 1 2 3 4
╰ array is rank 1, but picked 2nd order element
```

### Quotes are functions

```
    x =. 32

    -- quote the expression
    {x + x}
{x+x}

    -- evaluate the expression with empty argument :
    {x+x} :
64

    -- quotes are lazy
    2x = {x + x}
    2x:
64

    x = 8
    2x:
16

    -- ] is right arg, [ is left arg
    {] + ]}
{]+]}

    -- evaluate without argument
    {] + ]} :
╭ missing argument
│   {]+]}:
╰ expected right argument, got nothing

    -- try again with an argument
    {] + ]} x
64

    -- you might call a name bound to a quote with arguments a function
    double = {] + ]}
    double
{]+]}

    double x
64

    -- use =. to automatically quote the expression
    double =. ] + ]
    double x
64
```

### Arguments

`[` and `]` refer to the left and right arguments of the quote they're inside.
You can use `[.` and `].` to refer to arguments of outer quotes, and even
further up with more `.`.

### Documentation, aliases for built-in quotes

```
    Help '$'
$: Returns the length of each rank of the right argument.
Also written as Shape. See also #.

    db =. ] + ]
    'db' Doc 'Doubles the right argument.'

    Help 'db'
db: Doubles the right argument.
db =. ]+]
```

### Variations

```
    -- we've seen `,` and `,.`, how do we make our own?
    x =. 2*]
    x 10
20

    x. =. 3+]
    x 10
20
    x. 10
13

    -- oh
    x.. =. [+ 2*]
    10 x.. 10
30

    -- document each one separately
    'x' Doc 'Double the right argument'
    'x.' Doc 'Add three to the right argument'
    'x..' Doc 'Two times the left argument plus the right argument'
```

### Spread, unquote

```
    -- equivalent to 1 + 2 + 3 + 4 + 5
    +/ 1 2 3 4 5

    -- spread left argument through 2* right argument
    f =. [/ 2*]

    {1+]} f 1 2 3 4 5
╭ not a dyad
│   f=.[/2*]
│ '/' requires a dyad
╰   {1+]} f 1 2 3 4 5

    -- so you need to pass a dyad instead
    {[+1+]} f 1 2 3 4 5
34

    -- spread + through right argument, divide by length of right argument
    avg =. +/] % #]
    avg 42 8 15 4 16 23
18

    -- choose the array with the larger average
    largest =. (avg[ < avg]) ~ ([,.])

    x = 1 2 3 4 5
    y = 1 1 1 1 80
    x l y
1 1 1 1 80
```

### Combinators

As a little thought experiment, consider the following table. `r`, `s` are
quotes, and `x`, `y` are values.

| Expression      | With parentheses    | Traditionally written | Equivalent to
| --------------- | ------------------- | --------------------- | ---
| `r y`           | `r y`               | `r(y)`                |
| `x r y`         | `x r y`             | `r(x, y)`             |
| `r s y`         | `r (s y)`           | `r(s(y))`             |
| `x r s y`       | `x r (s y)`         | `r(x, s(y))`          |
| `r x s y`       | `r (x s y)`         | `r(s(x, y))`          |
| `s y r s y`     | `(s y) r (s y)`     | `r(s(y), s(y))`       | `r@s y`
| `s y r x s y`   | `(s y) r (x s y)`   | `r(s(y), s(x, y))`    |
| `x s y r s y`   | `(x s y) r (s y)`   | `r(s(x, y), s(y))`    |
| `x s y r x s y` | `(x s y) r (x s y)` | `r(s(x, y), s(x, y))` |

<!-- WOW LOOK AT THEM LINE ENDINGS -->

The first three expressions are fully readable without knowledge of the bindings
of any of these names, but what about the rest? We need combinators in order for
humans to be able to differentiate these expressions from lists, and so we don't
end up evaluating `x` and `y` multiple times. We'll name the quotes conjoined by
a combinator `t`.

| Expression  | As above        | AKA             | Looks like |
| ----------- | --------------- | --------------- | ---------- |
| `r@s y`     | `(s y) r (s y)` | `r(s(y), s(y))` | `t y`      |
| `x r@s y`   | `(s x) r (s y)` | `r(s(x), s(y))` | `x t y`    |

You could even implement a version of `@` yourself if you wanted using `[.` and
`[.`, although the builtin uses some magic to make it nicer:

```
    x = 1
    y = 2
    r =. [ + 3 + ]
    s =. 2* ]

    amp =. [. [ ] ].

    x r (s y)
5
    x r@s y
5
    x {r amp s} y
5
```

## goals

* general sanity
* syntax
  * more of a blend between J and APL
* types
  * copy/paste from J
* quotes
  * left and right arguments
* errors
  * report early and often, especially syntax
* able to browse the source code on github for mobile without scrolling
  horizontally

## syntax tree maybe

* `double =. ] + ]`
  add the right argument to the right argument
  `(+ right right)`
* `f =. [/ 2*]`
  multiply 2 times the right argument, then spread the left argument through
  that result
  `(/ left (* 2 right))`
* `avg =. (+/]) % #]`
  spread + through the right argument, then find the length of the right
  argument, then divide
  `(% (/ + right) (# right))`
* `l =. ([ avg&< ]) ~ ([,.])`
  average the right argument, then average the left argument, compare their
  results. push the right argument to the left argument, pick.
  `(~ (,. (left right)) (< (avg right) (avg left)))`

two types of identifiers:
* ascii punctuation: ``! " # $ % & ' ( ) * + , - . / ; < = > ? @ \ ^ _ ` | ~``
  excluding `:`, `[`, `]`, `{`, `}`
* ascii alphanumerics `[a-zA-Z][a-zA-Z0-9]`

these are all valid:
* `ding`, `d0ng`, `+.`, `-`

## more rigid definitions/thoughts

### definitions
* quote: `{}` a sponk program fragment, can contain references to bindings
  outside itself, lazily evaluated
* atom: a literal, a number, a string, an array, a value
* function: a quote with arguments `[` `]`
* combinator: in a loose sense, a higher-order function. dw about all that
  lambda calculus junk, where we're going you won't need it. you can't define
  your own because idk how to write that down in a way that makes sense and is
  consistent with what I want.

## execution model

* name resolution → tree building → quote expansion → evaluation
* quotes are simple substitution
  * for example
    ```
    double =. 2*]     -- define a function
    double  3         -- use it
    {2 * ]} 3         -- name -> {}
    {2 * 3} :         -- ] -> arg, arg -> :
     2 * 3            -- evaluate
     6                -- evaluate
    ```
* parsing uses a symbol table to look up definitions to determine what syntax
  is being constructed. it is known at parse time what each name refers to, so
  the big-ass table above can be disambiguated.
* each binding is determined to be an atom/function/combinator and checked for
  validity
* execution stops at a quote
  * if it contains a reference to an argument, it is a function

## Inspiration

J, [BYOL](http://www.buildyourownlisp.com/), APL, Scheme
