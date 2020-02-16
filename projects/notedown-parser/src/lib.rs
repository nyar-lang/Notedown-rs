#[cfg(test)]
#[macro_use]
extern crate quote;
extern crate pest;
#[cfg(test)]
extern crate pest_generator;
#[cfg(test)]
extern crate proc_macro;

#[cfg(test)]
mod pre_build;

mod note_down;
// mod note_text;

pub use note_down::{NoteDownParser, Rule as NoteDownRule};
// pub use note_text::{NoteTextParser, Rule as NoteTextRule};
