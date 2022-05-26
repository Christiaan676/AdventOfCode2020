// Ordering of operations defines precedence rules for part2 (ADD before MUL)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Token {
    LeftParenthesis,
    RightParenthesis,
    ADD,
    MULL,
    Number(usize),
}

impl From<&str> for Token {
    fn from(token: &str) -> Self {
        match token {
            "(" => Token::LeftParenthesis,
            ")" => Token::RightParenthesis,
            "+" => Token::ADD,
            "*" => Token::MULL,
            a => Token::Number(a.parse::<usize>().unwrap()),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let sum = input
        .lines()
        .map(|line| calculate(line, false))
        .sum::<usize>();
    println!("Part1: {}", sum);

    let sum = input
        .lines()
        .map(|line| calculate(line, true))
        .sum::<usize>();
    println!("Part2: {}", sum);
}

fn calculate(input: &str, part2: bool) -> usize {
    let rpn = to_rpn(tokenize(input), part2);

    rpn.into_iter().fold(Vec::new(), |mut acc, t| {
        match t {
            Token::Number(a) => acc.push(a),
            Token::ADD => {
                let result = acc.pop().unwrap() + acc.pop().unwrap();
                acc.push(result);
            }
            Token::MULL => {
                let result = acc.pop().unwrap() * acc.pop().unwrap();
                acc.push(result);
            }
            _ => unreachable!(),
        }
        acc
    })[0]
}

fn tokenize(input: &str) -> impl Iterator<Item = &str> {
    input.split(' ')
    // Modified input so that all '(' and ')' are surrounded by ' '
    // this makes splitting simpler
    // .flat_map(|token| {
    //     if token.starts_with('(') {
    //         let (a, b) = token.split_at(1);
    //         token.sp
    //         [a, b].iter()
    //     } else if token.ends_with(')') {
    //         let (a, b) = token.split_at(token.len() - 1);
    //         [a, b].iter()
    //     } else {
    //         [token].iter()
    //     }
    // })
    // .cloned()
}

// Shunting-yard algorithm to convert infix to reverse polish operation
fn to_rpn<'a, T>(tokens: T, part2: bool) -> Vec<Token>
where
    T: Iterator<Item = &'a str>,
{
    let mut output = Vec::new();
    let mut operator_stack = Vec::new();

    for token in tokens {
        let token = Token::from(token);
        match token {
            t @ Token::Number(_) => output.push(t),
            t @ Token::LeftParenthesis => operator_stack.push(t),
            Token::RightParenthesis => {
                // Pop all operators until a left parentehsis is found
                while let Some(token) = operator_stack.pop() {
                    if token == Token::LeftParenthesis {
                        break;
                    }
                    output.push(token);
                }
            }
            t @ Token::MULL | t @ Token::ADD => {
                // Move operators untill a LeftParenthesis (keap it on the stack) is found
                // or all operators are moved
                while let Some(token) = operator_stack.pop() {
                    if token == Token::LeftParenthesis || (token >= t && part2) {
                        operator_stack.push(token);
                        break;
                    }
                    output.push(token);
                }

                operator_stack.push(t);
            }
        }
    }

    // Pop all operators and move to the output
    operator_stack
        .into_iter()
        .rev()
        .for_each(|t| output.push(t));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(71, calculate("1 + 2 * 3 + 4 * 5 + 6", false));
        assert_eq!(51, calculate("1 + ( 2 * 3 ) + ( 4 * ( 5 + 6 ) )", false));
        assert_eq!(26, calculate("2 * 3 + ( 4 * 5 )", false));
        assert_eq!(437, calculate("5 + ( 8 * 3 + 9 + 3 * 4 * 3 )", false));
        assert_eq!(
            12240,
            calculate("5 * 9 * ( 7 * 3 * 3 + 9 * 3 + ( 8 + 6 * 4 ) )", false)
        );
        assert_eq!(
            13632,
            calculate(
                "( ( 2 + 4 * 9 ) * ( 6 + 9 * 8 + 6 ) + 6 ) + 2 + 4 * 2",
                false
            )
        );
    }

    #[test]
    fn part2_examples() {
        assert_eq!(231, calculate("1 + 2 * 3 + 4 * 5 + 6", true));
        assert_eq!(51, calculate("1 + ( 2 * 3 ) + ( 4 * ( 5 + 6 ) )", true));
        assert_eq!(46, calculate("2 * 3 + ( 4 * 5 )", true));
        assert_eq!(1445, calculate("5 + ( 8 * 3 + 9 + 3 * 4 * 3 )", true));
        assert_eq!(
            669060,
            calculate("5 * 9 * ( 7 * 3 * 3 + 9 * 3 + ( 8 + 6 * 4 ) )", true)
        );
        assert_eq!(
            23340,
            calculate(
                "( ( 2 + 4 * 9 ) * ( 6 + 9 * 8 + 6 ) + 6 ) + 2 + 4 * 2",
                true
            )
        );
    }
}
