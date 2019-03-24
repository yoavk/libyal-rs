pub trait AsFFIPtr {
    type Target;
    fn as_ffi_ptr(&mut self) -> *mut *mut Self::Target;
}


#[macro_export]
macro_rules! impl_as_ffi_ptr {
    ($ffi_handle_type: ident, $ffi_handle_field: ident, $s: ident) => {
        impl $crate::ffi::AsFFIPtr for $s {
            type Target = $ffi_handle_type;
            fn as_ffi_ptr(&mut self) -> *mut *mut Self::Target {
                let mut ptr = &mut self.$ffi_handle_field as *mut _;
                &mut ptr as *mut _
           }
        }
    }
}

macro_rules! impl_as_ffi_ptr_primitive {
    ($ffi_handle_type: ident, $s: ident) => {
        impl $crate::ffi::AsFFIPtr for $s {
            type Target = $ffi_handle_type;
            fn as_ffi_ptr(&mut self) -> *mut *mut Self::Target {
                let mut ptr = self as *mut _;
                &mut ptr as *mut _
           }
        }
    }
}


impl_as_ffi_ptr_primitive!(isize, isize);
impl_as_ffi_ptr_primitive!(i32, i32);