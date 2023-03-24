use internal_list::List;

/// A class that an object may be instantiated from.
pub struct Class{
  name: String,
  supers: Vec<ClassPtr>,
  list: List
}

/// A pointer to a class. These are managed by the runtime.
pub struct ClassPtr(NonNull<Class>);