
extern crate proc_macro;

struct Context<'a>(&'a str);

struct Parser<'c, 'a: 'c> {
    context: &'c Context<'a>,
}

impl <'c, 'a> Parser<'c, 'a> {
    fn parse(&'c self) -> Result<(), &'a str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
