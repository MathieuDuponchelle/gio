// This file was generated by gir (9e87ba8+) from gir-files (469db10)
// DO NOT EDIT

use Cancellable;
use Error;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Permission(Object<ffi::GPermission, ffi::GPermissionClass>);

    match fn {
        get_type => || ffi::g_permission_get_type(),
    }
}

pub trait PermissionExt {
    fn acquire<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn acquire_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<(), Error>) + Send + Sync + 'static>(&self, cancellable: P, callback: Q);

    fn get_allowed(&self) -> bool;

    fn get_can_acquire(&self) -> bool;

    fn get_can_release(&self) -> bool;

    fn impl_update(&self, allowed: bool, can_acquire: bool, can_release: bool);

    fn release<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn release_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<(), Error>) + Send + Sync + 'static>(&self, cancellable: P, callback: Q);

    fn get_property_allowed(&self) -> bool;

    fn get_property_can_acquire(&self) -> bool;

    fn get_property_can_release(&self) -> bool;

    fn connect_property_allowed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_can_acquire_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_can_release_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Permission> + IsA<glib::object::Object>> PermissionExt for O {
    fn acquire<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_permission_acquire(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn acquire_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<(), Error>) + Send + Sync + 'static>(&self, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Fn(Result<(), Error>) + Send + Sync + 'static>> = Box::new(Box::new(callback));
        extern "C" fn acquire_async_trampoline(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            unsafe {
                let mut error = ptr::null_mut();
                let _ = ffi::g_permission_acquire_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
                let callback: &&(Fn(Result<(), Error>) + Send + Sync + 'static) = transmute(user_data);
                callback(result);
            }
        }
        let callback = acquire_async_trampoline;
        unsafe {
            ffi::g_permission_acquire_async(self.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn get_allowed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_allowed(self.to_glib_none().0))
        }
    }

    fn get_can_acquire(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_can_acquire(self.to_glib_none().0))
        }
    }

    fn get_can_release(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_can_release(self.to_glib_none().0))
        }
    }

    fn impl_update(&self, allowed: bool, can_acquire: bool, can_release: bool) {
        unsafe {
            ffi::g_permission_impl_update(self.to_glib_none().0, allowed.to_glib(), can_acquire.to_glib(), can_release.to_glib());
        }
    }

    fn release<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_permission_release(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn release_async<'a, P: Into<Option<&'a Cancellable>>, Q: Fn(Result<(), Error>) + Send + Sync + 'static>(&self, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Fn(Result<(), Error>) + Send + Sync + 'static>> = Box::new(Box::new(callback));
        extern "C" fn release_async_trampoline(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            unsafe {
                let mut error = ptr::null_mut();
                let _ = ffi::g_permission_release_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
                let callback: &&(Fn(Result<(), Error>) + Send + Sync + 'static) = transmute(user_data);
                callback(result);
            }
        }
        let callback = release_async_trampoline;
        unsafe {
            ffi::g_permission_release_async(self.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn get_property_allowed(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "allowed".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_can_acquire(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "can-acquire".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_can_release(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "can-release".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn connect_property_allowed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::allowed",
                transmute(notify_allowed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_can_acquire_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::can-acquire",
                transmute(notify_can_acquire_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_can_release_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::can-release",
                transmute(notify_can_release_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_allowed_trampoline<P>(this: *mut ffi::GPermission, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Permission> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Permission::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_can_acquire_trampoline<P>(this: *mut ffi::GPermission, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Permission> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Permission::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_can_release_trampoline<P>(this: *mut ffi::GPermission, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Permission> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Permission::from_glib_borrow(this).downcast_unchecked())
}
