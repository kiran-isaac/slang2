/*
need to model...
- call stack state
- call frame keeps track of named lists & which is default
- each list is a pointer; passed down lists are the same pointer
- lists have a heap allocation & pointer to their class
- classes hold onto their structure, their supertypes, and a vtable

the bask call list is just the *actual* call list though
*/

pub mod error;
pub mod list;
pub mod symtab;
pub mod interpreter;
pub mod namespace;

#[cfg(test)]
mod tests;

pub use list::{Class, Pattern, PatternType, List, Value};