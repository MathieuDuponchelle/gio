// This file was generated by gir (https://github.com/gtk-rs/gir @ bd67955+)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Cancellable;
use Error;
use IOStream;
use SocketAddress;
use SocketClientEvent;
use SocketConnectable;
use SocketConnection;
use SocketFamily;
use SocketProtocol;
use SocketType;
use TlsCertificateFlags;
use ffi;
use glib;
use glib::StaticType;
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
    pub struct SocketClient(Object<ffi::GSocketClient, ffi::GSocketClientClass>);

    match fn {
        get_type => || ffi::g_socket_client_get_type(),
    }
}

impl SocketClient {
    pub fn new() -> SocketClient {
        unsafe {
            from_glib_full(ffi::g_socket_client_new())
        }
    }
}

impl Default for SocketClient {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SocketClientExt {
    fn add_application_proxy(&self, protocol: &str);

    fn connect<'a, P: IsA<SocketConnectable>, Q: Into<Option<&'a Cancellable>>>(&self, connectable: &P, cancellable: Q) -> Result<SocketConnection, Error>;

    fn connect_async<'a, P: IsA<SocketConnectable>, Q: Into<Option<&'a Cancellable>>, R: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, connectable: &P, cancellable: Q, callback: R);

    fn connect_to_host<'a, P: Into<Option<&'a Cancellable>>>(&self, host_and_port: &str, default_port: u16, cancellable: P) -> Result<SocketConnection, Error>;

