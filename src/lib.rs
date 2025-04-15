///! A simple command-line argument helper for parsing positional
/// arguments and options.
use std::{collections::HashMap, env};

/// Build the `Args` object associated with the arguments
/// that the program was started with.
///
/// ```
/// # use valargs::Args;
/// # fn main() {
/// let args = Args::parse();
/// if args.has_option("nevergonnaletyoudown") {
///     println!("got rickrolled");
/// }
/// # }
/// ```
pub fn parse() -> Args {
    Args::parse()
}

/// A struct representing parsed command-line arguments.
///
/// #### Example:
///
/// ```
/// # use valargs::Args;
/// # fn main() {
/// let args = Args::parse();
///
/// if let Some(cat_name) = args.nth(1) {
///     println!("the cat's name is {}", cat_name);
/// }
///
/// if args.has_option("orange") {
///     println!("the cat is an orange cat")
/// }
///
/// if let Some(favorite_food) = args.option_value("favorite-food") {
///     println!("the cat likes {} a lot", favorite_food)
/// } else {
///     println!("no information about the cat's favorite food...")
/// }
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Args {
    args: Vec<String>,
    options: HashMap<String, Option<String>>,
}

impl Args {
    /// Build the `Args` object associated with the arguments
    /// that the program was started with.
    ///
    /// ```
    /// # use valargs::Args;
    /// # fn main() {
    /// let args = Args::parse();
    /// if args.has_option("nevergonnaletyoudown") {
    ///     println!("got rickrolled");
    /// }
    /// # }
    /// ```
    pub fn parse() -> Args {
        Args::parse_raw(&env::args().collect::<Vec<_>>())
    }

    fn parse_raw(raw_args: &[String]) -> Args {
        let l = raw_args.len();

        let mut args = Vec::new();
        let mut options = HashMap::new();

        let mut i = 0;
        while i < l {
            let token = raw_args[i].clone();

            // Process the current token correctly whether it is an option
            // (starting with "--" or "-") or an argument.
            if let Some(stripped) = token.strip_prefix("--").or_else(|| token.strip_prefix("-")) {
                // Check if the option has an associated value.
                let param = raw_args
                    .get(i + 1)
                    .map(|s| s.to_owned())
                    .filter(|s| !s.starts_with("-"));

                // Skip the next token (the next iteration) if the option has
                // an associated value.
                if param.is_some() {
                    i += 1;
                }

                options.insert(stripped.to_string(), param);
            } else {
                args.push(token);
            }
            i += 1;
        }

        Args { args, options }
    }

    /// Gets the nth argument (including the executable name).
    pub fn nth<'a>(&'a self, index: usize) -> Option<&'a str> {
        self.args.get(index).map(|s| s.as_str())
    }

    /// Check if the given option name is present.
    pub fn has_option(&self, name: &str) -> bool {
        self.options.contains_key(name)
    }
    /// Get the value associated with the given option name if
    /// present.
    pub fn option_value<'a>(&'a self, name: &str) -> Option<&'a str> {
        self.options
            .get(name)
            .and_then(|o| o.as_ref())
            .map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args() {
        let args = Args::parse_raw(
            &[
                "exec",
                "arg1",
                "arg2",
                "--option0",
                "option0_value",
                "arg3",
                "-o",
            ]
            .map(|s| s.to_string()),
        );
        assert_eq!(Some("exec"), args.nth(0));
        assert_eq!(Some("arg1"), args.nth(1));
        assert_eq!(Some("arg2"), args.nth(2));
        assert_eq!(Some("arg3"), args.nth(3));
        assert_eq!(None, args.nth(4));

        assert_eq!(Some("option0_value"), args.option_value("option0"));
        assert!(args.has_option("o"));
    }
}
