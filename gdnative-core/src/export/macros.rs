#![macro_use]

#[doc(hidden)]
#[macro_export]
macro_rules! godot_wrap_method_if_deref {
    (true, $ret:expr) => {
        std::ops::Deref::deref(&$ret)
    };
    (false, $ret:expr) => {
        $ret
    };
}

// The ways of emit warnings is a terrible hack.
// This is because there is no way to emit warnings from macros in stable Rust.
//
// Follow these steps to emit warnings.
// - Detect whether reference types are used in gdnative-derive::methods::derive_methods().
// - Expand the call to the deprecated_reference_return!() macro to user code.
#[doc(hidden)]
#[macro_export]
#[deprecated = "This function does not actually pass by reference to the Godot engine. You can clarify by writing #[method(deref_return)]."]
macro_rules! deprecated_reference_return {
    () => {};
}

#[doc(hidden)]
#[macro_export]
#[deprecated = "\n#[export] is deprecated and will be removed in a future godot-rust version. Use #[method] instead. \n\
  For more information, see https://godot-rust.github.io/docs/gdnative/derive/derive.NativeClass.html."]
macro_rules! deprecated_export_syntax {
    () => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! godot_wrap_method_void {
    ($ident:ident, $void:tt) => {
        $ident
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! godot_wrap_method_inner {
    (
        $type_name:ty,
        $is_deref_return:ident,
        $map_method:ident,
        fn $method_name:ident(
            $self:ident
            $(, #[base] $base:ident : $base_ty:ty)?
            $(, $pname:ident : $pty:ty)*
            $(, #[opt] $opt_pname:ident : $opt_pty:ty)*
        ) -> $retty:ty
    ) => {
        {
            #[derive(Copy, Clone, Default)]
            struct ThisMethod;

            use $crate::export::{NativeClass, OwnerArg};
            use $crate::object::{Instance, TInstance};
            use ::gdnative::derive::FromVarargs;

            #[derive(FromVarargs)]
            #[allow(clippy::used_underscore_binding)]
            struct Args {
                $($pname: $pty,)*
                $(#[opt] $opt_pname: $opt_pty,)*
            }

            #[allow(unused_variables, unused_assignments, unused_mut)]
            impl $crate::export::StaticArgsMethod<$type_name> for ThisMethod {
                type Args = Args;
                fn call(
                    &self,
                    this: TInstance<'_, $type_name, $crate::object::ownership::Shared>,
                    Args { $($pname,)* $($opt_pname,)* }: Args,
                ) -> $crate::core_types::Variant {
                    this
                        .$map_method(|__rust_val, __base| {
                            #[allow(unused_unsafe)]
                            unsafe {
                                let ret = __rust_val.$method_name(
                                    $(OwnerArg::from_safe_ref($crate::godot_wrap_method_void!(__base,$base)),)?
                                    $($pname,)*
                                    $($opt_pname,)*
                                );
                                gdnative::core_types::OwnedToVariant::owned_to_variant(
                                    $crate::godot_wrap_method_if_deref!($is_deref_return, ret)
                                )
                            }
                        })
                        .unwrap_or_else(|err| {
                            $crate::godot_error!("gdnative-core: method call failed with error: {}", err);
                            $crate::godot_error!("gdnative-core: check module level documentation on gdnative::user_data for more information");
                            $crate::core_types::Variant::nil()
                        })
                }

                fn site() -> Option<$crate::log::Site<'static>> {
                    Some($crate::godot_site!($type_name::$method_name))
                }
            }

            $crate::export::StaticArgs::new(ThisMethod)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! godot_wrap_method_return_type {
    () => {
        ()
    };
    ($retty:ty) => {
        $retty: ty
    };
}

/// Convenience macro to wrap an object's method into a function pointer
/// that can be passed to the engine when registering a class.
#[macro_export]
macro_rules! godot_wrap_method {
    // mutable
    (
        $type_name:ty,
        $is_deref_return:ident,
        fn $method_name:ident(
            &mut $self:ident
            $(, #[base] $base:ident : $base_ty:ty)?
            $(, $pname:ident : $pty:ty)*
            $(, #[opt] $opt_pname:ident : $opt_pty:ty)*
            $(,)?
        ) $(-> $retty:ty)?
    ) => {
        $crate::godot_wrap_method_inner!(
            $type_name,
            $is_deref_return,
            map_mut,
            fn $method_name(
                $self
                $(, #[base] $base : $base_ty)?
                $(, $pname : $pty)*
                $(, #[opt] $opt_pname : $opt_pty)*
            ) -> godot_wrap_method_return_type!($($retty)?)
        )
    };
    // immutable
    (
        $type_name:ty,
        $is_deref_return:ident,
        fn $method_name:ident(
            & $self:ident
            $(, #[base] $base:ident : $base_ty:ty)?
            $(, $pname:ident : $pty:ty)*
            $(, #[opt] $opt_pname:ident : $opt_pty:ty)*
            $(,)?
        ) $(-> $retty:ty)?
    ) => {
        $crate::godot_wrap_method_inner!(
            $type_name,
            $is_deref_return,
            map,
            fn $method_name(
                $self
                $(, #[base] $base : $base_ty)?
                $(, $pname : $pty)*
                $(, #[opt] $opt_pname : $opt_pty)*
            ) -> godot_wrap_method_return_type!($($retty)?)
        )
    };
    // owned
    (
        $type_name:ty,
        $is_deref_return:ident,
        fn $method_name:ident(
            mut $self:ident
            $(, #[base] $base:ident : $base_ty:ty)?
            $(, $pname:ident : $pty:ty)*
            $(, #[opt] $opt_pname:ident : $opt_pty:ty)*
            $(,)?
        ) $(-> $retty:ty)?
    ) => {
        $crate::godot_wrap_method_inner!(
            $type_name,
            $is_deref_return,
            map_owned,
            fn $method_name(
                $self
                $(, #[base] $base : $base_ty)?
                $(, $pname : $pty)*
                $(, #[opt] $opt_pname : $opt_pty)*
            ) -> godot_wrap_method_return_type!($($retty)?)
        )
    };
    // owned
    (
        $type_name:ty,
        $is_deref_return:ident,
        fn $method_name:ident(
            $self:ident
            $(, #[base] $base:ident : $base_ty:ty)?
            $(, $pname:ident : $pty:ty)*
            $(, #[opt] $opt_pname:ident : $opt_pty:ty)*
            $(,)?
        ) $(-> $retty:ty)?
    ) => {
        $crate::godot_wrap_method_inner!(
            $type_name,
            $is_deref_return,
            map_owned,
            fn $method_name(
                $self
                $(, #[base] $base : $base_ty)?
                $(, $pname : $pty)*
                $(, #[opt] $opt_pname : $opt_pty)*
            ) -> godot_wrap_method_return_type!($($retty)?)
        )
    };
}
