#![allow(unused_imports)]

mod with_next;

use time::{Date, Time};

fn main() {
    println!("{:?}", Date::parse("20-210326", "%0C-%0y%0m%0d"));
    println!("{:?}", Date::parse("20210326", "%0C%0y%0m%0d"));
    println!("{:?}", Date::parse("999-00-01-01", "%C-%y-%m-%d"));
    println!("{:?}", Date::parse("100000101", "%C%y%m%d"));
    println!("{:?}", Date::parse("210231", "%y%m%d"));
    println!("{:?}", Time::parse("0000000001-14", "%G-%H"));
    println!("{:?}", Time::parse("0001000001-14", "%G-%H"));
    println!("{:?}", Time::parse("00000000001-14", "%G-%H"));
}

#[cfg(test)]
mod test {
    use crate::with_next::with_next;
    use proptest::{
        prelude::*,
        sample::select,
        test_runner::{Config, TestRunner},
    };
    use proptest_derive::Arbitrary;
    use std::fmt::Display;
    use time::Time;

    #[allow(non_camel_case_types)]
    #[derive(Arbitrary, Debug, PartialEq)]
    enum FormatSnippet {
        a,
        A,
        b,
        B,
        // c,
        C,
        d,
        D,
        F,
        g,
        G,
        H,
        I,
        j,
        m,
        M,
        N,
        p,
        P,
        String(String),
        LiteralPercentageSign,
    }

