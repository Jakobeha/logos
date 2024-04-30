use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
#[logos(subpattern fizz = r"[Ff]izz")]
#[logos(subpattern buzz = r"[Bb]uzz")]
#[logos(subpattern fizz_buzz = r"(?&fizz)(?&buzz)")]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
    #[regex(r"[0-9]+")]
    Number,

    #[regex(r"(?&fizz)\([0-9]+\)")]
    Fizz,

    #[regex(r"(?&buzz)\([0-9]+\)")]
    Buzz,

    #[regex(r"(?&fizz_buzz)\([0-9]+\)")]
    FizzBuzz,
}

fn main() {
    let mut lex = Token::lexer(r"
    1
    2
    Fizz(3)
    4
    Buzz(5)
    Fizz(6)
    7
    8
    Fizz(9)
    Buzz(10)
    11
    Fizz(12)
    13
    14
    FizzBuzz(15)");

    assert_eq!(lex.next(), Some(Ok(Token::Number)));
    assert_eq!(lex.slice(), "1");

    assert_eq!(lex.next(), Some(Ok(Token::Number)));
    assert_eq!(lex.slice(), "2");

    assert_eq!(lex.next(), Some(Ok(Token::Fizz)));
    assert_eq!(lex.slice(), "Fizz(3)");

    assert_eq!(lex.next(), Some(Ok(Token::Number)));
    assert_eq!(lex.slice(), "4");

    assert_eq!(lex.next(), Some(Ok(Token::Buzz)));
    assert_eq!(lex.slice(), "Buzz(5)");

    assert_eq!(lex.next(), Some(Ok(Token::Fizz)));
    assert_eq!(lex.slice(), "Fizz(6)");

    assert_eq!(lex.next(), Some(Ok(Token::Number)));
    assert_eq!(lex.slice(), "7");

    assert_eq!(lex.next(), Some(Ok(Token::Number)));
    assert_eq!(lex.slice(), "8");

    assert_eq!(lex.next(), Some(Ok(Token::Fizz)));
    assert_eq!(lex.slice(), "Fizz(9)");

    assert_eq!(lex.next(), Some(Ok(Token::Buzz)));
    assert_eq!(lex.slice(), "Buzz(10)");

    assert_eq!(lex.next(), Some(Ok(Token::Number)));
    assert_eq!(lex.slice(), "11");

    assert_eq!(lex.next(), Some(Ok(Token::Fizz)));
    assert_eq!(lex.slice(), "Fizz(12)");

    assert_eq!(lex.next(), Some(Ok(Token::Number)));
    assert_eq!(lex.slice(), "13");

    assert_eq!(lex.next(), Some(Ok(Token::Number)));
    assert_eq!(lex.slice(), "14");

    assert_eq!(lex.next(), Some(Ok(Token::FizzBuzz)));
    assert_eq!(lex.slice(), "FizzBuzz(15)");

    assert_eq!(lex.next(), None);
}