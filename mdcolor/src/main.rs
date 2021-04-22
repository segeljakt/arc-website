use clap::{App, Arg, ArgMatches, SubCommand};
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use std::io;
use std::process;

pub fn make_app() -> App<'static, 'static> {
    App::new("color-preprocessor")
        .about("A mdbook preprocessor which adds colors")
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    let preprocessor = AddColor::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
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

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(&renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

pub struct AddColor;

impl AddColor {
    pub fn new() -> AddColor {
        AddColor
    }
}

impl Preprocessor for AddColor {
    fn name(&self) -> &str {
        "color-preprocessor"
    }

    fn run(&self, _: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let mdbook::BookItem::Chapter(ch) = item {
                ch.content = ch.content.replace("<'", "<b>");
                ch.content = ch.content.replace("'>", "</b>");
                ch.content = ch.content.replace("'>", "</b>");
                //                     ch.content = ch.content.replace("<!", "<b style='color:green'>");
                //                     ch.content = ch.content.replace("!>", "</b>");
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
