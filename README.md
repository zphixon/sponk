
# sponk

(eventually) A very small array-oriented language.

Sponk mostly follows the same philosophy of J, except it has a slightly more sane syntax. Also with APL symbols because
I love the shit out of them.

## todo
* [ ] specification
* [ ] parser
* [ ] interpreter
* [ ] compiler to shader language????
* [ ] row polymorphism??
* [ ] should we call it tree-oriented instead of array-oriented?

symbols:

```
`1234567890-=qwertyuiop[]\asdfghjkl;'zxcvbnm,./
~!@#$%^&*()_+QWERTYUIOP{}|ASDFGHJKL:"ZXCVBNM<>?
⋄¨¯<≤=≥>≠∨∧×÷?⍵∊⍴~↑↓⍳○*←→⊢⍺⌈⌊_∇∆∘'⎕⍎⍕⊂⊥⊤|⍝⍀⌿
⌺⌶⍫⍒⍋⌽⍉⊖⍟⍱!⌹⍷⍨⍸⍥⍣⍞⍬⊣⍺⍤⌸⌷≡≢⊆⊃∩∪⍪⍙⍠
```

## lil ideas

if ⍴⍴array > 2 you can name the ranks??? that sounds super dope, and select ranks like ⌽⍉⊖ with those names instead of
ridiculous [rank] syntax

### Data types

* bool (maybe?)
* integer
* floating point
* real
* ratio
* string
* atom (`∆atom`)
* array: honestly the term 'array' is a little misleading. it's more like a tree, but in array programming languages we
  tend to give names to the special cases. a flat tree we know as a vector, a flat tree of n-vectors we call a matrix, a
  flat tree of flat trees of n-vectors we call a cube, and so on.

Sponk operates on n-dimensional arrays.

```apl
    x ← 1 2 3 4 5
    y ← 6 7 8 9 10
    x + y
┌→───────────┐
│7 9 11 13 15│
└~───────────┘

    x × y
┌→──────────-─┐
│6 14 24 36 50│
└~──────────-─┘

    ⍴ x
5
    ⍴⍴ x
1
    # x
5
    ⍝ there *is* a difference between ⍴ and #!
```

You can manipulate arrays with several operators.

```apl
    ⍝ list manipulation
    x ← 1 2 3 4 5
    y ← 6 7 8 9 10
    z ← 11 12 13 14 15

    ⍝ append
    x,y,z
┌→──────────────────────────────────┐
│1 2 3 4 5 6 7 8 9 10 11 12 13 14 15│
└~──────────────────────────────────┘

    ⍝ push
    x⍪y
┌→─────────┐
↓1 2 3 4  5│
│6 7 8 9 10│
└~─────────┘

    x⍪y⍪z
┌→─────────────┐
↓ 1  2  3  4  5│
│ 6  7  8  9 10│
│11 12 13 14 15│
└~─────────────┘

    ⍝ shape, length, rank of arrays
    a ← x⍪y⍪z
    ⍴ a       ⍝ a is a 3x5 array
3 5
    1 ⌷ ⍴ a   ⍝ a has three elements in its first rank
3
    ⍴⍴ a      ⍝ a is rank 2 (2-dimensional)
2

    ⍝ pick
    1 ⌷ 1 2 3 4
1
    8 ⌷ 1 2 3 4
╭ index out of bounds
│   8 ⌷ 1 2 3 4
╰ wanted 8, but array is length 4

    2 ⌷ a
6 7 8 9 10

    ⍝ multi-dimensional indexing: start with the largest rank first
    2 4 ⌷ a
9

    1 1 ⌷ 1 2 3 4
╭ rank mismatch
│   1 1 ⌷ 1 2 3 4
╰ array is rank 1, but picked 2nd order element
```

### Quotes and functions

```apl
    x ← 32

    ⍝ quote the expression
    {x + x}
{x+x}

    ⍝ evaluate the expression with empty argument ⍬
    {x+x} ⍬
64

    ⍝ quotes are lazy
    twox ← {x + x}
    twox ⍬
64

    x ← 8
    twox ⍬
