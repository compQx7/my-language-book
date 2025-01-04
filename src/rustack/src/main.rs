#[derive(Debug, PartialEq, Eq)]
// TODO: なぜライフタイムを使うのか？中身がすべて同じライフタイムではないから？
enum Value<'src> {
    Num(i32),
    Op(&'src str),
    Block(Vec<Value<'src>>),
}

impl<'src> Value<'src> {
    fn as_num(&self) -> i32 {
        match self {
            // TODO: なぜアスタリスクを使うのか？
            Self::Num(val) => *val,
            _ => panic!("Value is not a number."),
        }
    }
}

fn main() {
    // flattenの役割は？
    for line in std::io::stdin().lines().flatten() {
        parse(&line);
    }
}

fn parse<'a>(line: &'a str) -> Vec<Value> {
    let mut stack = vec![];
    let input: Vec<_> = line.split(" ").collect();
    // TODO: この文法が指す意味は？
    let mut words = &input[..];

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }
        if word == "{" {
            // TODO: mut になっていないが、初回の代入は mutable と見做されないのか？
            let value;
            (value, rest) = parse_block(rest);
            stack.push(value);
        } else if let Ok(parsed) = word.parse::<i32>() {
            stack.push(Value::Num(parsed));
        } else {
            match word {
                "+" => add(&mut stack),
                "-" => sub(&mut stack),
                "*" => mul(&mut stack),
                "/" => div(&mut stack),
                _ => panic!("{word:?} could not be parsed."),
            }
        }

        words = rest;
    }
    println!("Line: {words:?}");
    println!("Stack: {stack:?}");

    stack
}

// TODO: Signature の解説
fn parse_block<'src, 'a>(
    input: &'a [&'src str],
) -> (Value<'src>, &'a [&'src str]) {
    let mut tokens = vec![];
    // TODO: 文法解説
    let mut words = input;

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }
        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            tokens.push(value);
        } else if word == "}" {
            return (Value::Block(tokens), rest);
        } else if let Ok(value) = word.parse::<i32>() {
            tokens.push(Value::Num(value));
        } else {
            tokens.push(Value::Op(word));
        }
        words = rest;
    }

    (Value::Block(tokens), words)
}

fn add(stack: &mut Vec<Value>) {
    let rhs = stack.pop().unwrap().as_num();
    let lhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs + rhs));
}

fn sub(stack: &mut Vec<Value>) {
    let rhs = stack.pop().unwrap().as_num();
    let lhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs - rhs));
}

fn mul(stack: &mut Vec<Value>) {
    let rhs = stack.pop().unwrap().as_num();
    let lhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs * rhs));
}

fn div(stack: &mut Vec<Value>) {
    let rhs = stack.pop().unwrap().as_num();
    let lhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs / rhs));
}

#[cfg(test)]
mod test {
    // superは mod test が定義されている場所、つまりこのファイルのトップレベル？
    use super::{parse, Value::*};
    #[test]
    fn test_group() {
        assert_eq!(
            parse("1 2 + { 3 4 }"),
            vec![Num(3), Block(vec![Num(3), Num(4)])]
        );
    }
}
