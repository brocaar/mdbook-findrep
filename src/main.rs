use std::collections::HashMap;
use std::{io, process};

use clap::{Arg, Command};
use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext, parse_input};
use toml::value::Value;

struct FindRep;

impl FindRep {
    const NAME: &str = "findrep";

    fn new() -> FindRep {
        FindRep
    }
}

impl Preprocessor for FindRep {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut kv: HashMap<String, String> = HashMap::new();
        let preprocessors = ctx.config.preprocessors::<Value>()?;

        if let Some(config) = preprocessors.get(FindRep::NAME) {
            if let Some(table) = config.as_table() {
                for (k, v) in table {
                    kv.insert(k.to_string(), v.as_str().unwrap_or_default().into());
                }
            }
        }

        book.for_each_mut(|section: &mut BookItem| {
            if let BookItem::Chapter(c) = section {
                for (k, v) in &kv {
                    c.content = c.content.replace(&format!("%{}", k.to_uppercase()), v);
                }
            }
        });

        Ok(book)
    }
}

fn make_app() -> Command {
    Command::new("findrep")
        .about("mdBook find / replace processor")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();
    let preprocessor = FindRep::new();

    if let Some(_) = matches.subcommand_matches("supports") {
        process::exit(0);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{e:?}");
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook_preprocessor::MDBOOK_VERSION {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook_preprocessor::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
