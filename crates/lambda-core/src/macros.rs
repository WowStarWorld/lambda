#[macro_export]
macro_rules! impl_downcast {
    ($trait_name: ident) => {
        impl dyn $trait_name {
            pub fn is<T: $trait_name>(&self) -> bool {
                self.type_id() == std::any::TypeId::of::<T>()
            }
            pub fn downcast<T: $trait_name>(&self) -> Option<&T> {
                if self.is::<T>() {
                    Some(unsafe { &*(self as *const dyn $trait_name as *const T) })
                } else {
                    None
                }
            }
            pub fn downcast_mut<T: $trait_name>(&mut self) -> Option<&mut T> {
                if self.is::<T>() {
                    Some(unsafe { &mut *(self as *mut dyn $trait_name as *mut T) })
                } else {
                    None
                }
            }
        }
    };
}
