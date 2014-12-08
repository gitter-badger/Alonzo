# Nanite Functional Programming Language
## Haskell + JavaScript + Scheme = Nanite!
Nanite is a functional programming language based on Haskell's syntax (minus segnificant whitespace), ideas of pure functions and auto currying. It also uses JavaScript's idea of dynamic typing, plus JavaScript's curly braces, and Scheme's macros and size.
Here is how the language looks with it's standard library (Nanobit):

    let my-fun (x, y, z) = {
      x + y + z
    }

    let my-var = 124           ; Variables are functions under the hood, so we could keep the amount of up-front features down. In fact,
    let my-varfunc (x) = x + 1   ; the variable declaration is just a single-expression function!

    let my-bool = True         ; boolean true
    let my-bool2 = False       ; boolean false

    let my-num = 1             ; numbers
    let my-str = 'Sjfklsjlk'   ; strings
    let my-list = 1 : 2 : 3 : 4 ; Comma is just a macro that outputs this:
    ; cons 1, (cons 2, (cons 3, (cons 4, nil)))
    ; And since there are no variables, a lone identifier calls a function!!!
    my-var ; ==> 124

    my-varfunc 1 ; ==> 2

    my-fun 1, 2, 3

    ; Algebreic datatypes are just functions and macros too.....
    data Light = Red | Green | Yellow
    ; Which transforms to:
    ; let Light (mode) = {
    ;   if eq(mode, 'Red') {
    ;     return Red
    ;   }
    ;   if eq(mode, 'Green') {
    ;     return Green
    ;   }
    ;   if eq(mode, 'Yellow') {
    ;     return Yellow
    ;   }
    ; }
    ;
    ; let Red = 'Red'
    ; let Green = 'Green'
    ; let Yellow = 'Yellow'