    impl Display for FormatSnippet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use FormatSnippet::*;
            match self {
                a => write!(f, "%a"),
                A => write!(f, "%A"),
                b => write!(f, "%b"),
                B => write!(f, "%B"),
                C => write!(f, "%C"),
                d => write!(f, "%d"),
                D => write!(f, "%D"),
                F => write!(f, "%F"),
                g => write!(f, "%g"),
                G => write!(f, "%G"),
                H => write!(f, "%H"),
                I => write!(f, "%I"),
                j => write!(f, "%j"),
                m => write!(f, "%m"),
                M => write!(f, "%M"),
                N => write!(f, "%N"),
                p => write!(f, "%p"),
                P => write!(f, "%P"),
                String(string) => write!(f, "{}", string),
                LiteralPercentageSign => write!(f, "%%"),
            }
        }
    }

    fn valid_format_snippet(snippet: &FormatSnippet) -> bool {
        if let FormatSnippet::String(string) = snippet {
            !string.contains('%') && !string.is_empty()
        } else {
            true
        }
    }

    fn valid_format_snippets(snippets: &Vec<FormatSnippet>) -> bool {
        use FormatSnippet::*;
        snippets.contains(&H) && snippets.iter().all(valid_format_snippet)
    }

    fn render_snippets(snippets: &Vec<FormatSnippet>) -> String {
        let mut result = "".to_string();
        for snippet in snippets.iter() {
            result.push_str(&format!("{}", snippet));
        }
        result
    }

    #[derive(Debug, Clone)]
    struct TimeTest {
        time_string: String,
        format_string: String,
    }

    fn parse_input_strategy(
        snippet: &FormatSnippet,
        following_snippet: Option<&FormatSnippet>,
    ) -> BoxedStrategy<String> {
        use FormatSnippet::*;
        match snippet {
            a => select(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
                .prop_map(|x| x.to_string())
                .boxed(),
            A => select(vec![
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
                "Sunday",
            ])
            .prop_map(|x| x.to_string())
            .boxed(),
            b => select(vec![
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ])
            .prop_map(|x| x.to_string())
            .boxed(),
            B => select(vec![
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ])
            .prop_map(|x| x.to_string())
            .boxed(),
            C => switch_on_following_digit(
                following_snippet,
                integer(0, 999, 3),
                integer_with_padding_strategy(0, 999, (2 as usize)..4),
            ),
            d => integer(1, 31, 2),
            D => (integer(1, 12, 1), integer(1, 31, 2), integer(0, 99, 2))
                .prop_map(|(month, day, year)| format!("{}/{}/{}", month, day, year))
                .boxed(),
            F => (integer(0, 999, 0), integer(1, 12, 2), integer(1, 31, 2))
                .prop_map(|(year, month, day)| format!("{}-{}-{}", year, month, day))
                .boxed(),
            g => integer(0, 99, 2),
            G => switch_on_following_digit(
                following_snippet,
                integer_with_padding_strategy(100000, 999999, (0 as usize)..11),
                integer_with_padding_strategy(0, 999999, (0 as usize)..11),
            ),
            H => integer(0, 23, 2),
            I => integer(1, 12, 2),
            j => integer(1, 366, 3),
            m => integer(1, 12, 2),
            M => integer(0, 59, 2),
            N => integer(0, 999999999, 9),
            p => select(vec!["am".to_string(), "pm".to_string()]).boxed(),
            P => select(vec!["AM".to_string(), "PM".to_string()]).boxed(),
            String(string) => Just(string.clone()).boxed(),
            LiteralPercentageSign => Just("%".to_string()).boxed(),
        }
    }

    fn switch_on_following_digit(
        following: Option<&FormatSnippet>,
        max_size: BoxedStrategy<String>,
        dynamic_size: BoxedStrategy<String>,
    ) -> BoxedStrategy<String> {
        match following {
            Some(following) if starts_with_digit(following) => max_size,
            _ => dynamic_size,
        }
    }

    fn integer(from: i32, to: i32, padding: usize) -> BoxedStrategy<String> {
        integer_with_padding_strategy(from, to, Just(padding))
    }

    fn integer_with_padding_strategy(
        from: i32,
        to: i32,
        padding: impl Strategy<Value = usize> + 'static,
    ) -> BoxedStrategy<String> {
        (from..(to + 1), padding)
            .prop_map(move |(x, padding)| {
                let mut result = format!("{}", x);
                while result.len() < padding {
                    result = format!("0{}", result);
                }
                result
            })
            .boxed()
    }

    fn starts_with_digit(snippet: &FormatSnippet) -> bool {
        use FormatSnippet::*;
        match snippet {
            C | d | D | F | g | G | H | I | j | m | M | N => true,
            String(string) => match string.chars().next() {
                Some(char) => char.is_digit(10),
                None => false,
            },
            _ => false,
        }
    }

    fn format_string_strategy() -> impl Strategy<Value = TimeTest> {
        let result: BoxedStrategy<TimeTest> = any::<Vec<FormatSnippet>>()
            .prop_filter("invalid format snippets", |snippets| {
                valid_format_snippets(snippets)
            })
            .prop_flat_map(|snippets: Vec<FormatSnippet>| {
                let snippet_strategies = with_next(snippets.iter())
                    .map(|(snippet, following)| parse_input_strategy(&snippet, following));
                let mut input_strategy: BoxedStrategy<String> = Just("".to_string()).boxed();
                for snippet_strategy in snippet_strategies {
                    input_strategy = (input_strategy, snippet_strategy)
                        .prop_map(|(acc, next): (String, String)| format!("{}{}", acc, next))
                        .boxed();
                }
                input_strategy.prop_map(move |input| TimeTest {
                    time_string: input,
                    format_string: render_snippets(&snippets),
                })
            })
            .boxed();
        result
    }

    #[test]
    fn doesnt_crash_time() {
        let mut runner = TestRunner::new(Config {
            failure_persistence: None,
            cases: 10000,
            max_shrink_iters: 100000,
            ..Config::default()
        });
        runner
            .run(&format_string_strategy(), |time_test| {
                eprintln!(
                    "{:?}, {:?}",
                    &time_test.time_string, &time_test.format_string
                );
                eprintln!(
                    "{:?}",
                    Time::parse(time_test.time_string, time_test.format_string).unwrap()
                );
                Ok(())
            })
            .unwrap();
    }
}
