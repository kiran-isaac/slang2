/*
need to model...
- call stack state
- call frame keeps track of named lists & which is default
- each list is a pointer; passed down lists are the same pointer
- lists have a heap allocation & pointer to their class
- classes hold onto their structure, their supertypes, and a vtable

the bask call list is just the *actual* call list though
*/

use std::ptr::NonNull;

pub mod heap;
pub mod interpreter;
pub mod bytecode;

/// A type that a value may have. Either an object w/ a class, a primitive list, or a primitive type.
pub enum Type{
    Class(ClassPtr),
    List, Int, Float, Bool
}

/// A pattern that a list may follow.
pub struct Pattern(Vec<Type>, bool);

/// A class that an object may be instantiated from.
pub struct Class{
    name: String,
    structure: Pattern,
    supers: Vec<ClassPtr>
}

/// A pointer to a class. These are managed by the runtime.
pub struct ClassPtr(NonNull<Class>);

/// A list containing values and a reference to its class.
pub struct List(Vec<Value>, Option<ClassPtr>);

/// A pointer to a list. These are managed by the runtime.
pub struct ListPtr(NonNull<List>);

/// A value that can be stored on a list.
pub enum Value{
    List(ListPtr),
    Int(i64),
    Float(f64),
    Bool(bool)
}