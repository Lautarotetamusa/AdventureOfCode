use std::{io::{self, Read}, iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
enum Item {
    Num(u32),
    Bracket(char),
    Invalid,
    Op(String),
    Comma,
}

fn lex(input: &String) -> Vec<Item> {
    let mut items = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                items.push(Item::Num(consume_num(&mut it)));
            },
            '(' | ')' => {
                it.next();
                items.push(Item::Bracket(c));
            },
            ',' => {
                items.push(Item::Comma);
                it.next();
            }
            'm' => {
                items.push(consume_op(&mut it, "mul"));
            },
            'd' => {
                items.push(consume_op(&mut it.clone(), "don't()"));
                items.push(consume_op(&mut it, "do()"));
            }
            _ => {
                it.next();
                items.push(Item::Invalid);
            }
        }
    }
    items
}

fn consume_op(it: &mut Peekable<Chars>, keyword: &str) -> Item {
    let mut op = String::new();
    for c in it {
        op.push(c);
        if op.eq(keyword) {
            return Item::Op(op)
        }
        if op.len() >= keyword.len() {
            return Item::Invalid
        }
    }
    Item::Invalid
}

fn consume_num(it: &mut Peekable<Chars>) -> u32 {
    let mut number: u32 = 0;

    loop {
        let next = it.peek();
        if let Some(c) = next {
            if let Some(digit) = c.to_digit(10) {
                number = number * 10 + digit;
                it.next();
            }else{
                return number;
            }
        }
    }
}

fn parse(items: Vec<Item>) -> Result<u32, String> {
    let mut iter = items.into_iter().peekable();

    fn parse_expr(iter: &mut dyn Iterator<Item = Item>, enabled: &mut bool) -> Result<u32, String> {
        match iter.next() {
            Some(Item::Op(op)) if op == "mul" => {
                match iter.next() {
                    Some(Item::Bracket('(')) => {
                        match iter.next() {
                            Some(Item::Num(left)) => {
                                match iter.next() {
                                    Some(Item::Comma) => {
                                        match iter.next() {
                                            Some(Item::Num(right)) => {
                                                match iter.next() {
                                                    Some(Item::Bracket(')')) => {
                                                        //println!("{}*{} = {}", left, right, left*right);
                                                        if *enabled {
                                                            Ok(left * right)
                                                        }else {
                                                            Ok(0)
                                                        }
                                                    }
                                                    _ => Err("Missing closing parenthesis".to_string()),
                                                }
                                            },
                                            _ => Err("Expected number after comma".to_string())
                                        }
                                    }
                                    _ => Err("Expected comma after left operand".to_string()),
                                }
                            },
                            _ => Err("Expected number after left".to_string()),
                        }
                    }
                    _ => Err("Expected opening parenthesis after mul".to_string()),
                }
            },
            Some(Item::Op(op)) if op == "don't()" => {
                *enabled = false;
                parse_expr(iter, enabled)
            },
            Some(Item::Op(op)) if op == "do()" => {
                *enabled = true;
                parse_expr(iter, enabled)
            },
            Some(Item::Invalid) => Err("invalid".to_string()),
            Some(Item::Comma) => Err("comma".to_string()),
            Some(Item::Bracket(_)) => Err("bracket".to_string()),
            _ => Err("not found".to_string())
        }
    }

    let mut enabled = true;
    let mut res: u32 = 0;
    while iter.peek().is_some() {
        match parse_expr(&mut iter, &mut enabled) {
            Ok(n) => {
                //println!("{}", n);
                res += n;
            }
            Err(e) => {}//println!("{}", e)
        }
    }
    Ok(res)
}


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    //println!("{}", input);
    let items = lex(&input);
    //println!("{:?}", items);
    let res = parse(items).expect("ooa");
    println!("{}", res);
}