16

    ⍝ ⍺ is right arg, ⍵ is left arg
    {⍵ + ⍵}
{]+]}

    ⍝ evaluate without argument
    {⍵ + ⍵} ⍬
╭ missing argument
│   {⍵+⍵}⍬
╰ expected right argument, got nothing

    ⍝ try again with an argument
    {⍵ + ⍵} x
64

    ⍝ you might call a name bound to a quote with arguments a function
    double ← {⍵ + ⍵}
    double
{⍵+⍵}

    double x
64
```

### Arguments

TODO: meh

`⍺` and `⍵` refer to the left and right arguments of the quote they're inside.
⍺⍺ ⍵⍵ yes. ⍺⍵ ⍵⍺ ⍵⍺⍺ ⍺⍵⍵ ⍺⍵⍺ ⍵⍺⍵ ⍺⍺⍵ ⍵⍵⍺ etc?

### Documentation, aliases for built-in functions

```apl
    )help ⍴
Shape
⍴ ⍵ - Length of each rank of ⍵
e.g.
    a ← 1 2 3 4
    ⍴ a
4
    a ← a⍪a
    ⍴ a
2 4

⍺ ⍴ ⍵ - Change the shape of ⍵ to fit the dimensions specified by ⍺
e.g.
    ⍳12
1 2 3 4 5 6 7 8 9 10 11 12
    3 4 ⍴ ⍳12
1  2  3  4
5  6  7  8
9 10 11 12

    db ← {⍵ + ⍵}
    )doc db 'Doubles the right argument.'

    )help db
db - Doubles the right argument.
```

### Shape as structure

Matrices are super cool, but they aren't all that useful if you want to assign more meaningful structure to your data.
Hence some mechanisms available for adding semantic structure to your data.

```apl
    ⍝ we all know and love types, but classic APL/J/etc don't really *do* them very well.
    ⍝ I don't really think array-based languages are good for 

    ⍝ say you want to represent a person. what you might do in APL:
    robert ← 'Robert Dufresne' 1992 6 4
    ⍝ and just remember that the first item of a person is their name, the second is their birth year, and so on.
    ⍝ or, use types!! note that strings are true character vectors. this robert is a 'mixed vector'.

    ⍝ dyad ⎕ defines a type constructor. you must quote the inner types as atoms:
    ∆name ⎕ ∆str
    ∆year ⎕ ∆int
    ∆month ⎕ ∆int
    ∆day ⎕ ∆int
    ∆person ⎕ ∆name ∆year ∆month ∆day

    ⍝ use the type constructor with a 2xn shape array:
    robert ← person (name 'Robert Dufresne') (year 1992) (month 6) (day 4)

    ⍴ robert
∆name ∆year ∆month ∆day
    # robert     ⍝ dope!
4
    
    ⍝ use ⌷ to get each property
    ∆name ⌷ robert
'Robert Dufresne'
    ∆year ⌷ robert
1992

    ⍝ another
    michael ← person (name 'Michael Tomlinson') (year 1989) (month 8) (day 17)

    ⍝ and now, for a magic trick
    people ← robert,michael
    people
person (name 'Robert Dufresne') (year 1992) (month 6) (day 4)
person (name 'Michael Tomlinson') (year 1989) (month 8) (day 17)

    ∆name ⌷ people
'Robert Dufresne' 'Michael Tomlinson'

    ⍝ 🤯

    ⍝ there's an example data set from 'Mastering Dyalog APL' called Prod. it's defined as follows:
    ⍝ rank 1: years of production for our factory
    ⍝ rank 2: each individual production line
    ⍝ rank 3: each month's produced goods
    Prod
