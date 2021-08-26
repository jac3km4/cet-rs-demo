pub trait CppClass {
    /// # Safety
    /// Invokes the function pointer expecting provided types
    unsafe fn lookup_method<A>(&self, idx: usize) -> A;
}

pub trait CppClassMethods {
    const VTABLE_SIZE: usize;
}
