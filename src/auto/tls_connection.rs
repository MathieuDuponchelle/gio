// This file was generated by gir (https://github.com/gtk-rs/gir @ c133b69)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Cancellable;
use Error;
use IOStream;
use TlsCertificate;
use TlsCertificateFlags;
use TlsDatabase;
use TlsInteraction;
use TlsRehandshakeMode;
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
    pub struct TlsConnection(Object<ffi::GTlsConnection, ffi::GTlsConnectionClass>): IOStream;

    match fn {
        get_type => || ffi::g_tls_connection_get_type(),
    }
}

pub trait TlsConnectionExt {
    fn emit_accept_certificate(&self, peer_cert: &TlsCertificate, errors: TlsCertificateFlags) -> bool;

    fn get_certificate(&self) -> Option<TlsCertificate>;

    fn get_database(&self) -> Option<TlsDatabase>;

    fn get_interaction(&self) -> Option<TlsInteraction>;

    fn get_peer_certificate(&self) -> Option<TlsCertificate>;

    fn get_peer_certificate_errors(&self) -> TlsCertificateFlags;

    fn get_rehandshake_mode(&self) -> TlsRehandshakeMode;

    fn get_require_close_notify(&self) -> bool;

    #[deprecated]
    fn get_use_system_certdb(&self) -> bool;

