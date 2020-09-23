/*
But a system is a sequence of symbols and a set of production rules (and
technically an alphabet, but eh, that's implied)
A Symbol, accounting for parametricity, is a label: String and a sequence of
values (say, Vec<s32> to start with)
A ProductionRule has three main parts: the pattern, the optional guard, and the
replacementExpression
The pattern can have three parts - the predecessor, the target, and the
successor - only the target is mandatory
Each one of those has a structure that is like a Symbol, but slightly different
The Pattern is a label: String and a sequence of label: String or value: s32
The interpreter in the static production rule underneath takes the provided
context and the pattern, checks to see that all the top-level labels match, then
zips the corresponding Pattern.parameters with Symbol.parameters and iterates
over them
If the pattern's parameter is a label, then it pulls the symbol value and puts
it in a hash map. If the parameter is a value then it compares against the
symbol's value and returns true if they're equal, false otherwise
(the label match returns true if the label was added to the hash map; if the
label already exists in the map, then it checks the value. If those match, then
it returns true, otherwise false because the same label can't have two values)
If all that comes back true, then we've matched the pattern and have built up
the environment (the hash map) that we can use to evaluate the guard and the
replacementExpression (which I might call expansionExpression, that's a better
name)
(Eh, maybe not, it's not necessarily an expansion.)
The guard is a BooleanExpression, which can be expressed using a recursive ADT,
and the replacementExpression is a sequence of SymbolCalculation, which is
composed of a label and a sequence of ArithmeticExpressions
Each of these can be interpreted by walking their tree and replacing any labels
in them with the corresponding value from the environment, then recursively
interpreting the tree
(Or you could do the lookups while walking them. Either way.)
Interpreting a BooleanExpression returns a boolean. If that's also true, then we
can interpret the SymbolCalculations
Each SymbolCalculation results in a single Symbol, and the resultant collection
of Symbols is the replacement for the target: Symbol
And that can plug in to the existing static L-system machinery to produce a full
iteration of expansion
And then there's the matter of writing the parser that turns a strings into a
list of Symbols (for the axiom) and ProductionRules
In terms of the static L-system types, this winds up being an LSystem<Symbol>,
and there are four Production closures (one for each type of context) that all
call the same interpreter functions and close over the ProductionRule that they
interpret
So I need a function that takes a sequence of Symbols for the axiom and the
ProductionRules and creates an LSystem<Symbol> for you - wrapping up the
ProductionRules in Productions for you
Then the parser can generate a Symbol list and ProductionRules that get passed
to that, and viola
And once we have a set of Symbols and Productions, we can do some basic checking
to make sure that every Symbol with the same label has the same arity and
whatnot
*/
