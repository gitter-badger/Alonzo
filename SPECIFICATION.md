The Nanite Language Specification
=================================
Nanite is a language loosely based on Haskell's syntax, immutability, and destructering in functions. Nanite does have multiple dispatch built in, and because of its use of Scheme-eschew style macros, very little cannot be implemented in the language itself.
Nanite does not aim to be Object-Oriented in any way, only supporting algebraic data types (for example, Haskell's Option type) and enumerations. Functions are values, and are proper lambdas. There are no variables, just shorthand function syntax that looks like variable assignment.
Since there are no variables, to call a function with no arguments, just place it's name. All functions are curried automatically, as in Haskell. There are no operators defined by the language, just functions, although in the standard library, infix macros are defined for operators.
Lists are not built into the language, but instead are implemented in it. There are no loops of any kind, exept the usual map, filter, reject, each, etc. Recursion is highly recommended for tasks that require indefinite amounts of looping. Since there are no operators, there are no special characters that you cannot put in variable names. Even if you defeine
an infix macro, it will be delimited by whitespace. Nanite uses C-style block syntax, but otherwise is more similar to Haskell. Functions only ever take one argument, which is a list, which is destructured by the argument list in the function definition. From there, arguments can also be further destructured.
There are not to be any extensions to the original runtime after the language is finished. Any additional features _will_ be added to the standard library as macros, always. The language is implemented in Rust for maximum portability. Any operating system should be able to use it.
Nanite compiles to LLVM, and is parsed by a Top Down Operator Precedence (Based on Douglas Crockford's article). There will never, in any reimplementation of the language be any other features than these in the language:

  1. [True Lambdas](#functions)
    * Function Definitions
    * Function Expression Definitions
    * Lambda Functions
    * Multiple dispatch
    * Calls
  1. [Symbols](#symbols)
  1. [Booleans](#booleans)
  1. [Strings](#strings)
  1. [Conditions](#conditionals)
  1. [Macros](#macros)

**NOTE**: Comments are a macro, and so are lists and data types and enumerations. Symbols are not necessary, but extremely nice to have for symbolic operations.

**NOTE**: There are tests for every single of these features in the `parser_tests.rs`, `tokenizer_tests.rs`, and `compiler_tests.rs`.

<a name="functions"></a> Functions
---------
There are three types of functions, all with similar syntax:

  * Function definitions
  * Function expression definitions (variable declarations)
  * Lambdas (Anonymous functions)

### Function Definitions
Functions are defined using normal lexical scope, with no hoisting. The syntax is similar to Haskell:

    let list-a-list (x:xs) = { println x. list-a-list xs }
    let list-a-list (x) = { println x }
    let list-a-list ([]) = {  }

Notice that recursion, argument destructuring, and multiple dispatch are all supported. Also, other than assigning the value, it returns the function that was assigned.
The function arguments are really just destructuring the list passed in, which looks like arguments! The syntax is not quite consistent, but it is the same function under the hood.
### Function Expression Definitions
Since there are no variable declarations in Nanite, we might have to define variables like this:

    let foo () = { 1 }

But that is extremely verbose for such a common thing, so there is a macro defined in the standard library that makes it possible to do this:

    var foo = 1

### Lambda Functions
Lambda functions are just function definitions, with names and everything, because all functions have closures and recursion capabilities, and like previously stated, creating a function also returns the function, as well as assigning the name a value.
### Multiple dispatch
Functions are dispatched based on which one's argument list matches the arguments passed in. That simple. If there is an ambiguity, we cannot guaranty that one or the other will be chosen.
### Calls
Functions calls are basically passing a list into the function, which is destructured by the "arguments", which are actually just destructuring assignment. If there is an ambiguity, such as:

    get index-of-user user , array-of-users

The arguments are fed into the inner most function until it needs no more arguments, and the result of that function and the rest of the arguments are passed into the outer function.

<a name="symbols"></a> Symbols
-------
Symbols are surrounded by a backtick. Symbols always equal themselves, and symbols with the same name **are** the same symbol.
Example:

    let my-symbol = `sjflkjflkj`

<a name="booleans"></a> Booleans
-------
Booleans are like symbols, but there can only be two: `true` and `false`.

    eq true , true
    eq false , false
    not eq true , false

`and` and `or` are short-circuiting macros that are built using conditionals, which are another one of the things that are built in.

<a name="strings"></a> Strings
-------
Strings are created between `"` or `'`. There is no string templating or concatenation or substitution or slice built in, but all that is possible inside the standard library, with macros. Strings can be indexed though.

    get 0 , "Hello, World!"
    get 5 , 'Hello, World!'

<a name="conditionals"></a> Conditions
-------
There is only one. And there will never be more than The One. COND the Great! On a more serious note, a language does not need more than one conditional, except as syntactic sugar. Which can be supplied by macros.

    condition {
      eq 1 , 1 => let rec () = { println "foo" }
      not eq(1 , 1) => let rec () = { println "bar" }
    }

The syntax basically uses this form:

    condition {
      <expression0> => <lambda to call>
      <expression1> => <lambda to call>
      ...
      else          => <lambda to call>
    }

<a name="macros"></a> Macros
-------
Macros are the central feature in Nanite, because they are the reason that it is possible to write such a tiny language and yet get a readable, high-level functional language. Scheme has macros, but they are not obvous about their use, and not very easy to read their definition. So, I have decided to use the Rust or Sweet.JS style syntax for macros. Macros are going to be one of the most sophisticated features of this language.

FAQ
===

  1. **How do I identify data types?**
  Built-In datatypes can be identified by the `type` function, but user-defined types cannot, unless you specifically define a symbol property on them that allows them to be identified, and the ovveride a dispatch of the `type` funciton for that.

  2. **Will Nanite be able to load files in the future?**
  File loading will probably not be implemented, becouse of the certain imperitiveness and unsafeness that comes with that. In the future, they might be added though.

  3. **Why the Rust Programming Language vs. C or C++?**
  Rust is a very modern, flexible, and functional programming language, with closures, lambdas, and many other good features, as well as direct bindings for LLVM, and A good package manager and fast benchmark speed. It is also very robust, and type-safe, as well as avoiding all the dangerous pitfalls of C. It is also as fast or faster then C. C++ is also far to strange, dangerous, imperitive, and just palain implatable.

  4. **Will Nanite implement feature X in Y time?**
  No. Not ever. Implemnt it yourself. We have macros, and a very flexible syntax. And, do you really need X??

  4. **How do I learn Nanite?**
  Since the majority of programmers arn't functional programmers, and this is a very hard-core functional language, as well as not being marketed very well, and being very small, you can either contact me, put up an issue, write a tutorial yourself, read the rust code, or read the standard library. The last option is recommened.
