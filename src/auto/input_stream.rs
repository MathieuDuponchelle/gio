// This file was generated by gir (https://github.com/gtk-rs/gir @ bd67955+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Cancellable;
use Error;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct InputStream(Object<ffi::GInputStream, ffi::GInputStreamClass>);

    match fn {
        get_type => || ffi::g_input_stream_get_type(),
    }
}

pub trait InputStreamExt {
    fn clear_pending(&self);

    fn close<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn close_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: P, callback: Q);

    fn has_pending(&self) -> bool;

    fn is_closed(&self) -> bool;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn read_bytes<'a, P: Into<Option<&'a Cancellable>>>(&self, count: usize, cancellable: P) -> Result<glib::Bytes, Error>;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn read_bytes_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<glib::Bytes, Error>) + Send + 'static>(&self, count: usize, io_priority: glib::Priority, cancellable: P, callback: Q);

    fn set_pending(&self) -> Result<(), Error>;

    fn skip<'a, P: Into<Option<&'a Cancellable>>>(&self, count: usize, cancellable: P) -> Result<isize, Error>;

    fn skip_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<isize, Error>) + Send + 'static>(&self, count: usize, io_priority: glib::Priority, cancellable: P, callback: Q);
}

impl<O: IsA<InputStream>> InputStreamExt for O {
    fn clear_pending(&self) {
        unsafe {
            ffi::g_input_stream_clear_pending(self.to_glib_none().0);
        }
    }

    fn close<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_input_stream_close(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn close_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn close_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let _ = ffi::g_input_stream_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = close_async_trampoline::<Q>;
        unsafe {
            ffi::g_input_stream_close_async(self.to_glib_none().0, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn has_pending(&self) -> bool {
        unsafe {
            from_glib(ffi::g_input_stream_has_pending(self.to_glib_none().0))
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_input_stream_is_closed(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn read_bytes<'a, P: Into<Option<&'a Cancellable>>>(&self, count: usize, cancellable: P) -> Result<glib::Bytes, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_read_bytes(self.to_glib_none().0, count, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn read_bytes_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<glib::Bytes, Error>) + Send + 'static>(&self, count: usize, io_priority: glib::Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn read_bytes_async_trampoline<Q: FnOnce(Result<glib::Bytes, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_read_bytes_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_bytes_async_trampoline::<Q>;
        unsafe {
            ffi::g_input_stream_read_bytes_async(self.to_glib_none().0, count, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn set_pending(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_input_stream_set_pending(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn skip<'a, P: Into<Option<&'a Cancellable>>>(&self, count: usize, cancellable: P) -> Result<isize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_skip(self.to_glib_none().0, count, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn skip_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<isize, Error>) + Send + 'static>(&self, count: usize, io_priority: glib::Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn skip_async_trampoline<Q: FnOnce(Result<isize, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_skip_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = skip_async_trampoline::<Q>;
        unsafe {
            ffi::g_input_stream_skip_async(self.to_glib_none().0, count, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }
}
