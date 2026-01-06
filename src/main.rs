use std::collections::HashMap;
use std::{io, process};

use clap::{Arg, Command};
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use semver::{Version, VersionReq};

struct FindRep;

impl FindRep {
    fn new() -> FindRep {
        FindRep
    }
}

impl Preprocessor for FindRep {
    fn name(&self) -> &str {
        "findrep"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut kv: HashMap<String, String> = HashMap::new();

        if let Some(cfg) = ctx.config.get_preprocessor(self.name()) {
            for (k, v) in cfg {
                if let Some(s) = v.as_str() {
                    kv.insert(k.to_string(), s.to_string());
                }
            }
        }

        book.for_each_mut(|section: &mut BookItem| {
            if let BookItem::Chapter(c) = section {
                for (k, v) in &kv {
                    c.content = c.content.replace(&format!("${}", k.to_uppercase()), v);
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
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
