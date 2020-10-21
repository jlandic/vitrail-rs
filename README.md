# Vitrail

![Build](https://img.shields.io/github/workflow/status/jlandic/vitrail-rs/Rust/master?style=flat-square&logo=github-actions)
[![License](https://img.shields.io/github/license/jlandic/vitrail?style=flat-square)](https://opensource.org/licenses/MIT)

## Introduction

`Vitrail` is a text generator using context-free grammars, inspired by projects like [Tracery](https://github.com/galaxykate/tracery), or [Grammy](https://github.com/AlmasB/grammy).

It can be used on the spot by providing a grammar as a JSON file, and following the default syntax, and can also be used as a library.

_I also have a Kotlin version [here](https://github.com/jlandic/vitrail) ;)_

## Features

- Read grammar from JSON file
- Shared variables, available through all symbol expansions
- Customisable grammar syntax/operators (when used as a library)
- Potential for custom modifiers (when used as a library)

### JSON Grammar file

```json
{
    "root": [
        "[character>subject]Let's {speak} about {subject}. Did you know that {subject} {verb} {object:s:capitalize}?"
    ],
    "speak": [
        "speak",
        "babble",
        "talk"
    ],
    "character": [
        "Alice",
        "Bob",
        "Eve"
    ],
    "verb": [
        "shared",
        "ate",
        "saw",
        "destroyed",
        "stole",
        "lost"
    ],
    "object": [
        "the apple",
        "the banana",
        "the pancake",
        "a cinnamon roll"
    ]
}
```

### Rust Code

```rust
use vitrail::{
    config::GrammarSyntax,
    grammar::Grammar,
    modifier::{
        CapitalizeModifier,
        PluralizeModifier,
    },
};

let mut grammar = Grammar::from_json(
    "test.json",
    "any random seed",
    GrammarSyntax::default(),
)
        .with_modifier("capitalize".to_string(), &CapitalizeModifier {})
        .with_modifier("s".to_string(), &PluralizeModifier {});

    // Randomly expand the grammar 15 times
    for _ in 0..15 {
        println!("{}", &grammar.flatten());
    }
```

### Example output

```
Let's talk about Eve. Did you know that Eve stole The pancakes?
Let's speak about Bob. Did you know that Bob ate The pancakes?
Let's babble about Eve. Did you know that Eve shared The bananas?
Let's talk about Bob. Did you know that Bob saw The apples?
Let's babble about Bob. Did you know that Bob ate The pancakes?
Let's talk about Eve. Did you know that Eve lost A cinnamon rolls?
Let's speak about Bob. Did you know that Bob stole The bananas?
Let's speak about Bob. Did you know that Bob stole The bananas?
Let's talk about Bob. Did you know that Bob destroyed The bananas?
Let's babble about Bob. Did you know that Bob saw The apples?
Let's speak about Eve. Did you know that Eve saw The apples?
Let's talk about Bob. Did you know that Bob lost The bananas?
Let's babble about Alice. Did you know that Alice stole The bananas?
Let's babble about Alice. Did you know that Alice destroyed The bananas?
Let's babble about Eve. Did you know that Eve shared The bananas?
```

### Grammar features

#### Symbol expansion (default: `{ }`)

_Looks up the rules corresponding to the symbol, and picks one randomly as a replacement_

```json
{
  "root": "{subject} {verb} {object}",
  "subject": [
    "Alice",
    "Bob"
  ],
  "verb": [
    "shares",
    "eats",
    "sees"
  ],
  "object": [
    "the apple",
    "the banana"
  ]
}
```

#### Variable capture (default: `[symbol>variableName]`)

_Creates a `variableName` symbol with a static value. The rules corresponding to `symbol` determine the value of `variableName`.
This value never changes once it's initialized._

```json
{
  "root": "[person>subject]{subject} {verb}. {subject} also {verb}",
  "person": [
    "..."
  ],
  "...": []
}
```

In the example above, both `subject` symbols will expand with the value coming from `person`.

> Note:
> The placement of the variable capture matters!
>
> The algorithm expands symbols from left to right. This means that a symbol expansion captured inside a variable is only available for expressions on its right side.
>
> Otherwise, placement is free.

> Note (expanding on the previous note):
>
> While placement is free for now, future plans involve allowing for more granularity regarding the scope of variables.
>
> One possibility would be to declare captures inside a symbol to make it available only inside the scope of this symbol's expansion:
> ```json
> {
>   "root": "{[person>subject]part1}. {[person>subject]part2}"
> }
> ```
> In the example above, `subject` could have different values in the expansion of `part1` and `part2`.
>
> Do keep this in mind when writing your grammars.

#### Modifiers (default: `:modifierName`)

_Applies a modification on the expanded value of a symbol._

> Note: while a `Modifier` interface exists, and modifiers added to the `Grammar` at runtime are applied, the library does not provide any modifier yet.

```json
{
  "root": "{subject:capitalize}",
  "subject": [
    "john"
  ]
}
```

Considering that the `Grammar` has a modifier with the name `capitalize`, that capitalizes the first letter of the value passed to it, the result of the example above would be:
```
"John"
```

## License

`Vitrail` is released under [MIT License](https://opensource.org/licenses/MIT)