┌┌→──────────────────────────────────┐
↓↓26 16 22 17 21 44 25 22 23 44 41 33│
││43 36 47 49 30 22 57 20 45 60 43 22│
││                                   │
││44 21 58 57 17 43 47 17 43 26 53 23│
││29 19 23 38 53 47 38 22 40 57 35 26│
││                                   │
││37 27 53 26 29 46 25 26 30 20 32 16│
││56 55 25 47 38 27 39 59 20 28 42 25│
││                                   │
││21 57 55 44 16 54 26 16 55 56 45 45│
││16 55 26 20 27 55 36 39 43 38 50 16│
││                                   │
││27 23 56 41 53 60 39 47 44 47 17 28│
││24 35 61 26 22 35 24 20 31 35 47 37│
└└~──────────────────────────────────┘

    ⍝ without looking at the explanation above, how exactly do you get March's production statistics
    ⍝ for the second assembly line from two years ago (assuming it is now january)? it's hard to remember
    ⍝ the exact shape of our data in this form, which is why we invented databases in the first place!

    ⍝ in APL, perhaps: which is which?
    2 2 3 ⌷ ⊖Prod
26
    ⍝ alternatively
    (⊖Prod)[2;2;3]
26

    ⍝ define a shape-type with ⎕
    ∆year ⎕ ∆int
    ∆line ⎕ ∆int
    ∆month ⎕ ∆int
    ∆productionLine ⎕ ∆year ∆line ∆month

    ⍝ write Prod initially like
    Prod ← ∆productionLine ((26 16 22 17 ...) (43 36 47 49 ...)) ((44 21 ...) ...) ...

    ⍝ or reshape the existing prod
    newProd ← ∆productionLine ⍴ Prod
    ⍴ newProd
∆year ∆line ∆month
    (∆year 2) (∆line 2) (∆month 3) ⌷⊖newProd
26
    (⊖newProd)[∆year 2; ∆line 2; ∆month 3]
26

    ⍝ the ⌷ syntax for indexing is better don't @ me
```

### Spread, unquote

```
    ⍝ equivalent to 1 + 2 + 3 + 4 + 5
    +/ 1 2 3 4 5

    ⍝ spread left argument through 2* right argument
    f ← {⍺/ 2*⍵}

    {1+⍵} f 1 2 3 4 5
╭ not a dyad
│   f←{⍺/2*⍵}
│ '/' requires a dyad
╰   {1+⍵} f 1 2 3 4 5

    ⍝ so you need to pass a dyad instead
    {⍺+1+⍵} f 1 2 3 4 5
34

    ⍝ spread + through right argument, divide by length of right argument
    avg ← {(+/ ⍵) ÷ (⍴ ⍵)}
    avg 42 8 15 4 16 23
18

    ⍝ choose the array with the larger average
    largest ← {(1 + avg ⍺ < avg ⍵) ⌷ (⍺⍪⍵)}

    x ← 1 2 3 4 5
    y ← 1 1 1 1 80
    x largest y
1 1 1 1 80
```

### "Combinators"

TODO: does this still make sense if we're gonna do what we have above? idk

Unlike J, functions will always be surrounded by their arguments. The implicit "hook" and "fork" constructs are made
explicit in Sponk. Using the parenthesized versions will expand the functions every time they are referenced, but the
combinator versions will not.

| Combinator  | Expression  | Equivalent to   |
| ----------- | ----------- | --------------- |
| `&`         | `r&s y`     | `r (s y)`       |
|             | `x r&s y`   | `(s x) r (s y)` |
| `&.`        | `r&.s y`    | `y r (s y)`     |
|             | `x r&.s y`  | `x r (s y)`     |
| `@`         | `r@s y`     | `r (s y)`       |
|             | `x r@s y`   | `r (x s y)`     |

## goals

* general sanity
* syntax
  * more of a blend between J and APL
* types
  * copy/paste from J
* quotes
  * left and right arguments
* errors -------- make these good
  * report early and often, especially syntax

## syntax tree maybe

* `double ← {⍵ + ⍵}`
  add the right argument to the right argument
  `(+ ⍵ ⍵)`
* `f ← {⍺/ 2*⍵}`
  multiply 2 times the right argument, then spread the left argument through
  that result
  `(/ ⍺ (* 2 ⍵))`
* `avg ← {+/⍵ ÷ ⍴⍵}`
  spread + through the right argument, then find the length of the right
  argument, then divide
  `(÷ (/ + ⍵) (⍴ ⍵))`
* `l ← {avg ⍺ < avg ⍵ ⌷ ⍺⍪⍵}`
  average the right argument, then average the left argument, compare their
  results. push the right argument to the left argument, pick.
  `(~ (,. (left right)) (< (avg right) (avg left)))`

user-identifiers are utf-8, excluding symbols:
```
-`=[]\;',./~!@#$%^&*()_+{}|:"<>?⋄¨¯<≤=≥>≠∨∧×÷?⍵∊⍴~↑↓⍳○*←→⊢⍺⌈⌊_∇∆∘'⎕⍎⍕⊂⊥⊤|⍝⍀⌿⌺⌶⍫⍒⍋⌽⍉⊖⍟⍱!⌹⍷⍨⍸⍥⍣⍞⍬⊣⍺⍤⌸⌷≡≢⊆⊃∩∪⍪⍙⍠
```

