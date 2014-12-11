The Alonzo Language Specification
=================================
Alonzo is a language heavily based on Lambda Calculus' syntax, and minimalism. Alonzo also uses Haskell's idea of purity, although not quite to that extent. And because of Alonzo's use of Scheme-eschew style macros, very little cannot be implemented in the language itself.
Alonzo does not aim to be Object-Oriented in any way, only supporting algebraic data types (for example, Haskell's Option type) and enumerations. Functions are values, and are proper lambdas. All functions are curried automatically, as in Haskell, except that missing arguments are represented as an underscore. There are no operators defined by the language, just functions, although in the standard library, infix macros are defined for operators.
Lists are not built into the language, but instead are implemented in it. There are no loops of any kind, except the usual map, filter, reject, each, etc. Recursion is highly recommended for tasks that require indefinite amounts of looping. Since there are no operators, there are no special characters that you cannot put in variable names. Even if you define
an infix macro, it will be delimited by whitespace. Alonzo uses Scheme-like brace, paren, and curly brace inter polarity, but Alonzo has some best practices when using one or the other type of brace, to give it a more C-like feel, but otherwise is pretty much exactly the same as lambda calculus. Functions only ever take one argument, and return functions that take the other arguments.
There are not to be any extensions to the original runtime after the language is finished. Any additional features _will_ be added to the standard library as macros, always. The language is implemented in Rust for maximum portability. Any operating system should be able to use it.
Alonzo compiles to LLVM, and is parsed by a Top Down Operator Precedence (Based on Douglas Crockford's article). There will never, in any reimplementation of the language be any other features than these in the language:

  1. [True Lambdas](#functions)
  1. [Variable Assignment](#var)
  1. [Strings](#strings)
  1. [Macros](#macros)

**NOTE**: Comments are a macro, and so are lists and data types and enumerations.

**NOTE**: There are tests for every single of these features in the `parser_tests.rs`, `tokenizer_tests.rs`, and `compiler_tests.rs`.

<a name="functions"></a> Functions
---------
Lambdas are essential for any Turing-complete language, as lambda calculus can prove. The definition of a lambda is a function that has no name, but instead can be passed around as easily as a number or boolean. A lambda, when created, captures its surrounding environment in a closure.
Alonzo has been heavily inspired by Lambda Calculus, as even professed by its name. The syntax we have chosen is very similar to the lambda calculus syntax, but slightly influenced by C to make it more readable to the average programmer.
Here is an example of a normal lambda in Alonzo:

    {λx . λy . λz .
      xyz := +(x, y, z)
      *(xyz, 9)
    }(1, 2, 3)

As you may have noticed, those λ characters require unicode. This is not necessary if your editor or environment is not supportive to that type of unicode programming, but it is recommended, because it makes much more clear your intentions to make a lambda similar to that of a lambda calculus one. Also note that functions are curried, as professed by the lambda syntax, just as in Lambda Calculus.
Parenthisis and curly braces and square brackets are all completely equivalent, but to avoid a lispy feel, we mostly use curly braces to surround blocks, parenthesis to call functions and wrap expressions, and square brackets to go around the list macro. This is so that users can use most of the visual cues they are used to.
Here is the non-unicode version:

    {\x . \y . \z .
      xyz := +(x, y, z)
      *(xyz, 9)
    }(1, 2, 3)

Here is an example of currying:

    +1 := {λx . λy . λz .
      +(x, y, z)
    }(1, _, _)
    +4 := +1(4, _)
    +9 := +4(9) ; => 14 (:
    +3nums := {λx . λy . λz .
      +(x, y, z)
    }
    +3nums(1, 2, 3) ; => 6

Lambdas with no arguments are like this:

    {λ_ . 1 + 1}

Functions are called with no arguments by using empty parens (`()`)

### Side-Effects
Side effects are not allowed in normal functions at all. All functions must return the same thing when given the same arguments as previously. This is similar to Haskell, except that instead of using monads, which I find very confusing and hard to understand for beginners, we use a sort of 1/2 monad:

    main := {do .
      println "HW"
    }

Side effects are only allowed in a do-block, and do blocks can only be assigned to the main variable, which is called immediately if it is there. The last return value of the program is otherwise used. Which brings me to one point: The last expression in a do-block or lambda is the return value, which is highly convenient for functional programming.

<a name="var"></a> Variable Assignment
---------
Variables are immutable, and all structures, such as arrays and datatypes are therefor immutable. Alonzo uses Lexical scoping with variables and functions.
Variables are assigned like this:

    set('foobar', 2)

but in the standard library a macro for assignment is defined to make it easier and clearer:

    foobar := 2

The format is `<identifier> := <expression>` where an identifier is any sequence of characters, symbols, or numbers that is not already used. Variables can have numbers that come first, because numbers are defined inside the language anyway. All infix operators that you see are macros. So are lists, numbers, conditionals, and booleans.
To make a contract of a type of a variable just use a special form of comment:

    foobar::Int := 2

This special comment only comments one identifier, which it then associates with the identifier on the other side, and then it checks to make sure that the 2 is an Int. Since variables are immutable, this isn't really that much use, but for people who want it, it's there. (written as a macro).

<a name="strings"></a> Strings
-------
Strings are created between `"` or `'`. There is no string templating or concatenation or substitution or slice built in, but all that is possible inside the standard library, with macros. Strings can be indexed though.

    get 0 , "Hello, World!"
    get 5 , 'Hello, World!'

<a name="macros"></a> Macros
-------
Macros are the central feature in Alonzo, because they are the reason that it is possible to write such a tiny language and yet get a readable, high-level functional language. Scheme has macros, but they are not obvious about their use, and not very easy to read their definition. So, I have decided to use the Rust or Sweet.JS style syntax for macros. Macros are going to be one of the most sophisticated features of this language.

FAQ
===

  1. **How do I identify data types?**
  Built-In datatypes can be identified by the `type` function, but user-defined types cannot.

  2. **Will Alonzo be able to load files in the future?**
  File loading will probably not be implemented, because of the certain imperativeness and unsafeness that comes with that. In the future, they might be added though.

  3. **Why the Rust Programming Language vs. C or C++?**
  Rust is a very modern, flexible, and functional programming language, with closures, lambdas, and many other good features, as well as direct bindings for LLVM, and A good package manager and fast benchmark speed. It is also very robust, and type-safe, as well as avoiding all the dangerous pitfalls of C. It is also as fast or faster then C. C++ is also far to strange, dangerous, imperative, and just plain impalpable.

  4. **Will Alonzo implement feature X in Y time?**
  No. Not ever. Implement it yourself. We have macros, and a very flexible syntax. And, do you really need X??

  4. **How do I learn Alonzo?**
  Since the majority of programmers aren't functional programmers, and this is a very hard-core functional language, as well as not being marketed very well, and being very small, you can either contact me, put up an issue, write a tutorial yourself, read the rust code, or read the standard library. The last option is recommended.
