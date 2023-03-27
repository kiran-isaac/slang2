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
pub mod symbol_table;
mod interpreter;

#[cfg(test)]
mod tests;
mod namespace;

pub use list::{Class, ClassPtr, Pattern, List, ListPtr, Value};

// pub struct Runtime {
//   class_table: ClassTable,
//   focus : ListPtr,
//   call_stack : Vec<ListPtr>
// }