## more rigid definitions/thoughts

### definitions
* quote: `{}` a sponk program fragment, can contain references to bindings outside itself, lazily evaluated
* atom: a literal, a number, a string, an array, a value
* function: a quote with arguments `[` `]`
* combinator: in a loose sense, a higher-order function. dw about all that lambda calculus junk, where we're going you
  won't need it. you can't define your own because idk how to write that down in a way that makes sense and is
  consistent with what I want.

## execution model

* name resolution → logic tree building → quote expansion → evaluation
* quotes are simple substitution
  * for example
    ```
    double =. 2*]     -- define a function
    double  3         -- use it
    {2 * ]} 3         -- expand name -> {}
    {2 * 3} :         -- ] -> arg, arg -> :
     2 * 3            -- evaluate
     6                -- evaluate
    ```
  * more (this is actually wrong cause it's evaluating `y` before the full quote is expanded but whatever you get the
    point the quote expands and everything is evaluated from there)
    ```
    avg =. (+/]) % (#])
    largest =. ([ <&avg ]) ~ ([,.])

    x = 1 2 3 4 5
    y = 1 1 1 1 80
    x largest y
    x { (                                  [  <&avg                                 ]   ~ ([         ,. ]         ) } y
    x { ((avg                              [) < (avg                                ])) ~ ([         ,. ]         ) } y
    x { (({(+/ ]                ) % (# ])} [) < ({(+/ ]         ) % (# ]         )} ])) ~ ([         ,. ]         ) } y
    : { (({(+/ ]        ) % (# ]        )} x) < ({(+/ ]         ) % (# ]         )} y)) ~ (x         ,. y         ) } :
    : { (({(+/ x        ) % (# x        )} :) < ({(+/ y         ) % (# y         )} :)) ~ (x         ,. y         ) } :
        (({(+/ x        ) % (# x        )} :) < ({(+/ y         ) % (# y         )} :)) ~ (1 2 3 4 5 ,. 1 1 1 1 80)
        (({(+/ x        ) % (# x        )} :) < ( (+/ 1 1 1 1 80) % (# 1 1 1 1 80)   )) ~  1 2 3 4 5 \n 1 1 1 1 80
        (({(+/ x        ) % (# x        )} :) < ( (84           ) % (5           )   )) ~  1 2 3 4 5 \n 1 1 1 1 80
        (( (+/ 1 2 3 4 5) % (# 1 2 3 4 5)   ) < ( (84           ) % (5           )   )) ~  1 2 3 4 5 \n 1 1 1 1 80
        (( (15          ) % (5          )   ) < ( (84           ) % (5           )   )) ~  1 2 3 4 5 \n 1 1 1 1 80
        ((                3                 ) < (                 16.8               )) ~  1 2 3 4 5 \n 1 1 1 1 80
        (                                     1                                       ) ~  1 2 3 4 5 \n 1 1 1 1 80
                                                                                        1 1 1 1 80
    ```
* parsing uses a symbol table to look up definitions to determine what syntax is being constructed. it is known at parse
  time what each name refers to, so the big-ass table above can be disambiguated.
* each binding is determined to be an atom/function/combinator and checked for validity
* execution stops at a quote
  * if it contains a reference to an argument, it is a function

## Inspiration

J, [BYOL](http://www.buildyourownlisp.com/), Dyalog APL, Scheme

⍴⍴⍴ your boat
