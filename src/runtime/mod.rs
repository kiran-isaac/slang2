/*
need to model...
- call stack state
- call frame keeps track of named stacks & which is default
- each stack is a pointer; passed down stacks are the same pointer
- stacks have a heap allocation & pointer to their class
- classes hold onto their structure, their supertypes, and a vtable

the bask call stack is just the *actual* call stack though
*/

use std::ptr::NonNull;

pub mod heap;
pub mod interpreter;
pub mod bytecode;

/// A type that a value may have. Either an object w/ a class, a primitive stack, or a primitive type.
pub enum Type{
    Class(ClassPtr),
    Stack, Int, Float, Bool
}

/// A pattern that a stack may follow.
pub struct Pattern(Vec<Type>);

/// A class that an object may be instantiated from.
pub struct Class{
    name: String,
    structure: Pattern,
    supers: Vec<ClassPtr>
}

/// A pointer to a class. These are managed by the runtime.
pub struct ClassPtr(NonNull<Class>);

/// A stack containing values and a reference to its class.
pub struct Stack(Vec<Value>, Option<ClassPtr>);

/// A pointer to a stack. These are managed by the runtime.
pub struct StackPtr(NonNull<Stack>);

/// A value that can be stored on a stack.
pub enum Value{
    Stack(StackPtr),
    Int(i64),
    Float(f64),
    Bool(bool)
}