use crate::*;
use std::io;
use std::io::*;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn build() -> Templar {
    let mut builder = TemplarBuilder::default();
    builder.add_function("prompt", prompt);
    builder.build()
}

fn prompt(args: Data) -> Data {
    let prompt_string = data_unwrap_into!(String: args);
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true)).unwrap_or_default();
    write!(&mut stdout, "[{}] ➣ ", prompt_string).unwrap_or_default();
    stdout.set_color(ColorSpec::new().set_reset(true)).unwrap_or_default();
    // print!("{} ➣ ", prompt_string);
    stdout.flush().unwrap_or_default();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.into(),
        Err(e) => TemplarError::IO(e.to_string()).into()
    }
}
