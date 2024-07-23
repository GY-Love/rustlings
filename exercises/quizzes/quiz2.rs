enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // Complete the function.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        input.into_iter().map(|(s, command)| {
            match command {
                Command::Uppercase => s.to_uppercase(),
                Command::Trim => s.trim().to_string(),
                Command::Append(n) => {
                    let mut result = s;
                    for _ in 0..n {
                        result.push_str("bar");
                    }
                    result
                }
            }
        }).collect()
    }
}

fn main() {
    // You can optionally experiment here.
    let input = vec![
        ("hello".to_string(), Command::Uppercase),
        (" all roads lead to rome! ".to_string(), Command::Trim),
        ("foo".to_string(), Command::Append(1)),
        ("bar".to_string(), Command::Append(5)),
    ];
    let output = my_module::transformer(input);

    for s in output {
        println!("{}", s);
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    use super::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
