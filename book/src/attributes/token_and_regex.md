# `#[token]` and `#[regex]`

For each variant your declare in your `enum` that uses the `Logos` derive macro,
you can specify one or more string literal or regex it can match.

The usage syntax is a follows:

```rust,no_run,no_playground
#[derive(Logos)]
enum Token {
    #[token(literal [, callback, priority = <integer>, ignore(<flag>, ...)]]
    #[regex(literal [, callback, priority = <integer>, ignore(<flag>, ...)]]
    SomeVariant,
}
```

where `literal` can be any `&str` or `&[u8]` string literal,
`callback` can either be a closure, or a literal path to a function
(see [Using callbacks section](../callbacks.md)),
`priority` can be any positive integer
(see [Token disambiguation section](../token-disambiguation.md)),
and `flag` can by of: `case`, `ascii_case`. Only `literal` is **required**,
others are optional.

You can stack any number of `#[token]` and or `#[regex`] attributes on top of
the same variant.

```admonish info
For a list of supported `regex` literals, read the
[Common regular expressions section](../common-regex.md).
```

## Agnostic

`#[token]` and `#[regex]` attributes may also be specified on the enum declaration itself.

If no callback is provided, the lexer will skip the matched input when it encounters these patterns (equivalent to `#[logos(skip "...")]`). Otherwise, the provided callback must return one of these types:

- [`logos::Skip`](https://docs.rs/logos/latest/logos/struct.Skip.html)
- [`logos::FilterSkip<E>`](https://docs.rs/logos/latest/logos/enum.FilterSkip.html)
- `Token`
- `Result<Token, E>`
- [`logos::Filter<Token>`](https://docs.rs/logos/latest/logos/enum.Filter.html)
- [`logos::FilterResult<Token, E>`](https://docs.rs/logos/latest/logos/enum.FilterResult.html)

(where `Token` is the type of the token enum, and `E` is convertible `Into` the error type).

The provided callback to `#[token]` and `#[regex]` attributes on specific variants must *not* return one of the above types. See the [Using callbacks section](../callbacks.md) for more information.

```rust,no_run,no_playground
#[derive(Logos)]
// Lex a number, but return the FizzBuzz equivalent
#[regex("[0-9]{1,9}", |lex| {
    let value = lex.slice().parse::<u32>().unwrap();
    if value % 15 == 0 {
        Token::FizzBuzz
    } else if value % 3 == 0 {
        Token::Fizz
    } else if value % 5 == 0 {
        Token::Buzz
    } else {
        Token::Number(value)
    }
}]
// Skip whitespace
#[regex("[ \n\f\t]+")]
enum Token {
    // Each of these are handled by the above regex
    Number(u32),
    Fizz,
    Buzz,
    FizzBuzz
}
```
