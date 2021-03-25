fn main() {}

#[cfg(test)]
mod test {
    use std::fmt::Display;

    use proptest::prelude::*;
    use proptest_derive::Arbitrary;
    use time::Time;

    #[derive(Arbitrary, Debug, PartialEq)]
    enum FormatSnippet {
        H,
    }

    impl Display for FormatSnippet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                FormatSnippet::H => write!(f, "%H"),
            }
        }
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

    fn parse_input_strategy(format: &FormatSnippet) -> BoxedStrategy<String> {
        match format {
            FormatSnippet::H => ((0 as u8)..24).prop_map(|h| format!("{:02}", h)),
        }
        .boxed()
    }

    fn format_string_strategy() -> impl Strategy<Value = TimeTest> {
        let result: BoxedStrategy<TimeTest> = any::<Vec<FormatSnippet>>()
            .prop_filter("format snippets must contain at least %H", |snippets| {
                snippets.contains(&FormatSnippet::H)
            })
            .prop_flat_map(|snippets: Vec<FormatSnippet>| {
                let snippet_strategies = snippets
                    .iter()
                    .map(|snippet| parse_input_strategy(&snippet));
                let input_strategy: BoxedStrategy<String> = snippet_strategies.fold(
                    Just("".to_string()).boxed(),
                    |acc: BoxedStrategy<String>, next: BoxedStrategy<String>| {
                        (acc, next)
                            .prop_map(|(acc, next): (String, String)| format!("{}{}", acc, next))
                            .boxed()
                    },
                );
                input_strategy.prop_map(move |input| TimeTest {
                    time_string: input,
                    format_string: render_snippets(&snippets),
                })
            })
            .boxed();
        result
    }

    proptest! {
      #[test]
      fn doesnt_crash_time(
        time_test  in format_string_strategy(),
      ) {
        eprintln!("{:?}, {:?}", &time_test.time_string, &time_test.format_string);
        eprintln!("{:?}", Time::parse(time_test.time_string, time_test.format_string).unwrap());
      }
    }
}