    fn connect_to_host_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, host_and_port: &str, default_port: u16, cancellable: P, callback: Q);

    fn connect_to_service<'a, P: Into<Option<&'a Cancellable>>>(&self, domain: &str, service: &str, cancellable: P) -> Result<SocketConnection, Error>;

    fn connect_to_service_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, domain: &str, service: &str, cancellable: P, callback: Q);

    fn connect_to_uri<'a, P: Into<Option<&'a Cancellable>>>(&self, uri: &str, default_port: u16, cancellable: P) -> Result<SocketConnection, Error>;

    fn connect_to_uri_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, uri: &str, default_port: u16, cancellable: P, callback: Q);

    fn get_enable_proxy(&self) -> bool;

    fn get_family(&self) -> SocketFamily;

    fn get_local_address(&self) -> Option<SocketAddress>;

    fn get_protocol(&self) -> SocketProtocol;

    //#[cfg(any(feature = "v2_36", feature = "dox"))]
    //fn get_proxy_resolver(&self) -> /*Ignored*/Option<ProxyResolver>;

    fn get_socket_type(&self) -> SocketType;

    fn get_timeout(&self) -> u32;

    fn get_tls(&self) -> bool;

    fn get_tls_validation_flags(&self) -> TlsCertificateFlags;

    fn set_enable_proxy(&self, enable: bool);

    fn set_family(&self, family: SocketFamily);

    fn set_local_address<'a, P: IsA<SocketAddress> + 'a, Q: Into<Option<&'a P>>>(&self, address: Q);

    fn set_protocol(&self, protocol: SocketProtocol);

    //#[cfg(any(feature = "v2_36", feature = "dox"))]
    //fn set_proxy_resolver<'a, P: IsA</*Ignored*/ProxyResolver> + 'a, Q: Into<Option<&'a P>>>(&self, proxy_resolver: Q);

    fn set_socket_type(&self, type_: SocketType);

    fn set_timeout(&self, timeout: u32);

    fn set_tls(&self, tls: bool);

    fn set_tls_validation_flags(&self, flags: TlsCertificateFlags);

    fn get_property_type(&self) -> SocketType;

    fn set_property_type(&self, type_: SocketType);

    fn connect_event<F: Fn(&Self, SocketClientEvent, &SocketConnectable, &Option<IOStream>) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_enable_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_local_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn connect_property_proxy_resolver_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tls_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tls_validation_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SocketClient> + IsA<glib::object::Object>> SocketClientExt for O {
    fn add_application_proxy(&self, protocol: &str) {
        unsafe {
            ffi::g_socket_client_add_application_proxy(self.to_glib_none().0, protocol.to_glib_none().0);
        }
    }

    fn connect<'a, P: IsA<SocketConnectable>, Q: Into<Option<&'a Cancellable>>>(&self, connectable: &P, cancellable: Q) -> Result<SocketConnection, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect(self.to_glib_none().0, connectable.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_async<'a, P: IsA<SocketConnectable>, Q: Into<Option<&'a Cancellable>>, R: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, connectable: &P, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn connect_async_trampoline<R: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_async_trampoline::<R>;
        unsafe {
            ffi::g_socket_client_connect_async(self.to_glib_none().0, connectable.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn connect_to_host<'a, P: Into<Option<&'a Cancellable>>>(&self, host_and_port: &str, default_port: u16, cancellable: P) -> Result<SocketConnection, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_host(self.to_glib_none().0, host_and_port.to_glib_none().0, default_port, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_to_host_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, host_and_port: &str, default_port: u16, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn connect_to_host_async_trampoline<Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_host_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_host_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_client_connect_to_host_async(self.to_glib_none().0, host_and_port.to_glib_none().0, default_port, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn connect_to_service<'a, P: Into<Option<&'a Cancellable>>>(&self, domain: &str, service: &str, cancellable: P) -> Result<SocketConnection, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_service(self.to_glib_none().0, domain.to_glib_none().0, service.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_to_service_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, domain: &str, service: &str, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn connect_to_service_async_trampoline<Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_service_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_service_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_client_connect_to_service_async(self.to_glib_none().0, domain.to_glib_none().0, service.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn connect_to_uri<'a, P: Into<Option<&'a Cancellable>>>(&self, uri: &str, default_port: u16, cancellable: P) -> Result<SocketConnection, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_uri(self.to_glib_none().0, uri.to_glib_none().0, default_port, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_to_uri_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, uri: &str, default_port: u16, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn connect_to_uri_async_trampoline<Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_uri_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_uri_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_client_connect_to_uri_async(self.to_glib_none().0, uri.to_glib_none().0, default_port, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn get_enable_proxy(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_client_get_enable_proxy(self.to_glib_none().0))
        }
    }

    fn get_family(&self) -> SocketFamily {
        unsafe {
            from_glib(ffi::g_socket_client_get_family(self.to_glib_none().0))
        }
    }

    fn get_local_address(&self) -> Option<SocketAddress> {
        unsafe {
            from_glib_none(ffi::g_socket_client_get_local_address(self.to_glib_none().0))
        }
    }

    fn get_protocol(&self) -> SocketProtocol {
        unsafe {
            from_glib(ffi::g_socket_client_get_protocol(self.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v2_36", feature = "dox"))]
    //fn get_proxy_resolver(&self) -> /*Ignored*/Option<ProxyResolver> {
    //    unsafe { TODO: call ffi::g_socket_client_get_proxy_resolver() }
    //}

    fn get_socket_type(&self) -> SocketType {
        unsafe {
            from_glib(ffi::g_socket_client_get_socket_type(self.to_glib_none().0))
        }
    }

    fn get_timeout(&self) -> u32 {
        unsafe {
            ffi::g_socket_client_get_timeout(self.to_glib_none().0)
        }
    }

    fn get_tls(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_client_get_tls(self.to_glib_none().0))
        }
    }

    fn get_tls_validation_flags(&self) -> TlsCertificateFlags {
        unsafe {
            from_glib(ffi::g_socket_client_get_tls_validation_flags(self.to_glib_none().0))
        }
    }

    fn set_enable_proxy(&self, enable: bool) {
        unsafe {
            ffi::g_socket_client_set_enable_proxy(self.to_glib_none().0, enable.to_glib());
        }
    }

    fn set_family(&self, family: SocketFamily) {
        unsafe {
            ffi::g_socket_client_set_family(self.to_glib_none().0, family.to_glib());
        }
    }

    fn set_local_address<'a, P: IsA<SocketAddress> + 'a, Q: Into<Option<&'a P>>>(&self, address: Q) {
        let address = address.into();
        let address = address.to_glib_none();
        unsafe {
            ffi::g_socket_client_set_local_address(self.to_glib_none().0, address.0);
        }
    }

    fn set_protocol(&self, protocol: SocketProtocol) {
        unsafe {
            ffi::g_socket_client_set_protocol(self.to_glib_none().0, protocol.to_glib());
        }
    }

    //#[cfg(any(feature = "v2_36", feature = "dox"))]
    //fn set_proxy_resolver<'a, P: IsA</*Ignored*/ProxyResolver> + 'a, Q: Into<Option<&'a P>>>(&self, proxy_resolver: Q) {
    //    unsafe { TODO: call ffi::g_socket_client_set_proxy_resolver() }
    //}

    fn set_socket_type(&self, type_: SocketType) {
        unsafe {
            ffi::g_socket_client_set_socket_type(self.to_glib_none().0, type_.to_glib());
        }
    }

    fn set_timeout(&self, timeout: u32) {
        unsafe {
            ffi::g_socket_client_set_timeout(self.to_glib_none().0, timeout);
        }
    }

    fn set_tls(&self, tls: bool) {
        unsafe {
            ffi::g_socket_client_set_tls(self.to_glib_none().0, tls.to_glib());
        }
    }

    fn set_tls_validation_flags(&self, flags: TlsCertificateFlags) {
        unsafe {
            ffi::g_socket_client_set_tls_validation_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    fn get_property_type(&self) -> SocketType {
        unsafe {
            let mut value = Value::from_type(<SocketType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_type(&self, type_: SocketType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "type".to_glib_none().0, Value::from(&type_).to_glib_none().0);
        }
    }

    fn connect_event<F: Fn(&Self, SocketClientEvent, &SocketConnectable, &Option<IOStream>) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, SocketClientEvent, &SocketConnectable, &Option<IOStream>) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "event",
                transmute(event_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_enable_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::enable-proxy",
                transmute(notify_enable_proxy_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::family",
                transmute(notify_family_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_local_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::local-address",
                transmute(notify_local_address_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::protocol",
                transmute(notify_protocol_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn connect_property_proxy_resolver_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::proxy-resolver",
                transmute(notify_proxy_resolver_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::timeout",
                transmute(notify_timeout_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tls_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tls",
                transmute(notify_tls_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tls_validation_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tls-validation-flags",
                transmute(notify_tls_validation_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::type",
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn event_trampoline<P>(this: *mut ffi::GSocketClient, event: ffi::GSocketClientEvent, connectable: *mut ffi::GSocketConnectable, connection: *mut ffi::GIOStream, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &&(Fn(&P, SocketClientEvent, &SocketConnectable, &Option<IOStream>) + 'static) = transmute(f);
    f(&SocketClient::from_glib_borrow(this).downcast_unchecked(), from_glib(event), &from_glib_borrow(connectable), &from_glib_borrow(connection))
}

unsafe extern "C" fn notify_enable_proxy_trampoline<P>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SocketClient::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_family_trampoline<P>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SocketClient::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_local_address_trampoline<P>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SocketClient::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_protocol_trampoline<P>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SocketClient::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_36", feature = "dox"))]
unsafe extern "C" fn notify_proxy_resolver_trampoline<P>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SocketClient::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_timeout_trampoline<P>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SocketClient::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tls_trampoline<P>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SocketClient::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tls_validation_flags_trampoline<P>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SocketClient::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SocketClient::from_glib_borrow(this).downcast_unchecked())
}
