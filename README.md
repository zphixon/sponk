
# sponk

(eventually) A very small array-oriented language.

## goals

* syntax
  * minimalistic
  * not as mind-bending as J
* types
  * numbers
    * ratios
    * floats
  * bools
  * objects
    * arrays
    * strings
* functions
  * lambda
  * closures?
  * left and right arguments?
  * point-free style?
* errors?
  * end of the world, destroy everything
  * monadic error handling
  * checked exceptions

## notes

Some general Rust interpreter-writing thoughts.

* ownership is the hard part
  * interpreter and enclosed values need to mutate each other
* scope(vec\<hashmap\<string, value\>\>)
  * fn push(&mut self, other: scope)
* interpreter
  * source: vec\<statement\>
    * could also use visitor pattern
  * scope: scope
    * owns its own scope
* value
  * kind: valuekind
    * bool, int, float, etc
    * instance
    * function
  * scope: option\<scope\>
    * maybe rc\<refcell\>
    * function
      * clones interpreter's scope upon creation - would get expensive
        * analysis to not waste memory
        * do nothing and allow reflection
      * interpreter pushes value's scope before calling
        * might allow for closures
    * instance
      * used for fields, methods