    fn handshake<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn handshake_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: i32, cancellable: P, callback: Q);

    fn set_certificate(&self, certificate: &TlsCertificate);

    fn set_database<P: IsA<TlsDatabase>>(&self, database: &P);

    fn set_interaction<'a, P: Into<Option<&'a TlsInteraction>>>(&self, interaction: P);

    fn set_rehandshake_mode(&self, mode: TlsRehandshakeMode);

    fn set_require_close_notify(&self, require_close_notify: bool);

    #[deprecated]
    fn set_use_system_certdb(&self, use_system_certdb: bool);

    fn get_property_base_io_stream(&self) -> Option<IOStream>;

    fn connect_accept_certificate<F: Fn(&Self, &TlsCertificate, TlsCertificateFlags) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_base_io_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_database_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_interaction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_peer_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_peer_certificate_errors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rehandshake_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_require_close_notify_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[deprecated]
    fn connect_property_use_system_certdb_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TlsConnection> + IsA<glib::object::Object>> TlsConnectionExt for O {
    fn emit_accept_certificate(&self, peer_cert: &TlsCertificate, errors: TlsCertificateFlags) -> bool {
        unsafe {
            from_glib(ffi::g_tls_connection_emit_accept_certificate(self.to_glib_none().0, peer_cert.to_glib_none().0, errors.to_glib()))
        }
    }

    fn get_certificate(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(ffi::g_tls_connection_get_certificate(self.to_glib_none().0))
        }
    }

    fn get_database(&self) -> Option<TlsDatabase> {
        unsafe {
            from_glib_none(ffi::g_tls_connection_get_database(self.to_glib_none().0))
        }
    }

    fn get_interaction(&self) -> Option<TlsInteraction> {
        unsafe {
            from_glib_none(ffi::g_tls_connection_get_interaction(self.to_glib_none().0))
        }
    }

    fn get_peer_certificate(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(ffi::g_tls_connection_get_peer_certificate(self.to_glib_none().0))
        }
    }

    fn get_peer_certificate_errors(&self) -> TlsCertificateFlags {
        unsafe {
            from_glib(ffi::g_tls_connection_get_peer_certificate_errors(self.to_glib_none().0))
        }
    }

    fn get_rehandshake_mode(&self) -> TlsRehandshakeMode {
        unsafe {
            from_glib(ffi::g_tls_connection_get_rehandshake_mode(self.to_glib_none().0))
        }
    }

    fn get_require_close_notify(&self) -> bool {
        unsafe {
            from_glib(ffi::g_tls_connection_get_require_close_notify(self.to_glib_none().0))
        }
    }

    fn get_use_system_certdb(&self) -> bool {
        unsafe {
            from_glib(ffi::g_tls_connection_get_use_system_certdb(self.to_glib_none().0))
        }
    }

    fn handshake<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_tls_connection_handshake(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn handshake_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn handshake_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let _ = ffi::g_tls_connection_handshake_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = handshake_async_trampoline::<Q>;
        unsafe {
            ffi::g_tls_connection_handshake_async(self.to_glib_none().0, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn set_certificate(&self, certificate: &TlsCertificate) {
        unsafe {
            ffi::g_tls_connection_set_certificate(self.to_glib_none().0, certificate.to_glib_none().0);
        }
    }

    fn set_database<P: IsA<TlsDatabase>>(&self, database: &P) {
        unsafe {
            ffi::g_tls_connection_set_database(self.to_glib_none().0, database.to_glib_none().0);
        }
    }

    fn set_interaction<'a, P: Into<Option<&'a TlsInteraction>>>(&self, interaction: P) {
        let interaction = interaction.into();
        let interaction = interaction.to_glib_none();
        unsafe {
            ffi::g_tls_connection_set_interaction(self.to_glib_none().0, interaction.0);
        }
    }

    fn set_rehandshake_mode(&self, mode: TlsRehandshakeMode) {
        unsafe {
            ffi::g_tls_connection_set_rehandshake_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn set_require_close_notify(&self, require_close_notify: bool) {
        unsafe {
            ffi::g_tls_connection_set_require_close_notify(self.to_glib_none().0, require_close_notify.to_glib());
        }
    }

    fn set_use_system_certdb(&self, use_system_certdb: bool) {
        unsafe {
            ffi::g_tls_connection_set_use_system_certdb(self.to_glib_none().0, use_system_certdb.to_glib());
        }
    }

    fn get_property_base_io_stream(&self) -> Option<IOStream> {
        unsafe {
            let mut value = Value::from_type(<IOStream as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "base-io-stream".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_accept_certificate<F: Fn(&Self, &TlsCertificate, TlsCertificateFlags) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TlsCertificate, TlsCertificateFlags) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "accept-certificate",
                transmute(accept_certificate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_base_io_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::base-io-stream",
                transmute(notify_base_io_stream_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::certificate",
                transmute(notify_certificate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_database_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::database",
                transmute(notify_database_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_interaction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::interaction",
                transmute(notify_interaction_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_peer_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::peer-certificate",
                transmute(notify_peer_certificate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_peer_certificate_errors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::peer-certificate-errors",
                transmute(notify_peer_certificate_errors_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_rehandshake_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::rehandshake-mode",
                transmute(notify_rehandshake_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_require_close_notify_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::require-close-notify",
                transmute(notify_require_close_notify_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_system_certdb_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-system-certdb",
                transmute(notify_use_system_certdb_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn accept_certificate_trampoline<P>(this: *mut ffi::GTlsConnection, peer_cert: *mut ffi::GTlsCertificate, errors: ffi::GTlsCertificateFlags, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<TlsConnection> {
    let f: &&(Fn(&P, &TlsCertificate, TlsCertificateFlags) -> bool + 'static) = transmute(f);
    f(&TlsConnection::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(peer_cert), from_glib(errors)).to_glib()
}

unsafe extern "C" fn notify_base_io_stream_trampoline<P>(this: *mut ffi::GTlsConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_certificate_trampoline<P>(this: *mut ffi::GTlsConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_database_trampoline<P>(this: *mut ffi::GTlsConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_interaction_trampoline<P>(this: *mut ffi::GTlsConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_peer_certificate_trampoline<P>(this: *mut ffi::GTlsConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_peer_certificate_errors_trampoline<P>(this: *mut ffi::GTlsConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_rehandshake_mode_trampoline<P>(this: *mut ffi::GTlsConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_require_close_notify_trampoline<P>(this: *mut ffi::GTlsConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_system_certdb_trampoline<P>(this: *mut ffi::GTlsConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsConnection::from_glib_borrow(this).downcast_unchecked())
}
