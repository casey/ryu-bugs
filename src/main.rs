fn main() {}

#[cfg(test)]
mod test {
    use std::fmt::Display;

    use proptest::{prelude::*, sample::select};
    use proptest_derive::Arbitrary;
    use time::Time;

    #[allow(non_camel_case_types)]
    #[derive(Arbitrary, Debug, PartialEq)]
    enum FormatSnippet {
        a,
        H,
        String(String),
        LiteralPercentageSign,
    }

    impl Display for FormatSnippet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use FormatSnippet::*;
            match self {
                a => write!(f, "%a"),
                H => write!(f, "%H"),
                String(string) => write!(f, "{}", string),
                LiteralPercentageSign => write!(f, "%%"),
            }
        }
    }

    fn valid_format_snippet(snippet: &FormatSnippet) -> bool {
        if let FormatSnippet::String(string) = snippet {
            !string.contains('%')
        } else {
            true
        }
    }

    fn valid_format_snippets(snippets: &Vec<FormatSnippet>) -> bool {
        snippets.contains(&FormatSnippet::H) && snippets.iter().all(valid_format_snippet)
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
        use FormatSnippet::*;
        match format {
            a => select(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
                .prop_map(|x| x.to_string())
                .boxed(),
            H => ((0 as u8)..24).prop_map(|h| format!("{:02}", h)).boxed(),
            String(string) => Just(string.clone()).boxed(),
            LiteralPercentageSign => Just("%".to_string()).boxed(),
        }
    }

    fn format_string_strategy() -> impl Strategy<Value = TimeTest> {
        let result: BoxedStrategy<TimeTest> = any::<Vec<FormatSnippet>>()
            .prop_filter("invalid format snippets", |snippets| {
                valid_format_snippets(&snippets)
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
